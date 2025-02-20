use macroquad::prelude::*;
use crate::config::SimulationConfig;
use crate::ecosystem::Ecosystem;
use crate::species::Agent;

mod config;
mod ecosystem;
mod species;

const VIOLET: Color = Color::new(0.5, 0.0, 0.5, 1.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "EcoSim".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

enum AppState {
    ConfigMenu,
    Simulation,
}

struct ConfigField {
    label: String,
    value: f32,
    is_int: bool,
    step: f32,
}

impl ConfigField {
    fn display_value(&self) -> String {
        if self.is_int {
            format!("{}", self.value as i32)
        } else {
            format!("{:.1}", self.value)
        }
    }
}

/// TrackingInfo uniquement pour herbivores.
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
        self.died = None;
    }
    
    fn mark_death(&mut self, cause: &str) {
        self.died = Some(cause.to_string());
    }
}

/// Avance la simulation d'une itération et met à jour le tracking.
fn advance_simulation(eco: &mut Ecosystem, track: &mut Option<TrackingInfo>) {
    eco.step();
    if let Some(t) = track {
        if let Some(a) = eco.herbivores.iter().find(|h| h.id == t.agent_id) {
            t.overwrite(a);
        } else if t.died.is_none() {
            t.mark_death("Energy Depletion");
        }
    }
}

/// Charge un état depuis l'historique et ajuste le tracking.
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
            if let Some(a) = eco.herbivores.iter().find(|h| h.id == t.agent_id) {
                t.overwrite(a);
            } else if t.died.is_none() {
                t.mark_death("Energy Depletion");
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::ConfigMenu;
    let cell_size: f32 = 10.0;
    // Grille 140x65
    let grid_width: usize = 140;
    let grid_height: usize = 65;
    // Offsets
    let offset_x: f32 = 50.0;
    let offset_y: f32 = 50.0;

    // Champs de configuration
    let mut fields = vec![
        ConfigField { label: "Initial Plants".to_string(), value: 200.0, is_int: true, step: 1.0 },
        ConfigField { label: "Initial Herbivores".to_string(), value: 80.0, is_int: true, step: 1.0 },
        ConfigField { label: "Plant Growth Rate".to_string(), value: 0.2, is_int: false, step: 0.1 },
        ConfigField { label: "Herbivore Reproduction Rate".to_string(), value: 0.1, is_int: false, step: 0.1 },
        ConfigField { label: "Herbivore Energy Gain".to_string(), value: 5.0, is_int: true, step: 1.0 },
        ConfigField { label: "Herbivore Energy Loss".to_string(), value: 1.0, is_int: true, step: 1.0 },
        ConfigField { label: "Herbivore Initial Energy".to_string(), value: 10.0, is_int: true, step: 1.0 },
        ConfigField { label: "Herbivore Reproduction Threshold".to_string(), value: 15.0, is_int: true, step: 1.0 },
    ];
    let mut selected_field: usize = 0;

    let mut ecosystem: Option<Ecosystem> = None;
    let mut history: Vec<Ecosystem> = Vec::new();
    let mut current_index: usize = 0;
    let mut simulation_step: usize = 0;

    // Tracking uniquement pour herbivores.
    let mut tracking: Option<TrackingInfo> = None;

    loop {
        clear_background(BLACK);

        match app_state {
            AppState::ConfigMenu => {
                let start_x = offset_x;
                let mut y = offset_y;
                for (i, field) in fields.iter().enumerate() {
                    let color = if i == selected_field { YELLOW } else { WHITE };
                    draw_text(
                        &format!("{}: {}", field.label, field.display_value()),
                        start_x,
                        y,
                        20.0,
                        color,
                    );
                    y += 30.0;
                }
                y += 20.0;
                draw_text("Up/Down: Select", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Left/Right: Change", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Enter: Start", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Esc: Quit", start_x, y, 20.0, WHITE);

                if is_key_pressed(KeyCode::Up) {
                    if selected_field > 0 { selected_field -= 1; }
                }
                if is_key_pressed(KeyCode::Down) {
                    if selected_field < fields.len() - 1 { selected_field += 1; }
                }
                if is_key_pressed(KeyCode::Left) {
                    let f = &mut fields[selected_field];
                    f.value = (f.value - f.step).max(0.0);
                }
                if is_key_pressed(KeyCode::Right) {
                    let f = &mut fields[selected_field];
                    f.value += f.step;
                }
                if is_key_pressed(KeyCode::Enter) {
                    let config = SimulationConfig {
                        grid_width,
                        grid_height,
                        initial_plants: fields[0].value as usize,
                        initial_herbivores: fields[1].value as usize,
                        plant_growth_rate: fields[2].value,
                        herbivore_reproduction_rate: fields[3].value,
                        herbivore_energy_gain: fields[4].value as i32,
                        herbivore_energy_loss: fields[5].value as i32,
                        herbivore_initial_energy: fields[6].value as i32,
                        herbivore_reproduction_threshold: fields[7].value as i32,
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
                // Clic pour tracker un herbivore uniquement.
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(ref eco) = ecosystem {
                        let (mx, my) = mouse_position();
                        if mx >= offset_x && my >= offset_y &&
                           mx < offset_x + (grid_width as f32 * cell_size) &&
                           my < offset_y + (grid_height as f32 * cell_size)
                        {
                            let cx = ((mx - offset_x) / cell_size).floor() as usize;
                            let cy = ((my - offset_y) / cell_size).floor() as usize;
                            if eco.herbivores.iter().find(|a| a.x == cx && a.y == cy).is_none() {
                                tracking = None;
                            } else {
                                if let Some(a) = eco.herbivores.iter().find(|a| a.x == cx && a.y == cy) {
                                    tracking = Some(TrackingInfo::new(a));
                                }
                            }
                        } else {
                            tracking = None;
                        }
                    }
                }

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
                if is_key_down(KeyCode::Space) {
                    if let Some(ref mut eco) = ecosystem {
                        advance_simulation(eco, &mut tracking);
                        history.push(eco.clone());
                        current_index += 1;
                        simulation_step = current_index;
                    }
                }

                // Dessin de la grille
                if let Some(ref eco) = ecosystem {
                    for y in 0..eco.height {
                        for x in 0..eco.width {
                            let mut color = LIGHTGRAY;
                            if let Some(t) = &tracking {
                                let is_tracked = eco.herbivores.iter().any(|a| a.id == t.agent_id && a.x == x && a.y == y);
                                if is_tracked {
                                    color = VIOLET;
                                } else if eco.herbivores.iter().any(|h| h.x == x && h.y == y) {
                                    color = RED;
                                } else if eco.plants.iter().any(|p| p.x == x && p.y == y) {
                                    color = GREEN;
                                }
                            } else {
                                if eco.herbivores.iter().any(|h| h.x == x && h.y == y) {
                                    color = RED;
                                } else if eco.plants.iter().any(|p| p.x == x && p.y == y) {
                                    color = GREEN;
                                }
                            }
                            draw_rectangle(
                                offset_x + x as f32 * cell_size,
                                offset_y + y as f32 * cell_size,
                                cell_size - 1.0,
                                cell_size - 1.0,
                                color,
                            );
                        }
                    }
                }

                // Calcul des moyennes
                let avg_plants: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.plants.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };
                let avg_herbivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.herbivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else { 0.0 };

                // Position de base pour l'UI en bas
                let base_x = offset_x;
                let base_y = offset_y + (grid_height as f32 * cell_size) + 30.0;

                // 1) Commandes (à gauche)
                draw_text("Left/Right: Step Backward/Forward", base_x, base_y, 20.0, WHITE);
                draw_text("Hold Space: Continuous Update", base_x, base_y + 30.0, 20.0, WHITE);
                draw_text("Left Click On A Herbivore To Track It", base_x, base_y + 60.0, 20.0, WHITE);
                draw_text("Esc: Quit", base_x, base_y + 90.0, 20.0, WHITE);

                // 2) Statistiques (au centre)
                let stats_x = base_x + 550.0; // Espace pour séparer les commandes
                let stats_y = base_y;
                draw_text(&format!("Step: {}", simulation_step), stats_x, stats_y, 20.0, YELLOW);
                draw_text(&format!("Plants: {} (Avg: {:.1})", 
                    if let Some(ref eco) = ecosystem { eco.plants.len() } else { 0 }, 
                    avg_plants
                ), 
                stats_x, stats_y + 30.0, 20.0, GREEN);
                draw_text(&format!("Herbivores: {} (Avg: {:.1})", 
                    if let Some(ref eco) = ecosystem { eco.herbivores.len() } else { 0 }, 
                    avg_herbivores
                ), 
                stats_x, stats_y + 60.0, 20.0, RED);

                // 3) Tracking (à droite des stats)
                let track_x = stats_x + 500.0; // Espace supplémentaire
                let track_y = base_y;
                if let Some(t) = &tracking {
                    draw_text("Tracked Herbivore Info:", track_x, track_y, 20.0, YELLOW);
                    draw_text(&format!("Born: ({}, {})", t.born_x, t.born_y), track_x, track_y + 30.0, 20.0, WHITE);
                    draw_text(&format!("Position: ({}, {})", t.x, t.y), track_x, track_y + 60.0, 20.0, WHITE);
                    draw_text(&format!("Energy: {}", t.energy), track_x, track_y + 90.0, 20.0, WHITE);
                    let died_text = t.died.as_deref().unwrap_or("Not Yet");
                    draw_text(&format!("Died: {}", died_text), track_x, track_y + 120.0, 20.0, WHITE);
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }

        next_frame().await;
    }
}
