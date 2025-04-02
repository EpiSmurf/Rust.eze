/*
    Simulation Configuration and Agent
    Definitions

    This module définit les paramètres de configuration
    pour la simulation (taille de la grille, populations
    initiales, taux de croissance, énergie, etc.) et
    fournit le type Agent et son enum associé.
*/

#[derive(Clone)]
/// Configuration parameters for the simulation.
pub struct SimulationConfig {
    /// Width of the simulation grid.
    pub grid_width: usize,
    /// Height of the simulation grid.
    pub grid_height: usize,

    /// Initial number of light plants.
    pub initial_plants: usize,
    /// Initial number of dark plants.
    pub initial_dark_plants: usize,

    /// Initial number of herbivores.
    pub initial_herbivores: usize,
    /// Initial number of carnivores.
    pub initial_carnivores: usize,
    /// Initial number of omnivores.
    pub initial_omnivores: usize,

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

    /// Reproduction rate for omnivores.
    pub omnivore_reproduction_rate: f32,
    /// Energy gained by an omnivore when consuming a plant.
    pub omnivore_energy_gain_plants: i32,
    /// Energy gained by an omnivore when consuming a herbivore.
    pub omnivore_energy_gain_herbivores: i32,
    /// Energy lost by an omnivore each step.
    pub omnivore_energy_loss: i32,
    /// Initial energy of omnivores.
    pub omnivore_initial_energy: i32,
    /// Energy threshold for omnivore reproduction.
    pub omnivore_reproduction_threshold: i32,

    /// Chance each iteration to spawn a water patch.
    pub water_spawn_chance: f32,
    /// Duration (in iterations) after which water disappears.
    pub water_lifespan: usize,

    /// Chance each iteration to spawn a tree patch.
    pub tree_spawn_chance: f32,
    /// Duration (in iterations) after which trees disappear.
    pub tree_lifespan: usize,
}

impl Default for SimulationConfig {
    /// Provides a default simulation configuration.
    fn default() -> Self {
        SimulationConfig {
            grid_width: 114,
            grid_height: 52,

            initial_plants: 1750,
            initial_dark_plants: 1750,

            initial_herbivores: 520,
            initial_carnivores: 740,
            initial_omnivores: 1300,

            plant_growth_rate: 0.25,
            herbivore_reproduction_rate: 0.15,
            herbivore_energy_gain: 10,
            herbivore_energy_loss: 1,
            herbivore_initial_energy: 30,
            herbivore_reproduction_threshold: 10,

            carnivore_reproduction_rate: 0.10,
            carnivore_energy_gain: 25,
            carnivore_energy_loss: 1,
            carnivore_initial_energy: 80,
            carnivore_reproduction_threshold: 20,

            omnivore_reproduction_rate: 0.10,
            omnivore_energy_gain_plants: 3,
            omnivore_energy_gain_herbivores: 10,
            omnivore_energy_loss: 2,
            omnivore_initial_energy: 35,
            omnivore_reproduction_threshold: 20,

            water_spawn_chance: 0.01,
            water_lifespan: 400,

            tree_spawn_chance: 0.02,
            tree_lifespan: 400,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Types of agents in the simulation.
pub enum AgentType {
    /// Represents a light plant.
    LightPlant,
    /// Represents a dark plant.
    DarkPlant,
    /// Represents a herbivore.
    Herbivore,
    /// Represents a carnivore.
    Carnivore,
    /// Represents an omnivore.
    Omnivore,
    /// Represents a water patch.
    Water,
    /// Represents a tree (dark brown), 2x2.
    Tree,
}

#[derive(Debug, Clone)]
/// Represents an agent in the simulation.
pub struct Agent {
    /// Unique identifier for the agent.
    pub id: u32,
    /// The type of the agent.
    pub agent_type: AgentType,
    /// X-coordinate position on the grid.
    pub x: usize,
    /// Y-coordinate position on the grid.
    pub y: usize,
    /// Current energy level of the agent.
    pub energy: i32,
    /// Flag indicating if the agent is marked to die next iteration.
    pub pending_death: bool,
    /// The cause of death (if applicable).
    pub death_cause: Option<String>,
    /// For water and trees: iteration of birth.
    pub birth_iteration: Option<usize>,
}

impl Agent {
    /// Creates a new agent with the specified properties.
    pub fn new(
        id: u32,
        agent_type: AgentType,
        x: usize,
        y: usize,
        energy: i32,
    ) -> Self {
        Agent {
            id,
            agent_type,
            x,
            y,
            energy,
            pending_death: false,
            death_cause: None,
            birth_iteration: None,
        }
    }

    /// Creates a new water patch agent.
    pub fn new_water(
        id: u32,
        x: usize,
        y: usize,
        birth: usize,
    ) -> Self {
        Agent {
            id,
            agent_type: AgentType::Water,
            x,
            y,
            energy: 0,
            pending_death: false,
            death_cause: None,
            birth_iteration: Some(birth),
        }
    }

    /// Creates a new tree agent.
    pub fn new_tree(
        id: u32,
        x: usize,
        y: usize,
        birth: usize,
    ) -> Self {
        Agent {
            id,
            agent_type: AgentType::Tree,
            x,
            y,
            energy: 0,
            pending_death: false,
            death_cause: None,
            birth_iteration: Some(birth),
        }
    }
}
