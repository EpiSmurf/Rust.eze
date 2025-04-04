use macroquad::prelude::*;
use crate::config::{SimulationConfig, AgentType, Agent};
use crate::ecosystem::{Ecosystem, SimulationStats};

mod config;
mod ecosystem;

const VIOLET: Color = Color::new(0.5, 0.0, 0.5, 1.0);
const DARK_GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);
const ORANGE: Color = Color::new(1.0, 0.65, 0.0, 1.0);
const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
const BROWN: Color = Color::new(0.4, 0.2, 0.0, 1.0);

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust.eze".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        ..Default::default()
    }
}

enum AppState {
    ConfigMenu,
    Simulation,
    StatsScreen,
}

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

fn advance_simulation(eco: &mut Ecosystem, track: &mut Option<TrackingInfo>, stats: &mut SimulationStats) {
    eco.step(stats);
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
    let cell_size: f32 = 12.5;
    let grid_width: usize = 114;
    let grid_height: usize = 52;
    let offset_x: f32 = 50.0;
    let offset_y: f32 = 50.0;
    let default_config = SimulationConfig::default();
    let mut fields = vec![
        ConfigField {
            label: "Initial Light Plants".to_string(),
            is_int: true,
            input: default_config.initial_light_plants.to_string(),
        },
        ConfigField {
            label: "Initial Dark Plants".to_string(),
            is_int: true,
            input: default_config.initial_dark_plants.to_string(),
        },
        ConfigField {
            label: "Initial Herbivores".to_string(),
            is_int: true,
            input: default_config.initial_herbivores.to_string(),
        },
        ConfigField {
            label: "Initial Carnivores".to_string(),
            is_int: true,
            input: default_config.initial_carnivores.to_string(),
        },
        ConfigField {
            label: "Initial Omnivores".to_string(),
            is_int: true,
            input: default_config.initial_omnivores.to_string(),
        },
        ConfigField {
            label: "Water Spawn Chance".to_string(),
            is_int: false,
            input: default_config.water_spawn_chance.to_string(),
        },
        ConfigField {
            label: "Tree Spawn Chance".to_string(),
            is_int: false,
            input: default_config.tree_spawn_chance.to_string(),
        },
    ];
    let mut selected_field: usize = 0;
    let mut ecosystem: Option<Ecosystem> = None;
    let mut history: Vec<Ecosystem> = Vec::new();
    let mut current_index: usize = 0;
    let mut simulation_step: usize = 0;
    let mut tracking: Option<TrackingInfo> = None;
    let mut stats: SimulationStats = SimulationStats::default();
    loop {
        clear_background(BLACK);
        match app_state {
            AppState::ConfigMenu => {
                let start_x = offset_x;
                let mut y = offset_y;
                y += 20.0;
                draw_text("Rust.eze", start_x, y, 50.0, VIOLET);
                y += 60.0;
                draw_text("Contributors:", start_x, y, 30.0, YELLOW);
                y += 40.0;
                draw_text("Guillaume DUFOUR", start_x, y, 20.0, WHITE);
                y += 25.0;
                draw_text("Lucas BONDARENKO", start_x, y, 20.0, WHITE);
                y += 25.0;
                draw_text("Philippe RASTOUL", start_x, y, 20.0, WHITE);
                y += 25.0;
                draw_text("Amir-Alexandre BARKALLAH", start_x, y, 20.0, WHITE);
                y += 60.0;
                for (i, field) in fields.iter().enumerate() {
                    let font_size = if i == selected_field { 22.5 } else { 20.0 };
                    let color = if i == selected_field {
                        WHITE
                    } else {
                        match field.label.as_str() {
                            "Initial Carnivores" => RED,
                            "Initial Herbivores" => PINK,
                            "Initial Omnivores" => ORANGE,
                            "Initial Dark Plants" => DARK_GREEN,
                            "Initial Light Plants" => GREEN,
                            "Water Spawn Chance" => BLUE,
                            "Tree Spawn Chance" => BROWN,
                            _ => WHITE,
                        }
                    };

                    draw_text(
                        &format!("{}: {}", field.label, field.display_value()),
                        start_x,
                        y,
                        font_size,
                        color,
                    );
                    y += 30.0;
                }
                y += 30.0;
                draw_text("Up/Down: Switch Field", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Type Digits or '.' to Change Values", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Backspace: Delete", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Enter: Start Simulation", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Esc: Quit", start_x, y, 20.0, WHITE);
                if is_key_pressed(KeyCode::Up) && selected_field > 0 {
                    selected_field -= 1;
                }
                if is_key_pressed(KeyCode::Down) && selected_field < fields.len() - 1 {
                    selected_field += 1;
                }
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
                if is_key_pressed(KeyCode::Enter) {
                    let config = SimulationConfig {
                        grid_width,
                        grid_height,
                        initial_light_plants: fields[0].input.parse::<usize>().unwrap_or(default_config.initial_light_plants),
                        initial_dark_plants: fields[1].input.parse::<usize>().unwrap_or(default_config.initial_dark_plants),
                        initial_herbivores: fields[2].input.parse::<usize>().unwrap_or(default_config.initial_herbivores),
                        initial_carnivores: fields[3].input.parse::<usize>().unwrap_or(default_config.initial_carnivores),
                        initial_omnivores: fields[4].input.parse::<usize>().unwrap_or(default_config.initial_omnivores),
                        water_spawn_chance: fields[5].input.parse::<f32>().unwrap_or(default_config.water_spawn_chance),
                        water_lifespan: default_config.water_lifespan,
                        tree_spawn_chance: fields[6].input.parse::<f32>().unwrap_or(default_config.tree_spawn_chance),
                        tree_lifespan: default_config.tree_lifespan,
                        plant_growth_rate: default_config.plant_growth_rate,
                        herbivore_reproduction_rate: default_config.herbivore_reproduction_rate,
                        herbivore_energy_gain: default_config.herbivore_energy_gain,
                        herbivore_energy_loss: default_config.herbivore_energy_loss,
                        herbivore_initial_energy: default_config.herbivore_initial_energy,
                        herbivore_reproduction_threshold: default_config.herbivore_reproduction_threshold,
                        carnivore_reproduction_rate: default_config.carnivore_reproduction_rate,
                        carnivore_energy_gain: default_config.carnivore_energy_gain,
                        carnivore_energy_loss: default_config.carnivore_energy_loss,
                        carnivore_initial_energy: default_config.carnivore_initial_energy,
                        carnivore_reproduction_threshold: default_config.carnivore_reproduction_threshold,
                        omnivore_reproduction_rate: default_config.omnivore_reproduction_rate,
                        omnivore_energy_gain_plants: default_config.omnivore_energy_gain_plants,
                        omnivore_energy_gain_herbivores: default_config.omnivore_energy_gain_herbivores,
                        omnivore_energy_loss: default_config.omnivore_energy_loss,
                        omnivore_initial_energy: default_config.omnivore_initial_energy,
                        omnivore_reproduction_threshold: default_config.omnivore_reproduction_threshold,
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
                        advance_simulation(eco, &mut tracking, &mut stats);
                        history.push(eco.clone());
                        current_index += 1;
                        simulation_step = current_index;
                    }
                }
                if is_key_down(KeyCode::Space) {
                    if let Some(ref mut eco) = ecosystem {
                        advance_simulation(eco, &mut tracking, &mut stats);
                        history.push(eco.clone());
                        current_index += 1;
                        simulation_step = current_index;
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    app_state = AppState::StatsScreen;
                }
                if let Some(ref eco) = ecosystem {
                    for y in 0..eco.height {
                        for x in 0..eco.width {
                            let mut color = LIGHTGRAY;
                            if eco.trees.iter().any(|t| t.x == x && t.y == y) {
                                color = BROWN;
                            } else if eco.waters.iter().any(|w| w.x == x && w.y == y) {
                                color = BLUE;
                            } else {
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
                            draw_rectangle(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size - 1.0, cell_size - 1.0, color);
                        }
                    }
                }
                let avg_plants: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.plants.len()).sum::<usize>() as f32 / history.len() as f32
                } else {
                    0.0
                };
                let avg_herbivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.herbivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else {
                    0.0
                };
                let avg_carnivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.carnivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else {
                    0.0
                };
                let avg_omnivores: f32 = if !history.is_empty() {
                    history.iter().map(|eco| eco.omnivores.len()).sum::<usize>() as f32 / history.len() as f32
                } else {
                    0.0
                };
                let base_x = offset_x;
                let base_y = offset_y + (grid_height as f32 * cell_size) + 30.0;
                draw_text("Left/Right: Step Backward/Forward", base_x, base_y, 20.0, WHITE);
                draw_text("Hold Space: Continuous Update", base_x, base_y + 30.0, 20.0, WHITE);
                draw_text("Left Click on an Animal to Track it", base_x, base_y + 60.0, 20.0, WHITE);
                draw_text("Esc: Show Statistics", base_x, base_y + 90.0, 20.0, WHITE);
                let stats_x = base_x + 550.0;
                let stats_y = base_y;
                draw_text(&format!("Step: {}", simulation_step), stats_x, stats_y, 20.0, YELLOW);
                draw_text(&format!("Plants: {} (Avg: {:.1})", if let Some(ref eco) = ecosystem { eco.plants.len() } else { 0 }, avg_plants), stats_x, stats_y + 30.0, 20.0, GREEN);
                draw_text(&format!("Herbivores: {} (Avg: {:.1})", if let Some(ref eco) = ecosystem { eco.herbivores.len() } else { 0 }, avg_herbivores), stats_x, stats_y + 60.0, 20.0, PINK);
                draw_text(&format!("Carnivores: {} (Avg: {:.1})", if let Some(ref eco) = ecosystem { eco.carnivores.len() } else { 0 }, avg_carnivores), stats_x, stats_y + 90.0, 20.0, RED);
                draw_text(&format!("Omnivores: {} (Avg: {:.1})", if let Some(ref eco) = ecosystem { eco.omnivores.len() } else { 0 }, avg_omnivores), stats_x, stats_y + 120.0, 20.0, ORANGE);
                let track_x = stats_x + 500.0;
                let track_y = base_y;
                if let Some(t) = &tracking {
                    draw_text("Tracked Animal Info:", track_x, track_y, 20.0, VIOLET);
                    draw_text(&format!("Born: ({}, {})", t.born_x, t.born_y), track_x, track_y + 30.0, 20.0, WHITE);
                    draw_text(&format!("Position: ({}, {})", t.x, t.y), track_x, track_y + 60.0, 20.0, WHITE);
                    draw_text(&format!("Energy: {}", t.energy), track_x, track_y + 90.0, 20.0, WHITE);
                    let died_text = t.died.as_deref().unwrap_or("Not Yet");
                    draw_text(&format!("Died: {}", died_text), track_x, track_y + 120.0, 20.0, WHITE);
                }
            }
            AppState::StatsScreen => {
                draw_text("Simulation Statistics", offset_x, offset_y + 15.0, 30.0, WHITE);
                let mut line = 1;
                let line_height = 15.0;
                line += 4;
                draw_text(&format!("Step Count: {}", simulation_step), offset_x, offset_y + line_height * (line as f32), 25.0, YELLOW);
                line += 3;
                draw_text("Plants", offset_x, offset_y + line_height * (line as f32), 20.0, GREEN);
                line += 1;
                draw_text(&format!("Births: {}   Deaths: {}", stats.plant_births, stats.plant_deaths), offset_x, offset_y + line_height * (line as f32), 20.0, GREEN);
                line += 2;
                draw_text("Herbivores", offset_x, offset_y + line_height * (line as f32), 20.0, PINK);
                line += 1;
                draw_text(&format!("Births: {}   Deaths: {}   Consumptions: {}", stats.herbivore_births, stats.herbivore_deaths, stats.herbivore_consumptions), offset_x, offset_y + line_height * (line as f32), 20.0, PINK);
                line += 2;
                draw_text("Carnivores", offset_x, offset_y + line_height * (line as f32), 20.0, RED);
                line += 1;
                draw_text(&format!("Births: {}   Deaths: {}   Consumptions: {}", stats.carnivore_births, stats.carnivore_deaths, stats.carnivore_consumptions), offset_x, offset_y + line_height * (line as f32), 20.0, RED);
                line += 2;
                draw_text("Omnivores", offset_x, offset_y + line_height * (line as f32), 20.0, ORANGE);
                line += 1;
                draw_text(&format!("Births: {}   Deaths: {}   Consumptions (Plants): {}   Consumptions (Herbivores): {}", stats.omnivore_births, stats.omnivore_deaths, stats.omnivore_consumptions_plants, stats.omnivore_consumptions_herbivores), offset_x, offset_y + line_height * (line as f32), 20.0, ORANGE);
                line += 2;
                draw_text("Lakes", offset_x, offset_y + line_height * (line as f32), 20.0, BLUE);
                line += 1;
                draw_text(&format!("Appearances: {}   Disappearances: {}", stats.water_births, stats.water_deaths), offset_x, offset_y + line_height * (line as f32), 20.0, BLUE);
                line += 2;
                draw_text("Trees", offset_x, offset_y + line_height * (line as f32), 20.0, BROWN);
                line += 1;
                draw_text(&format!("Appearances: {}   Disappearances: {}", stats.tree_births, stats.tree_deaths), offset_x, offset_y + line_height * (line as f32), 20.0, BROWN);
                line += 4;
                draw_text("Press Esc Again to Quit", offset_x, offset_y + line_height * (line as f32), 20.0, WHITE);
                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await;
    }
}
