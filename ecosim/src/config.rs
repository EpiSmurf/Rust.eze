#[derive(Clone)]
pub struct SimulationConfig {
    pub grid_width: usize,
    pub grid_height: usize,
    pub initial_plants: usize,
    pub initial_herbivores: usize,
    pub plant_growth_rate: f32,
    pub herbivore_reproduction_rate: f32,
    pub herbivore_energy_gain: i32,
    pub herbivore_energy_loss: i32,
    pub herbivore_initial_energy: i32,
    pub herbivore_reproduction_threshold: i32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            grid_width: 140,  // Mise à jour : 140 colonnes
            grid_height: 65,  // 65 lignes
            initial_plants: 300,
            initial_herbivores: 150,
            plant_growth_rate: 0.2,
            herbivore_reproduction_rate: 0.1,
            herbivore_energy_gain: 5,
            herbivore_energy_loss: 1,
            herbivore_initial_energy: 10,
            herbivore_reproduction_threshold: 15,
        }
    }
}
