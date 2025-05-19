use macroquad::prelude::*;

use crate::config::{SimulationConfig, AgentType};
use crate::ecosystem::{Ecosystem, SimulationStats};

mod config;
mod ecosystem;

const DARK_GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);

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
    SimulationSelector,
    ConfigMenu,
    Simulation,
    StatsScreen,
}

struct ConfigField {
    label: String,
    is_int: bool,
    input: String,
    color: Color,
}

impl ConfigField {
    fn display_value(&self) -> String {
        self.input.clone()
    }
}

struct SimulationInstance {
    ecosystem: Ecosystem,
    history: Vec<Ecosystem>,
    current_index: usize,
    stats: SimulationStats,
    selected: bool,
}

impl SimulationInstance {
    fn new(config: SimulationConfig) -> Self {
        let ecosystem = Ecosystem::new_custom(config);
        let mut history = Vec::new();
        history.push(ecosystem.clone());

        Self {
            ecosystem,
            history,
            current_index: 0,
            stats: SimulationStats::default(),
            selected: true,
        }
    }

    fn advance(&mut self) {
        self.ecosystem.step(&mut self.stats);
        self.history.push(self.ecosystem.clone());
        self.current_index += 1;
    }

    fn go_back(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.ecosystem = self.history[self.current_index].clone();
        }
    }

    fn iteration_count(&self) -> usize {
        self.ecosystem.iteration_count
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app_state = AppState::SimulationSelector;
    let cell_size: f32 = 12.5;
    let offset_x: f32 = 100.0;
    let offset_y: f32 = 50.0;

    let mut num_simulations = 2;
    let mut current_config_index = 0;
    let mut selected_field_index = 0;
    let mut configs: Vec<Vec<ConfigField>> = Vec::new();
    let mut simulations: Vec<SimulationInstance> = Vec::new();
    let mut all_selected = true;

    loop {
        clear_background(BLACK);

        match app_state {
            AppState::SimulationSelector => {
                let screen_width = screen_width();
                let screen_height = screen_height();
                let center_x = screen_width / 2.0;
                let start_y = screen_height / 3.0;

                let title_width = 400.0;
                let title_height = 80.0;
                let title_x = center_x - title_width / 2.0;
                let title_y = start_y - 100.0;

                draw_rectangle(title_x, title_y, title_width, title_height, Color::new(0.2, 0.1, 0.3, 0.7));
                draw_rectangle_lines(title_x, title_y, title_width, title_height, 2.0, VIOLET);
                draw_text("Rust.eze", center_x - 100.0, title_y + 55.0, 60.0, VIOLET);

                let options_width = 500.0;
                let options_height = 300.0;
                let options_x = center_x - options_width / 2.0;
                let options_y = start_y;

                draw_rectangle(options_x, options_y, options_width, options_height, Color::new(0.1, 0.1, 0.1, 0.8));
                draw_rectangle_lines(options_x, options_y, options_width, options_height, 2.0, WHITE);

                draw_text("Select number of simulations", center_x - 180.0, options_y + 40.0, 30.0, WHITE);

                let two_sim_color = if num_simulations == 2 { GREEN } else { WHITE };
                let four_sim_color = if num_simulations == 4 { GREEN } else { WHITE };

                let option_y = options_y + 100.0;
                let option_height = 50.0;
                let option_width = 300.0;
                let option_x = center_x - option_width / 2.0;

                if num_simulations == 2 {
                    draw_rectangle(option_x, option_y, option_width, option_height, Color::new(0.0, 0.5, 0.0, 0.3));
                }
                draw_rectangle_lines(option_x, option_y, option_width, option_height, 1.0, two_sim_color);
                draw_text("2 Simulations", center_x - 80.0, option_y + 35.0, 25.0, two_sim_color);

                if num_simulations == 4 {
                    draw_rectangle(option_x, option_y + 70.0, option_width, option_height, Color::new(0.0, 0.5, 0.0, 0.3));
                }
                draw_rectangle_lines(option_x, option_y + 70.0, option_width, option_height, 1.0, four_sim_color);
                draw_text("4 Simulations", center_x - 80.0, option_y + 105.0, 25.0, four_sim_color);

                let instructions_y = options_y + options_height + 30.0;
                draw_text("Up/Down: Select Option", center_x - 120.0, instructions_y, 20.0, WHITE);
                draw_text("Enter: Continue to Configuration", center_x - 160.0, instructions_y + 30.0, 20.0, WHITE);
                draw_text("Esc: Quit", center_x - 50.0, instructions_y + 60.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Up) && num_simulations == 4 {
                    num_simulations = 2;
                }

                if is_key_pressed(KeyCode::Down) && num_simulations == 2 {
                    num_simulations = 4;
                }

                if is_key_pressed(KeyCode::Enter) {
                    configs.clear();

                    let default_configs = match num_simulations {
                        2 => vec![
                            SimulationConfig::default(),
                            {
                                let mut config = SimulationConfig::default();
                                config.initial_carnivores = 0;
                                config
                            }, 
                        ],
                        4 => vec![
                            SimulationConfig::default(), 
                            {
                                let mut config = SimulationConfig::default();
                                config.initial_omnivores = 0;
                                config
                            }, 
                            {
                                let mut config = SimulationConfig::default();
                                config.initial_carnivores = 0;
                                config
                            }, 
                            {
                                let mut config = SimulationConfig::default();
                                config.water_spawn_chance = 0.0;
                                config.tree_spawn_chance = 0.0;
                                config
                            }, 
                        ],
                        _ => vec![SimulationConfig::default()],
                    };

                    for config in default_configs {
                        let fields = vec![
                            ConfigField {
                                label: "Initial Light Plants".to_string(),
                                is_int: true,
                                input: config.initial_light_plants.to_string(),
                                color: GREEN,
                            },
                            ConfigField {
                                label: "Initial Dark Plants".to_string(),
                                is_int: true,
                                input: config.initial_dark_plants.to_string(),
                                color: DARK_GREEN,
                            },
                            ConfigField {
                                label: "Initial Herbivores".to_string(),
                                is_int: true,
                                input: config.initial_herbivores.to_string(),
                                color: PINK,
                            },
                            ConfigField {
                                label: "Initial Carnivores".to_string(),
                                is_int: true,
                                input: config.initial_carnivores.to_string(),
                                color: RED,
                            },
                            ConfigField {
                                label: "Initial Omnivores".to_string(),
                                is_int: true,
                                input: config.initial_omnivores.to_string(),
                                color: ORANGE,
                            },
                            ConfigField {
                                label: "Lakes Spawn Chance".to_string(),
                                is_int: false,
                                input: config.water_spawn_chance.to_string(),
                                color: BLUE,
                            },
                            ConfigField {
                                label: "Trees Spawn Chance".to_string(),
                                is_int: false,
                                input: config.tree_spawn_chance.to_string(),
                                color: BROWN,
                            },
                        ];

                        configs.push(fields);
                    }

                    current_config_index = 0;
                    selected_field_index = 0;
                    app_state = AppState::ConfigMenu;
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            },

            AppState::ConfigMenu => {
                let start_x = offset_x;
                let mut y = offset_y;

                y += 30.0;
                draw_text("Rust.eze", start_x, y, 50.0, VIOLET);
                y += 60.0;

                draw_text(&format!("Configuration for Simulation {}", current_config_index + 1), start_x, y, 30.0, YELLOW);
                y += 40.0;

                let fields = &mut configs[current_config_index];

                for (i, field) in fields.iter().enumerate() {
                    let font_size = if i == selected_field_index { 22.5 } else { 20.0 };
                    let color = if i == selected_field_index { WHITE } else { field.color };

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

                if current_config_index < num_simulations - 1 {
                    draw_text("Right Arrow: Next Simulation", start_x, y, 20.0, WHITE);
                    y += 30.0;
                }

                if current_config_index > 0 {
                    draw_text("Left Arrow: Previous Simulation", start_x, y, 20.0, WHITE);
                    y += 30.0;
                }

                draw_text("Enter: Start Simulations", start_x, y, 20.0, WHITE);
                y += 30.0;
                draw_text("Esc: Back to Selector", start_x, y, 20.0, WHITE);

                if is_key_pressed(KeyCode::Up) && selected_field_index > 0 {
                    selected_field_index -= 1;
                }

                if is_key_pressed(KeyCode::Down) && selected_field_index < fields.len() - 1 {
                    selected_field_index += 1;
                }

                let field = &mut fields[selected_field_index];
                if let Some(ch) = get_char_pressed() {
                    if ch.is_ascii_digit() || (ch == '.' && !field.is_int && !field.input.contains('.')) {
                        field.input.push(ch);
                    }
                }

                if is_key_pressed(KeyCode::Backspace) {
                    field.input.pop();
                }

                if is_key_pressed(KeyCode::Right) && current_config_index < num_simulations - 1 {
                    current_config_index += 1;
                    selected_field_index = 0;
                }

                if is_key_pressed(KeyCode::Left) && current_config_index > 0 {
                    current_config_index -= 1;
                    selected_field_index = 0;
                }

                if is_key_pressed(KeyCode::Enter) {
                    simulations.clear();

                    let screen_width = screen_width();
                    let horizontal_spacing = (screen_width - 2.0 * offset_x) / 2.0;
                    let grid_width = (horizontal_spacing - 50.0) / cell_size;

                    let (grid_width, grid_height) = match num_simulations {
                        2 => (grid_width as usize, 52),
                        4 => (grid_width as usize, 26),
                        _ => (grid_width as usize, 52),
                    };

                    let default_config = SimulationConfig::default();

                    for sim_config_fields in &configs {
                        let config = SimulationConfig {
                            grid_width,
                            grid_height,
                            initial_light_plants: sim_config_fields[0].input.parse().unwrap_or(default_config.initial_light_plants),
                            initial_dark_plants: sim_config_fields[1].input.parse().unwrap_or(default_config.initial_dark_plants),
                            initial_herbivores: sim_config_fields[2].input.parse().unwrap_or(default_config.initial_herbivores),
                            initial_carnivores: sim_config_fields[3].input.parse().unwrap_or(default_config.initial_carnivores),
                            initial_omnivores: sim_config_fields[4].input.parse().unwrap_or(default_config.initial_omnivores),
                            water_spawn_chance: sim_config_fields[5].input.parse().unwrap_or(default_config.water_spawn_chance),
                            water_lifespan: default_config.water_lifespan,
                            tree_spawn_chance: sim_config_fields[6].input.parse().unwrap_or(default_config.tree_spawn_chance),
                            tree_lifespan: default_config.tree_lifespan,
                            plant_growth_rate: default_config.plant_growth_rate,
                            herbivore_energy_gain: default_config.herbivore_energy_gain,
                            herbivore_energy_loss: default_config.herbivore_energy_loss,
                            herbivore_initial_energy: default_config.herbivore_initial_energy,
                            herbivore_reproduction_threshold: default_config.herbivore_reproduction_threshold,
                            carnivore_energy_gain: default_config.carnivore_energy_gain,
                            carnivore_energy_loss: default_config.carnivore_energy_loss,
                            carnivore_initial_energy: default_config.carnivore_initial_energy,
                            carnivore_reproduction_threshold: default_config.carnivore_reproduction_threshold,
                            omnivore_energy_gain_plants: default_config.omnivore_energy_gain_plants,
                            omnivore_energy_gain_herbivores: default_config.omnivore_energy_gain_herbivores,
                            omnivore_energy_loss: default_config.omnivore_energy_loss,
                            omnivore_initial_energy: default_config.omnivore_initial_energy,
                            omnivore_reproduction_threshold: default_config.omnivore_reproduction_threshold,
                        };

                        simulations.push(SimulationInstance::new(config));
                    }

                    all_selected = true;
                    app_state = AppState::Simulation;
                }

                if is_key_pressed(KeyCode::Escape) {
                    app_state = AppState::SimulationSelector;
                }
            },

            AppState::Simulation => {
                let screen_width = screen_width();
                let screen_height = screen_height();
                let horizontal_spacing = (screen_width - 2.0 * offset_x) / 2.0;

                let grid_positions = match num_simulations {
                    2 => vec![
                        (offset_x, offset_y),
                        (offset_x + horizontal_spacing, offset_y),
                    ],
                    4 => {

                        let grid_height = simulations[0].ecosystem.height as f32 * cell_size;
                        let stats_height = 40.0; 
                        let total_height = grid_height + stats_height + 20.0; 

                        vec![
                            (offset_x, offset_y),
                            (offset_x + horizontal_spacing, offset_y),
                            (offset_x, offset_y + total_height + 20.0), 
                            (offset_x + horizontal_spacing, offset_y + total_height + 20.0),
                        ]
                    },
                    _ => vec![(offset_x, offset_y)],
                };

                if is_key_pressed(KeyCode::Tab) {
                    if all_selected {
                        all_selected = false;
                        for sim in &mut simulations {
                            sim.selected = false;
                        }
                        if !simulations.is_empty() {
                            simulations[0].selected = true;
                        }
                    } else {
                        let current_index = simulations.iter().position(|s| s.selected).unwrap_or(0);
                        simulations[current_index].selected = false;

                        let next_index = (current_index + 1) % simulations.len();
                        if next_index == 0 {
                            all_selected = true;
                            for sim in &mut simulations {
                                sim.selected = true;
                            }
                        } else {
                            simulations[next_index].selected = true;
                        }
                    }
                }

                if is_key_pressed(KeyCode::Right) {
                    for sim in &mut simulations {
                        if sim.selected || all_selected {
                            sim.advance();
                        }
                    }
                }

                if is_key_pressed(KeyCode::Left) {
                    for sim in &mut simulations {
                        if sim.selected || all_selected {
                            sim.go_back();
                        }
                    }
                }

                if is_key_down(KeyCode::Space) {
                    for sim in &mut simulations {
                        if sim.selected || all_selected {
                            sim.advance();
                        }
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    app_state = AppState::StatsScreen;
                }

                for (idx, sim) in simulations.iter().enumerate() {
                    let (grid_x, grid_y) = grid_positions[idx];
                    let eco = &sim.ecosystem;

                    let border_color = if sim.selected && !all_selected { VIOLET } else { WHITE };
                    let border_thickness = if sim.selected && !all_selected { 3.0 } else { 1.0 };

                    draw_rectangle_lines(
                        grid_x - 5.0, 
                        grid_y - 5.0, 
                        eco.width as f32 * cell_size + 10.0, 
                        eco.height as f32 * cell_size + 10.0, 
                        border_thickness, 
                        border_color
                    );

                    for y in 0..eco.height {
                        for x in 0..eco.width {
                            let mut color = LIGHTGRAY;

                            if eco.trees.iter().any(|t| t.x == x && t.y == y) {
                                color = BROWN;
                            } else if eco.waters.iter().any(|w| w.x == x && w.y == y) {
                                color = BLUE;
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

                            draw_rectangle(
                                grid_x + x as f32 * cell_size, 
                                grid_y + y as f32 * cell_size, 
                                cell_size - 1.0, 
                                cell_size - 1.0, 
                                color
                            );
                        }
                    }

                    let stats_x = grid_x;
                    let stats_y = grid_y + (eco.height as f32 * cell_size) + 18.0;

                    draw_text(&format!("Sim {}: Iteration {}", idx + 1, sim.iteration_count()), stats_x, stats_y, 18.0, YELLOW);

                    let total_light_plants = eco.plants.iter().filter(|p| p.agent_type == AgentType::LightPlant).count();
                    let total_dark_plants = eco.plants.iter().filter(|p| p.agent_type == AgentType::DarkPlant).count();

                    draw_text(&format!("Light Plants: {}", total_light_plants), stats_x, stats_y + 16.0, 15.0, GREEN);
                    draw_text(&format!("Dark Plants: {}", total_dark_plants), stats_x + 140.0, stats_y + 16.0, 15.0, DARK_GREEN);
                    draw_text(&format!("Herbivores: {}", eco.herbivores.len()), stats_x + 270.0, stats_y + 16.0, 15.0, PINK);
                    draw_text(&format!("Carnivores: {}", eco.carnivores.len()), stats_x + 390.0, stats_y + 16.0, 15.0, RED);
                    draw_text(&format!("Omnivores: {}", eco.omnivores.len()), stats_x + 510.0, stats_y + 16.0, 15.0, ORANGE);
                }

                let control_y = screen_height - 20.0;

                draw_text("Space: Continuous Update | Left/Right: Previous/Next Frame | Tab: Cycle Selection | Esc: Statistics", 
                          offset_x, control_y, 18.0, WHITE);
            },

            AppState::StatsScreen => {
                draw_text("Simulation Statistics", offset_x, offset_y + 15.0, 30.0, WHITE);

                let column_width = 450.0;
                let num_rows = if num_simulations <= 2 { 1 } else { 2 };

                for idx in 0..simulations.len() {
                    let row = idx / 2;
                    let col = idx % 2;

                    let x_pos = offset_x + (col as f32) * column_width;
                    let y_pos = offset_y + 60.0 + (row as f32) * 350.0;

                    let sim = &simulations[idx];
                    draw_text(&format!("Simulation {}", idx + 1), x_pos, y_pos, 25.0, YELLOW);

                    let mut line_y = y_pos + 30.0;

                    draw_text(&format!("Iteration Count: {}", sim.iteration_count()), x_pos, line_y, 20.0, WHITE);
                    line_y += 25.0;

                    let stats = &sim.stats;

                    draw_text("Light Plants", x_pos, line_y, 20.0, GREEN);
                    line_y += 20.0;
                    draw_text(&format!("Births: {} Deaths: {}", stats.light_plant_births, stats.light_plant_deaths), 
                              x_pos, line_y, 18.0, GREEN);
                    line_y += 25.0;

                    draw_text("Dark Plants", x_pos, line_y, 20.0, DARK_GREEN);
                    line_y += 20.0;
                    draw_text(&format!("Births: {} Deaths: {}", stats.dark_plant_births, stats.dark_plant_deaths), 
                              x_pos, line_y, 18.0, DARK_GREEN);
                    line_y += 25.0;

                    draw_text("Herbivores", x_pos, line_y, 20.0, PINK);
                    line_y += 20.0;
                    draw_text(&format!("Births: {} Deaths: {} Consumptions: {}", 
                                      stats.herbivore_births, stats.herbivore_deaths, stats.herbivore_consumptions), 
                              x_pos, line_y, 18.0, PINK);
                    line_y += 25.0;

                    draw_text("Carnivores", x_pos, line_y, 20.0, RED);
                    line_y += 20.0;
                    draw_text(&format!("Births: {} Deaths: {} Consumptions: {}", 
                                      stats.carnivore_births, stats.carnivore_deaths, stats.carnivore_consumptions), 
                              x_pos, line_y, 18.0, RED);
                    line_y += 25.0;

                    draw_text("Omnivores", x_pos, line_y, 20.0, ORANGE);
                    line_y += 20.0;
                    draw_text(&format!("Births: {} Deaths: {} P: {} H: {}", 
                                      stats.omnivore_births, stats.omnivore_deaths,
                                      stats.omnivore_consumptions_plants, stats.omnivore_consumptions_herbivores), 
                              x_pos, line_y, 18.0, ORANGE);
                }

                let instructions_y = offset_y + 40.0 + (num_rows as f32) * 350.0 + 20.0;
                draw_text("Press Esc to Return to Simulations", offset_x, instructions_y, 20.0, WHITE);
                draw_text("Press X to Quit", offset_x, instructions_y + 30.0, 20.0, WHITE);

                if is_key_pressed(KeyCode::Escape) {
                    app_state = AppState::Simulation;
                }

                if is_key_pressed(KeyCode::X) {
                    break;
                }
            },
        }

        next_frame().await;
    }
}