/*
    EcoSim Main Module

    This module initializes the simulation
    window (using macroquad), handles the
    configuration menu, user input, simulation
    state updates, and rendering. It also
    supports agent tracking and history
    navigation through simulation steps.
*/

use macroquad::prelude::*;
use crate::config::AgentType;

use crate::config::{SimulationConfig, Agent};
use crate::ecosystem::Ecosystem;

mod config;
mod ecosystem;

const VIOLET: Color = Color::new(0.5, 0.0, 0.5, 1.0);
    // Color used for tracking a selected
    // agent.
const DARK_GREEN: Color = Color::new(0.0, 0.5, 0.0, 1.0);
    // PINK and RED are used for herbivores and
    // carnivores, respectively.

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

/// Application state used to toggle between the
/// configuration menu and simulation.
enum AppState {
    ConfigMenu,
    Simulation,
}

/// Represents a configurable field in the
/// simulation configuration menu.
struct ConfigField {
    /// The label shown for the field.
    label: String,
    /// The current numerical value of the field.
    value: f32,
    /// Indicates whether the field should be
    /// treated as an integer.
    is_int: bool,
    /// String representation of the user input.
    input: String,
}

impl ConfigField {
    /// Returns the display string for the
    /// configuration field.
    fn display_value(&self) -> String {
        self.input.clone()
    }
}

/// Structure used to track an agent's details
/// (for herbivores or carnivores).
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
    /// Creates a new TrackingInfo instance based on
    /// an agent.
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
    
    /// Updates the tracked information with the
    /// agent's current state.
    fn overwrite(&mut self, agent: &Agent) {
        self.x = agent.x;
        self.y = agent.y;
        self.energy = agent.energy;
        self.died = None;
    }
    
    /// Marks the agent as dead with a specified
    /// cause.
    fn mark_death(&mut self, cause: &str) {
        self.died = Some(cause.to_string());
    }
}

/// Advances the simulation by one step and
/// updates tracking information for the selected
/// agent.
fn advance_simulation(
    eco: &mut Ecosystem,
    track: &mut Option<TrackingInfo>
) {
    eco.step();
    if let Some(t) = track {
        if let Some(agent) = eco.herbivores.iter()
            .find(|h| h.id == t.agent_id)
            .or_else(|| eco.carnivores.iter()
                .find(|c| c.id == t.agent_id))
        {
            t.overwrite(agent);
        } else if t.died.is_none() {
            t.mark_death("Energy Depletion");
        }
    }
}

/// Loads a specific simulation state from history
/// and updates the tracked agent.
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
            if let Some(agent) = eco.herbivores.iter()
                .find(|h| h.id == t.agent_id)
                .or_else(|| eco.carnivores.iter()
                    .find(|c| c.id == t.agent_id))
            {
                t.overwrite(agent);
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
    let grid_width: usize = 140;
    let grid_height: usize = 65;
    let offset_x: f32 = 50.0;
    let offset_y: f32 = 50.0;

    // Define configuration fields for the simulation.
    let mut fields = vec![
        ConfigField {
            label: "Initial Plants".to_string(),
            value: 100.0,
            is_int: true,
            input: "100".to_string()
        },
        ConfigField {
            label: "Initial Dark Green Plants".to_string(),
            value: 50.0,
            is_int: true,
            input: "50".to_string()
        },
        ConfigField {
            label: "Initial Herbivores".to_string(),
            value: 300.0,
            is_int: true,
            input: "300".to_string()
        },
        ConfigField {
            label: "Initial Carnivores".to_string(),
            value: 100.0,
            is_int: true,
            input: "100".to_string()
        },
        ConfigField {
            label: "Plant Growth Rate".to_string(),
            value: 0.25,
            is_int: false,
            input: "0.25".to_string()
        },
        ConfigField {
            label: "Herbivore Reproduction Rate".to_string(),
            value: 0.12,
            is_int: false,
            input: "0.12".to_string()
        },
        ConfigField {
            label: "Herbivore Energy Gain".to_string(),
            value: 7.0,
            is_int: true,
            input: "7".to_string()
        },
        ConfigField {
            label: "Herbivore Energy Loss".to_string(),
            value: 1.0,
            is_int: true,
            input: "1".to_string()
        },
        ConfigField {
            label: "Herbivore Initial Energy".to_string(),
            value: 30.0,
            is_int: true,
            input: "30".to_string()
        },
        ConfigField {
            label: "Herbivore Reproduction Threshold".to_string(),
            value: 12.0,
            is_int: true,
            input: "12".to_string()
        },
        ConfigField {
            label: "Carnivore Reproduction Rate".to_string(),
            value: 0.12,
            is_int: false,
            input: "0.12".to_string()
        },
        ConfigField {
            label: "Carnivore Energy Gain".to_string(),
            value: 20.0,
            is_int: true,
            input: "20".to_string()
        },
        ConfigField {
            label: "Carnivore Energy Loss".to_string(),
            value: 1.0,
            is_int: true,
            input: "1".to_string()
        },
        ConfigField {
            label: "Carnivore Initial Energy".to_string(),
            value: 80.0,
            is_int: true,
            input: "80".to_string()
        },
        ConfigField {
            label: "Carnivore Reproduction Threshold".to_string(),
            value: 20.0,
            is_int: true,
            input: "20".to_string()
        },
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
                // Display each configuration field.
                for (i, field) in fields.iter().enumerate() {
                    let color = if i == selected_field {
                        YELLOW
                    } else {
                        WHITE
                    };
                    draw_text(
                        &format!("{}: {}",
                            field.label,
                            field.display_value()
                        ),
                        start_x,
                        y,
                        20.0,
                        color
                    );
                    y += 30.0;
                }
                y += 20.0;
                draw_text(
                    "Up/Down: Change field selection",
                    start_x,
                    y,
                    20.0,
                    WHITE
                );
                y += 30.0;
                draw_text(
                    "Type numbers and '.' to modify values",
                    start_x,
                    y,
                    20.0,
                    WHITE
                );
                y += 30.0;
                draw_text(
                    "Backspace: Delete | Enter: Confirm and start",
                    start_x,
                    y,
                    20.0,
                    WHITE
                );

                if is_key_pressed(KeyCode::Up) {
                    if selected_field > 0 {
                        selected_field -= 1;
                    }
                }
                if is_key_pressed(KeyCode::Down) {
                    if selected_field < fields.len() - 1 {
                        selected_field += 1;
                    }
                }

                {
                    let field = &mut fields[selected_field];
                    if let Some(ch) = get_char_pressed() {
                        if ch.is_ascii_digit() ||
                           (ch == '.' && !field.is_int &&
                           !field.input.contains('.'))
                        {
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
                        initial_plants: fields[0].input
                            .parse::<usize>()
                            .unwrap_or(fields[0].value as usize),
                        initial_dark_green_plants: fields[1].input
                            .parse::<usize>()
                            .unwrap_or(fields[1].value as usize),
                        initial_herbivores: fields[2].input
                            .parse::<usize>()
                            .unwrap_or(fields[2].value as usize),
                        initial_carnivores: fields[3].input
                            .parse::<usize>()
                            .unwrap_or(fields[3].value as usize),
                        plant_growth_rate: fields[4].input
                            .parse::<f32>()
                            .unwrap_or(fields[4].value),
                        herbivore_reproduction_rate: fields[5].input
                            .parse::<f32>()
                            .unwrap_or(fields[5].value),
                        herbivore_energy_gain: fields[6].input
                            .parse::<i32>()
                            .unwrap_or(fields[6].value as i32),
                        herbivore_energy_loss: fields[7].input
                            .parse::<i32>()
                            .unwrap_or(fields[7].value as i32),
                        herbivore_initial_energy: fields[8].input
                            .parse::<i32>()
                            .unwrap_or(fields[8].value as i32),
                        herbivore_reproduction_threshold: fields[9].input
                            .parse::<i32>()
                            .unwrap_or(fields[9].value as i32),
                        carnivore_reproduction_rate: fields[10].input
                            .parse::<f32>()
                            .unwrap_or(fields[10].value),
                        carnivore_energy_gain: fields[11].input
                            .parse::<i32>()
                            .unwrap_or(fields[11].value as i32),
                        carnivore_energy_loss: fields[12].input
                            .parse::<i32>()
                            .unwrap_or(fields[12].value as i32),
                        carnivore_initial_energy: fields[13].input
                            .parse::<i32>()
                            .unwrap_or(fields[13].value as i32),
                        carnivore_reproduction_threshold: fields[14].input
                            .parse::<i32>()
                            .unwrap_or(fields[14].value as i32),
                    };
                    ecosystem = Some(
                        Ecosystem::new_custom(config)
                    );
                    history.push(
                        ecosystem.as_ref().unwrap().clone()
                    );
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
                        if mx >= offset_x &&
                           my >= offset_y &&
                           mx < offset_x + (grid_width as f32 *
                           cell_size) &&
                           my < offset_y + (grid_height as f32 *
                           cell_size)
                        {
                            let cx = ((mx - offset_x) /
                                cell_size)
                                .floor() as usize;
                            let cy = ((my - offset_y) /
                                cell_size)
                                .floor() as usize;
                            if eco.herbivores.iter()
                                .find(|a| a.x == cx &&
                                a.y == cy)
                                .is_none() &&
                               eco.carnivores.iter()
                                .find(|a| a.x == cx &&
                                a.y == cy)
                                .is_none()
                            {
                                tracking = None;
                            } else {
                                if let Some(a) =
                                    eco.herbivores.iter()
                                        .find(|a| a.x == cx &&
                                        a.y == cy)
                                    .or_else(|| eco.carnivores.iter()
                                        .find(|a| a.x == cx &&
                                        a.y == cy))
                                {
                                    tracking =
                                        Some(TrackingInfo::new(a));
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
                        load_from_history(
                            &history,
                            current_index,
                            &mut ecosystem,
                            &mut tracking,
                            &mut simulation_step
                        );
                    }
                }
                if is_key_pressed(KeyCode::Right) {
                    if current_index < history.len() - 1 {
                        current_index += 1;
                        load_from_history(
                            &history,
                            current_index,
                            &mut ecosystem,
                            &mut tracking,
                            &mut simulation_step
                        );
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

                if let Some(ref eco) = ecosystem {
                    for y in 0..eco.height {
                        for x in 0..eco.width {
                            let mut color = LIGHTGRAY;
                            if let Some(t) = &tracking {
                                if (eco.herbivores.iter().any(|a| 
                                    a.id == t.agent_id &&
                                    a.x == x && a.y == y))
                                    || (eco.carnivores.iter().any(|a| 
                                    a.id == t.agent_id &&
                                    a.x == x && a.y == y))
                                {
                                    color = VIOLET;
                                } else if eco.carnivores.iter()
                                    .any(|c| c.x == x &&
                                    c.y == y)
                                {
                                    color = RED;
                                } else if eco.herbivores.iter()
                                    .any(|h| h.x == x &&
                                    h.y == y)
                                {
                                    color = PINK;
                                } else if eco.plants.iter()
                                    .any(|p| p.x == x &&
                                    p.y == y)
                                {
                                    if eco.plants.iter().any(|p| 
                                        p.x == x && p.y == y &&
                                        p.agent_type ==
                                        AgentType::DarkGreenPlant)
                                    {
                                        color = DARK_GREEN;
                                    } else {
                                        color = GREEN;
                                    }
                                }
                            } else {
                                if eco.carnivores.iter().any(|c| 
                                    c.x == x && c.y == y)
                                {
                                    color = RED;
                                } else if eco.herbivores.iter().any(|h| 
                                    h.x == x && h.y == y)
                                {
                                    color = PINK;
                                } else if eco.plants.iter().any(|p| 
                                    p.x == x && p.y == y)
                                {
                                    if eco.plants.iter().any(|p| 
                                        p.x == x && p.y == y &&
                                        p.agent_type ==
                                        AgentType::DarkGreenPlant)
                                    {
                                        color = DARK_GREEN;
                                    } else {
                                        color = GREEN;
                                    }
                                }
                            }
                            draw_rectangle(
                                offset_x + x as f32 * cell_size,
                                offset_y + y as f32 * cell_size,
                                cell_size - 1.0,
                                cell_size - 1.0,
                                color
                            );
                        }
                    }
                }

                let avg_plants: f32 = if !history.is_empty() {
                    history.iter()
                        .map(|eco| eco.plants.len())
                        .sum::<usize>() as f32 /
                    history.len() as f32
                } else { 0.0 };
                let avg_herbivores: f32 = if !history.is_empty() {
                    history.iter()
                        .map(|eco| eco.herbivores.len())
                        .sum::<usize>() as f32 /
                    history.len() as f32
                } else { 0.0 };
                let avg_carnivores: f32 = if !history.is_empty() {
                    history.iter()
                        .map(|eco| eco.carnivores.len())
                        .sum::<usize>() as f32 /
                    history.len() as f32
                } else { 0.0 };

                let base_x = offset_x;
                let base_y = offset_y + (grid_height as f32 *
                    cell_size) + 30.0;

                draw_text(
                    "Left/Right: Step Backward/Forward",
                    base_x,
                    base_y,
                    20.0,
                    WHITE
                );
                draw_text(
                    "Hold Space: Continuous Update",
                    base_x,
                    base_y + 30.0,
                    20.0,
                    WHITE
                );
                draw_text(
                    "Left Click On An Agent To Track It",
                    base_x,
                    base_y + 60.0,
                    20.0,
                    WHITE
                );
                draw_text(
                    "Esc: Quit",
                    base_x,
                    base_y + 90.0,
                    20.0,
                    WHITE
                );

                let stats_x = base_x + 550.0;
                let stats_y = base_y;
                draw_text(
                    &format!("Step: {}", simulation_step),
                    stats_x,
                    stats_y,
                    20.0,
                    YELLOW
                );
                draw_text(
                    &format!("Plants: {} (Avg: {:.1})",
                        if let Some(ref eco) = ecosystem {
                            eco.plants.len()
                        } else { 0 },
                        avg_plants
                    ),
                    stats_x,
                    stats_y + 30.0,
                    20.0,
                    GREEN
                );
                draw_text(
                    &format!("Herbivores: {} (Avg: {:.1})",
                        if let Some(ref eco) = ecosystem {
                            eco.herbivores.len()
                        } else { 0 },
                        avg_herbivores
                    ),
                    stats_x,
                    stats_y + 60.0,
                    20.0,
                    PINK
                );
                draw_text(
                    &format!("Carnivores: {} (Avg: {:.1})",
                        if let Some(ref eco) = ecosystem {
                            eco.carnivores.len()
                        } else { 0 },
                        avg_carnivores
                    ),
                    stats_x,
                    stats_y + 90.0,
                    20.0,
                    RED
                );

                let track_x = stats_x + 500.0;
                let track_y = base_y;
                if let Some(t) = &tracking {
                    draw_text(
                        "Tracked Agent Info:",
                        track_x,
                        track_y,
                        20.0,
                        VIOLET
                    );
                    draw_text(
                        &format!("Born: ({}, {})",
                            t.born_x, t.born_y),
                        track_x,
                        track_y + 30.0,
                        20.0,
                        WHITE
                    );
                    draw_text(
                        &format!("Position: ({}, {})",
                            t.x, t.y),
                        track_x,
                        track_y + 60.0,
                        20.0,
                        WHITE
                    );
                    draw_text(
                        &format!("Energy: {}",
                            t.energy),
                        track_x,
                        track_y + 90.0,
                        20.0,
                        WHITE
                    );
                    let died_text = t.died.as_deref()
                        .unwrap_or("Not Yet");
                    draw_text(
                        &format!("Died: {}",
                            died_text),
                        track_x,
                        track_y + 120.0,
                        20.0,
                        WHITE
                    );
                }

                if is_key_pressed(KeyCode::Escape) {
                    break;
                }
            }
        }
        next_frame().await;
    }
}
