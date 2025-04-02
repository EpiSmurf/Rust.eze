/*
    EcoSim Main Module

    Ce module initialise la fenêtre de simulation (macroquad),
    gère le menu de configuration, la saisie utilisateur,
    la mise à jour de l’état de la simulation et l’affichage.
    Il permet aussi de suivre un agent et de naviguer dans
    l’historique des itérations.
*/

use macroquad::prelude::*;
use crate::config::{SimulationConfig, AgentType, Agent};
use crate::ecosystem::Ecosystem;

mod config;
mod ecosystem;

// Taille d’un agent multipliée par 1.25 : 10.0 -> 12.5
const VIOLET: Color = Color::new(0.5, 0.0, 0.5, 1.0);
const DARK_GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);
const ORANGE: Color = Color::new(1.0, 0.65, 0.0, 1.0);
const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
const BROWN: Color = Color::new(0.4, 0.2, 0.0, 1.0); // For trees

/// Configures the macroquad window.
fn window_conf() -> Conf {
    Conf {
        window_title: "EcoSim".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

/// Application state.
enum AppState {
    ConfigMenu,
    Simulation,
}

/// Représente un champ configurable dans le menu.
struct ConfigField {
    label: String,
    is_int: bool,
    input: String,
}

impl ConfigField {
    fn display_value(&self) -> String {
        self.input.clone()
    }
}

/// Structure pour suivre un agent particulier.
#[derive(Clone)]
struct TrackingInfo {
    pub agent_id: u32,
    pub born_x: usize,
    pub born_y: usize,
    pub x: usize,
    pub y: usize,
    pub energy: i32,
    pub died: Option<String>,
}

impl TrackingInfo {
    fn new(agent: &Agent) -> Self {
        TrackingInfo {
            agent_id: agent.id,
            born_x: agent.x,
            born_y: agent.y,
            x: agent.x,
            y: agent.y,
            energy: agent.energy,
            died: None,
        }
    }

    fn overwrite(&mut self, agent: &Agent) {
        self.x = agent.x;
        self.y = agent.y;
        self.energy = agent.energy;
        if agent.pending_death {
            self.died = agent.death_cause.clone();
        } else {
            self.died = None;
        }
    }

    fn mark_death(&mut self, cause: &str) {
        self.died = Some(cause.to_string());
    }
}

fn advance_simulation(eco: &mut Ecosystem, track: &mut Option<TrackingInfo>) {
    eco.step();
    if let Some(t) = track {
        if let Some(agent) = eco.herbivores.iter().find(|h| h.id == t.agent_id)
            .or_else(|| eco.carnivores.iter().find(|c| c.id == t.agent_id))
            .or_else(|| eco.omnivores.iter().find(|o| o.id == t.agent_id))
        {
            t.overwrite(agent);
        } else if t.died.is_none() {
            t.mark_death("Unknown (likely removed)");
        }
    }
}

fn load_from_history(
    history: &Vec<Ecosystem>,
    idx: usize,
    ecosystem: &mut Option<Ecosystem>,
    track: &mut Option<TrackingInfo>,
    simulation_step: &mut usize,
) {
    *ecosystem = Some(history[idx].clone());
    *simulation_step = idx;
    if let Some(t) = track {
        if let Some(ref eco) = ecosystem {
            if let Some(agent) = eco.herbivores.iter().find(|h| h.id == t.agent_id)
                .or_else(|| eco.carnivores.iter().find(|c| c.id == t.agent_id))
                .or_else(|| eco.omnivores.iter().find(|o| o.id == t.agent_id))
            {
                t.overwrite(agent);
            } else if t.died.is_none() {
                t.mark_death("Unknown (likely removed)");
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::ConfigMenu;

    // Grid: width = 114, height = 52
    let cell_size: f32 = 12.5;
    let grid_width: usize = 114;
    let grid_height: usize = 52;
    let offset_x: f32 = 50.0;
    let offset_y: f32 = 50.0;

    // Paramètres : on ne paramètre plus water_lifespan et tree_lifespan.
    let mut fields = vec![
        ConfigField { label: "Initial Light Plants".to_string(), is_int: true, input: "100".to_string() },
        ConfigField { label: "Initial Dark Plants".to_string(), is_int: true, input: "50".to_string() },
        ConfigField { label: "Initial Herbivores".to_string(), is_int: true, input: "300".to_string() },
        ConfigField { label: "Initial Carnivores".to_string(), is_int: true, input: "100".to_string() },
        ConfigField { label: "Initial Omnivores".to_string(), is_int: true, input: "50".to_string() },
        ConfigField { label: "Water Spawn Chance".to_string(), is_int: false, input: "0.01".to_string() },
        // On définit la valeur par défaut à 0.02
        ConfigField { label: "Tree Spawn Chance".to_string(), is_int: false, input: "0.02".to_string() },
    ];
    let mut selected_field: usize = 0;
    let mut ecosystem: Option<Ecosystem> = None;
    let mut history: Vec<Ecosystem> = Vec::new();
    let mut current_index: usize = 0;
    let mut simulation_step: usize = 0;
    let mut tracking: Option<TrackingInfo> = None;

    loop {
        clear_background(BLACK);
        match app_state {
            AppState::ConfigMenu => {
                let start_x = offset_x;
                let mut y = offset_y;
                for (i, field) in fields.iter().enumerate() {
                    let color = if i == selected_field { YELLOW } else { WHITE };
                    draw_text(&format!("{}: {}", field.label, field.display_value()), start_x, y, 20.0, color);
                    y += 30.0;
                }
                y += 20.0;
                draw_text("Up/Down: Changer de champ", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Tapez chiffres ou '.' (si non-entier).", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Backspace: effacer | Enter: valider et lancer la simu", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Esc: Quitter", start_x, y, 20.0, WHITE);

                // Gestion du clavier pour changer la sélection
                if is_key_pressed(KeyCode::Up) && selected_field > 0 {
                    selected_field -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_field < fields.len() - 1 {
                    selected_field += 1;
                }

                // Gestion de la saisie pour le champ sélectionné
                {
                    let field = &mut fields[selected_field];
                    if let Some(ch) = get_char_pressed() {
                        if ch.is_ascii_digit() || (ch == '.' && !field.is_int && !field.input.contains('.')) {
                            field.input.push(ch);
                        }
                    }
                    if is_key_pressed(KeyCode::Backspace) {
                        field.input.pop();
                    }
                }

                // Touche Enter => Construction de la config + lancement
                if is_key_pressed(KeyCode::Enter) {
                    let default_config = SimulationConfig::default();
                    let config = SimulationConfig {
                        grid_width,
                        grid_height,
                        initial_plants: fields[0].input.parse::<usize>().unwrap_or(default_config.initial_plants),
                        initial_dark_plants: fields[1].input.parse::<usize>().unwrap_or(default_config.initial_dark_plants),
                        initial_herbivores: fields[2].input.parse::<usize>().unwrap_or(default_config.initial_herbivores),
                        initial_carnivores: fields[3].input.parse::<usize>().unwrap_or(default_config.initial_carnivores),
                        initial_omnivores: fields[4].input.parse::<usize>().unwrap_or(default_config.initial_omnivores),

                        water_spawn_chance: fields[5].input.parse::<f32>().unwrap_or(default_config.water_spawn_chance),
                        water_lifespan: default_config.water_lifespan,
                        tree_spawn_chance: fields[6].input.parse::<f32>().unwrap_or(default_config.tree_spawn_chance),
                        tree_lifespan: default_config.tree_lifespan,

                        ..default_config
                    };
                    ecosystem = Some(Ecosystem::new_custom(config));
                    history.push(ecosystem.as_ref().unwrap().clone());
                    current_index = 0;
                    simulation_step = 0;
                    app_state = AppState::Simulation;
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
            AppState::Simulation => {
                // Clic gauche => tenter de sélectionner un agent à traquer
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(ref eco) = ecosystem {
                        let (mx, my) = mouse_position();
                        if mx >= offset_x && my >= offset_y && mx < offset_x + (grid_width as f32 * cell_size)
                            && my < offset_y + (grid_height as f32 * cell_size)
                        {
                            let cx = ((mx - offset_x) / cell_size).floor() as usize;
                            let cy = ((my - offset_y) / cell_size).floor() as usize;
                            if eco.herbivores.iter().any(|a| a.x == cx && a.y == cy)
                                || eco.carnivores.iter().any(|a| a.x == cx && a.y == cy)
                                || eco.omnivores.iter().any(|a| a.x == cx && a.y == cy)
                            {
                                if let Some(a) = eco.herbivores.iter().find(|a| a.x == cx && a.y == cy)
                                    .or_else(|| eco.carnivores.iter().find(|a| a.x == cx && a.y == cy))
                                    .or_else(|| eco.omnivores.iter().find(|a| a.x == cx && a.y == cy))
                                {
                                    tracking = Some(TrackingInfo::new(a));
                                }
                            } else {
                                tracking = None;
                            }
                        } else {
                            tracking = None;
                        }
                    }
                }

                // Navigation dans l’historique (flèches gauche/droite)
                if is_key_pressed(KeyCode::Left) {
                    if current_index > 0 {
                        current_index -= 1;
                        load_from_history(&history, current_index, &mut ecosystem, &mut tracking, &mut simulation_step);
                    }
                }
                if is_key_pressed(KeyCode::Right) {
                    if current_index < history.len() - 1 {
                        current_index += 1;
                        load_from_history(&history, current_index, &mut ecosystem, &mut tracking, &mut simulation_step);
                    } else if let Some(ref mut eco) = ecosystem {
                        advance_simulation(eco, &mut tracking);
                        history.push(eco.clone());
                        current_index += 1;
                        simulation_step = current_index;
                    }
                }

                // Space => avance la sim en continu (tant que c’est maintenu)
                if is_key_down(KeyCode::Space) {
                    if let Some(ref mut eco) = ecosystem {
                        advance_simulation(eco, &mut tracking);
                        history.push(eco.clone());
                        current_index += 1;
                        simulation_step = current_index;
                    }
                }

                // Affichage de la grille
                if let Some(ref eco) = ecosystem {
                    for y in 0..eco.height {
                        for x in 0..eco.width {
                            let mut color = LIGHTGRAY;
                            // Les arbres en priorité
                            if eco.trees.iter().any(|t| t.x == x && t.y == y) {
                                color = BROWN;
                            } else if eco.waters.iter().any(|w| w.x == x && w.y == y) {
                                color = BLUE;
                            } else {
                                // Si un agent est tracké, on le colorie en violet
                                if let Some(t) = &tracking {
                                    let is_tracked = eco.herbivores.iter().any(|a| a.id == t.agent_id && a.x == x && a.y == y)
                                        || eco.carnivores.iter().any(|a| a.id == t.agent_id && a.x == x && a.y == y)
                                        || eco.omnivores.iter().any(|a| a.id == t.agent_id && a.x == x && a.y == y);
                                    if is_tracked {
                                        color = VIOLET;
                                    } else if eco.carnivores.iter().any(|c| c.x == x && c.y == y) {
                                        color = RED;
                                    } else if eco.herbivores.iter().any(|h| h.x == x && h.y == y) {
                                        color = PINK;
                                    } else if eco.omnivores.iter().any(|o| o.x == x && o.y == y) {
                                        color = ORANGE;
                                    } else if eco.plants.iter().any(|p| p.x == x && p.y == y) {
                                        if eco.plants.iter().any(|p| p.x == x && p.y == y && p.agent_type == AgentType::DarkPlant) {
                                            color = DARK_GREEN;
                                        } else {
                                            color = GREEN;
                                        }
                                    }
                                } else {
                                    // Sinon colorations classiques
                                    if eco.carnivores.iter().any(|c| c.x == x && c.y == y) {
                                        color = RED;
                                    } else if eco.herbivores.iter().any(|h| h.x == x && h.y == y) {
                                        color = PINK;
                                    } else if eco.omnivores.iter().any(|o| o.x == x && o.y == y) {
                                        color = ORANGE;
                                    } else if eco.plants.iter().any(|p| p.x == x && p.y == y) {
                                        if eco.plants.iter().any(|p| p.x == x && p.y == y && p.agent_type == AgentType::DarkPlant) {
                                            color = DARK_GREEN;
                                        } else {
                                            color = GREEN;
                                        }
                                    }
                                }
                            }
                            draw_rectangle(offset_x + x as f32 * cell_size,
                                           offset_y + y as f32 * cell_size,
                                           cell_size - 1.0, cell_size - 1.0, color);
                        }
                    }
                }

                // Affichage de stats
                let avg_plants: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.plants.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };
                let avg_herbivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.herbivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };
                let avg_carnivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.carnivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };
                let avg_omnivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.omnivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };

                let base_x = offset_x;
                let base_y = offset_y + (grid_height as f32 * cell_size) + 30.0;
                draw_text("Left/Right: Step Backward/Forward", base_x, base_y, 20.0, WHITE);
                draw_text("Hold Space: Continuous Update", base_x, base_y + 30.0, 20.0, WHITE);
                draw_text("Left Click On An Agent To Track It", base_x, base_y + 60.0, 20.0, WHITE);
                draw_text("Esc: Quit", base_x, base_y + 90.0, 20.0, WHITE);

                let stats_x = base_x + 550.0;
                let stats_y = base_y;
                draw_text(&format!("Step: {}", simulation_step), stats_x, stats_y, 20.0, YELLOW);
                draw_text(&format!("Plants: {} (Avg: {:.1})",
                    if let Some(ref eco) = ecosystem { eco.plants.len() } else { 0 },
                    avg_plants
                ), stats_x, stats_y + 30.0, 20.0, GREEN);
                draw_text(&format!("Herbivores: {} (Avg: {:.1})",
                    if let Some(ref eco) = ecosystem { eco.herbivores.len() } else { 0 },
                    avg_herbivores
                ), stats_x, stats_y + 60.0, 20.0, PINK);
                draw_text(&format!("Carnivores: {} (Avg: {:.1})",
                    if let Some(ref eco) = ecosystem { eco.carnivores.len() } else { 0 },
                    avg_carnivores
                ), stats_x, stats_y + 90.0, 20.0, RED);
                draw_text(&format!("Omnivores: {} (Avg: {:.1})",
                    if let Some(ref eco) = ecosystem { eco.omnivores.len() } else { 0 },
                    avg_omnivores
                ), stats_x, stats_y + 120.0, 20.0, ORANGE);

                let track_x = stats_x + 500.0;
                let track_y = base_y;
                if let Some(t) = &tracking {
                    draw_text("Tracked Agent Info:", track_x, track_y, 20.0, VIOLET);
                    draw_text(&format!("Born: ({}, {})", t.born_x, t.born_y),
                              track_x, track_y + 30.0, 20.0, WHITE);
                    draw_text(&format!("Position: ({}, {})", t.x, t.y),
                              track_x, track_y + 60.0, 20.0, WHITE);
                    draw_text(&format!("Energy: {}", t.energy),
                              track_x, track_y + 90.0, 20.0, WHITE);
                    let died_text = t.died.as_deref().unwrap_or("Not Yet");
                    draw_text(&format!("Died: {}", died_text),
                              track_x, track_y + 120.0, 20.0, WHITE);
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await;
    }
}
