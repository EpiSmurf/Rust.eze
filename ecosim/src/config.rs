/*
    Simulation Configuration and Agent Definitions

    This module defines the configuration parameters for the simulation (grid size, initial populations,
    growth and energy parameters, etc.) and provides the Agent type and its associated enum.
*/

#[derive(Clone)]
/// Configuration parameters for the simulation.
pub struct SimulationConfig {
    /// Width of the simulation grid.
    pub grid_width: usize,
    /// Height of the simulation grid.
    pub grid_height: usize,
    /// Initial number of plants.
    pub initial_plants: usize,
    /// Initial number of dark green plants.
    pub initial_dark_green_plants: usize,
    /// Initial number of herbivores.
    pub initial_herbivores: usize,
    /// Initial number of carnivores.
    pub initial_carnivores: usize,
    /// Growth rate of plants.
    pub plant_growth_rate: f32,
    /// Reproduction rate for herbivores.
    pub herbivore_reproduction_rate: f32,
    /// Energy gained by a herbivore when consuming a plant.
    pub herbivore_energy_gain: i32,
    /// Energy lost by a herbivore each step.
    pub herbivore_energy_loss: i32,
    /// Initial energy of herbivores.
    pub herbivore_initial_energy: i32,
    /// Energy threshold for herbivore reproduction.
    pub herbivore_reproduction_threshold: i32,
    /// Reproduction rate for carnivores.
    pub carnivore_reproduction_rate: f32,
    /// Energy gained by a carnivore when consuming a herbivore.
    pub carnivore_energy_gain: i32,
    /// Energy lost by a carnivore each step.
    pub carnivore_energy_loss: i32,
    /// Initial energy of carnivores.
    pub carnivore_initial_energy: i32,
    /// Energy threshold for carnivore reproduction.
    pub carnivore_reproduction_threshold: i32,
}

impl Default for SimulationConfig {
    /// Provides a default simulation configuration.
    fn default() -> Self {
        SimulationConfig {
            grid_width: 140,
            grid_height: 65,
            initial_plants: 100,
            initial_dark_green_plants: 50,
            initial_herbivores: 300,
            initial_carnivores: 100,
            plant_growth_rate: 0.25,
            herbivore_reproduction_rate: 0.12,
            herbivore_energy_gain: 7,
            herbivore_energy_loss: 1,
            herbivore_initial_energy: 30,
            herbivore_reproduction_threshold: 12,
            carnivore_reproduction_rate: 0.12,
            carnivore_energy_gain: 20,
            carnivore_energy_loss: 1,
            carnivore_initial_energy: 80,
            carnivore_reproduction_threshold: 20,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Types of agents in the simulation.
pub enum AgentType {
    /// Represents a normal plant.
    Plant,
    /// Represents a dark green plant.
    DarkGreenPlant, //
    /// Represents a herbivore.
    Herbivore,
    /// Represents a carnivore.
    Carnivore,
}

#[derive(Debug, Clone)]
/// Represents an agent in the simulation.
pub struct Agent {
    /// Unique identifier for the agent.
    pub id: u32,
    /// The type of the agent (Plant, DarkGreenPlant, Herbivore, or Carnivore).
    pub agent_type: AgentType,
    /// X-coordinate position on the grid.
    pub x: usize,
    /// Y-coordinate position on the grid.
    pub y: usize,
    /// Current energy level of the agent.
    pub energy: i32,
}

impl Agent {
    /// Creates a new agent with the specified properties.
    pub fn new(id: u32, agent_type: AgentType, x: usize, y: usize, energy: i32) -> Self {
        Agent { id, agent_type, x, y, energy }
    }
}
