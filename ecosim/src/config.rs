#[derive(Clone)]
pub struct SimulationConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub initial_light_plants: usize,
    pub initial_dark_plants: usize,
    pub initial_herbivores: usize,
    pub initial_carnivores: usize,
    pub initial_omnivores: usize,
    pub plant_growth_rate: f32,
    pub herbivore_energy_gain: i32,
    pub herbivore_energy_loss: i32,
    pub herbivore_initial_energy: i32,
    pub herbivore_reproduction_threshold: i32,
    pub carnivore_energy_gain: i32,
    pub carnivore_energy_loss: i32,
    pub carnivore_initial_energy: i32,
    pub carnivore_reproduction_threshold: i32,
    pub omnivore_energy_gain_plants: i32,
    pub omnivore_energy_gain_herbivores: i32,
    pub omnivore_energy_loss: i32,
    pub omnivore_initial_energy: i32,
    pub omnivore_reproduction_threshold: i32,
    pub water_spawn_chance: f32,
    pub water_lifespan: usize,
    pub tree_spawn_chance: f32,
    pub tree_lifespan: usize,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            grid_width: 57,
            grid_height: 52,
            initial_light_plants: 150,
            initial_dark_plants: 75,
            initial_herbivores: 120,
            initial_carnivores: 40,
            initial_omnivores: 40,
            plant_growth_rate: 0.20,
            herbivore_energy_gain: 7,
            herbivore_energy_loss: 1,
            herbivore_initial_energy: 30,
            herbivore_reproduction_threshold: 15,
            carnivore_energy_gain: 10,
            carnivore_energy_loss: 1,
            carnivore_initial_energy: 120,
            carnivore_reproduction_threshold: 20,
            omnivore_energy_gain_plants: 2,
            omnivore_energy_gain_herbivores: 5,
            omnivore_energy_loss: 1,
            omnivore_initial_energy: 45,
            omnivore_reproduction_threshold: 25,
            water_spawn_chance: 0.005,
            water_lifespan: 500,
            tree_spawn_chance: 0.005,
            tree_lifespan: 500,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentType {
    LightPlant,
    DarkPlant,
    Herbivore,
    Carnivore,
    Omnivore,
    Water,
    Tree,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: u32,
    pub agent_type: AgentType,
    pub x: usize,
    pub y: usize,
    pub energy: i32,
    pub pending_death: bool,
    pub death_cause: Option<String>,
    pub birth_iteration: Option<usize>,
}

impl Agent {
    pub fn new(id: u32, agent_type: AgentType, x: usize, y: usize, energy: i32) -> Self {
        Self {
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

    pub fn new_water(id: u32, x: usize, y: usize, birth: usize) -> Self {
        Self {
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

    pub fn new_tree(id: u32, x: usize, y: usize, birth: usize) -> Self {
        Self {
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
