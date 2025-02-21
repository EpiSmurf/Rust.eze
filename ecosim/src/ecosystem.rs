/*
    Ecosystem Simulation Module

    This module defines the Ecosystem struct, which represents the simulation grid and contains
    the collections of plant, herbivore, and carnivore agents. It also implements the simulation
    step logic and statistics collection for each iteration.
*/

use crate::config::{SimulationConfig, Agent, AgentType};
use rand::Rng;

#[allow(dead_code)]
/// Holds statistical data for a single simulation iteration.
pub struct IterationStats {
    pub herbivores_initial: usize,
    pub herbivores_final: usize,
    pub carnivores_initial: usize,
    pub carnivores_final: usize,
    pub plants_initial: usize,
    pub plants_final: usize,
    pub herbivores_eaten_count: usize,
    pub carnivores_eaten_count: usize,
    pub herbivores_reproduction_count: usize,
    pub carnivores_reproduction_count: usize,
    pub herbivores_died_count: usize,
    pub carnivores_died_count: usize,
    pub herbivores_energy_min: Option<i32>,
    pub herbivores_energy_max: Option<i32>,
    pub herbivores_energy_avg: Option<f32>,
    pub carnivores_energy_min: Option<i32>,
    pub carnivores_energy_max: Option<i32>,
    pub carnivores_energy_avg: Option<f32>,
}

#[allow(dead_code)]
impl IterationStats {
    /// Prints the iteration statistics in a formatted output.
    pub fn print(&self, total_herb_eaten: usize, total_carn_eaten: usize, total_herb_rep: usize, total_carn_rep: usize, total_herb_died: usize, total_carn_died: usize) {
        println!(
            "Iteration Stats:\n Plants: {} -> {},\n Herbivores: {} -> {} (Eaten: {} total: {}), Reproduction: {} (total: {}), Deaths: {} (total: {}),\n Carnivores: {} -> {} (Eaten: {} total: {}), Reproduction: {} (total: {}), Deaths: {} (total: {})",
            self.plants_initial, self.plants_final,
            self.herbivores_initial, self.herbivores_final, self.herbivores_eaten_count, total_herb_eaten,
            self.herbivores_reproduction_count, total_herb_rep, self.herbivores_died_count, total_herb_died,
            self.carnivores_initial, self.carnivores_final, self.carnivores_eaten_count, total_carn_eaten,
            self.carnivores_reproduction_count, total_carn_rep, self.carnivores_died_count, total_carn_died
        );
    }
}

#[derive(Clone)]
/// Represents the entire ecosystem, including grid dimensions and agent collections.
pub struct Ecosystem {
    /// Width of the simulation grid.
    pub width: usize,
    /// Height of the simulation grid.
    pub height: usize,
    /// Vector containing all plant agents.
    pub plants: Vec<Agent>,
    /// Vector containing all herbivore agents.
    pub herbivores: Vec<Agent>,
    /// Vector containing all carnivore agents.
    pub carnivores: Vec<Agent>,
    /// Simulation configuration parameters.
    pub config: SimulationConfig,
    /// Next unique agent identifier.
    pub next_agent_id: u32,
}

impl Ecosystem {
    /// Creates a new ecosystem using a custom simulation configuration.
    pub fn new_custom(config: SimulationConfig) -> Self {
        let width = config.grid_width;
        let height = config.grid_height;
        let mut rng = rand::thread_rng();
        let mut plants = Vec::new();
        let mut herbivores = Vec::new();
        let mut carnivores = Vec::new();
        let mut next_agent_id: u32 = 0;

        // Randomly place the initial plants.
        for _ in 0..config.initial_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::Plant, x, y, 0));
            next_agent_id += 1;
        }

        // Randomly place the initial herbivores.
        for _ in 0..config.initial_herbivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            herbivores.push(Agent::new(next_agent_id, AgentType::Herbivore, x, y, config.herbivore_initial_energy));
            next_agent_id += 1;
        }

        // Randomly place the initial carnivores.
        for _ in 0..config.initial_carnivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            carnivores.push(Agent::new(next_agent_id, AgentType::Carnivore, x, y, config.carnivore_initial_energy));
            next_agent_id += 1;
        }

        Ecosystem {
            width,
            height,
            plants,
            herbivores,
            carnivores,
            config,
            next_agent_id,
        }
    }

    /// Returns a list of indices for plants located at the specified (x, y) coordinates.
    fn find_plants_at(&self, x: usize, y: usize) -> Vec<usize> {
        self.plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| plant.x == x && plant.y == y)
            .map(|(i, _)| i)
            .collect()
    }

    /// Returns a list of indices for herbivores located at the specified (x, y) coordinates.
    fn find_herbivores_at(&self, x: usize, y: usize) -> Vec<usize> {
        self.herbivores
            .iter()
            .enumerate()
            .filter(|(_, herbivore)| herbivore.x == x && herbivore.y == y)
            .map(|(i, _)| i)
            .collect()
    }

    /// Returns a random adjacent coordinate (including the current cell) given a starting position.
    fn random_adjacent_aux(rng: &mut impl Rng, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        let dx: i32 = rng.gen_range(-1..=1);
        let dy: i32 = rng.gen_range(-1..=1);
        let new_x = if dx < 0 {
            x.saturating_sub(dx.abs() as usize)
        } else {
            std::cmp::min(x + dx as usize, width - 1)
        };
        let new_y = if dy < 0 {
            y.saturating_sub(dy.abs() as usize)
        } else {
            std::cmp::min(y + dy as usize, height - 1)
        };
        (new_x, new_y)
    }

    /// Advances the simulation by one step and returns the iteration statistics.
    pub fn step(&mut self) -> IterationStats {
        let mut rng = rand::thread_rng();
        let plants_initial = self.plants.len();
        let herbivores_initial = self.herbivores.len();
        let carnivores_initial = self.carnivores.len();

        // Process plant growth.
        let plants_copy = self.plants.clone();
        let mut new_plants: Vec<Agent> = Vec::new();
        for plant in plants_copy {
            if rng.gen::<f32>() < self.config.plant_growth_rate {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, plant.x, plant.y, self.width, self.height);
                if self.find_plants_at(nx, ny).is_empty()
                    && !new_plants.iter().any(|p: &Agent| p.x == nx && p.y == ny)
                {
                    new_plants.push(Agent::new(self.next_agent_id, AgentType::Plant, nx, ny, 0));
                    self.next_agent_id += 1;
                }
            }
        }
        self.plants.extend(new_plants);

        // Process herbivores.
        let mut herbivores_eaten_count = 0;
        let mut herbivores_reproduction_count = 0;
        let mut herbivores_died_count = 0;
        let current_herbivores = std::mem::take(&mut self.herbivores);
        let mut updated_herbivores = Vec::new();
        let mut new_herbivores = Vec::new();
        for mut herbivore in current_herbivores {
            // Move the herbivore with a probability.
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, self.width, self.height);
                herbivore.x = nx;
                herbivore.y = ny;
            }
            // Deduct energy for movement.
            herbivore.energy -= self.config.herbivore_energy_loss;
            // Check for a plant at the current location.
            let plant_indices = self.find_plants_at(herbivore.x, herbivore.y);
            if !plant_indices.is_empty() {
                let index = plant_indices[0];
                self.plants.swap_remove(index);
                herbivore.energy += self.config.herbivore_energy_gain;
                herbivores_eaten_count += 1;
            }
            // Check if the herbivore can reproduce.
            if herbivore.energy >= self.config.herbivore_reproduction_threshold
                && rng.gen::<f32>() < self.config.herbivore_reproduction_rate
            {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, self.width, self.height);
                let offspring_energy = herbivore.energy / 2;
                herbivore.energy -= offspring_energy;
                new_herbivores.push(Agent::new(self.next_agent_id, AgentType::Herbivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                herbivores_reproduction_count += 1;
            }
            // Check if the herbivore dies.
            if herbivore.energy <= 1 {
                herbivore.energy = 0;
                herbivores_died_count += 1;
            } else {
                updated_herbivores.push(herbivore);
            }
        }
        updated_herbivores.extend(new_herbivores);
        self.herbivores = updated_herbivores;

        // Process carnivores.
        let mut carnivores_eaten_count = 0;
        let mut carnivores_reproduction_count = 0;
        let mut carnivores_died_count = 0;
        let current_carnivores = std::mem::take(&mut self.carnivores);
        let mut updated_carnivores = Vec::new();
        let mut new_carnivores = Vec::new();
        for mut carnivore in current_carnivores {
            // Move the carnivore with a probability.
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, carnivore.x, carnivore.y, self.width, self.height);
                carnivore.x = nx;
                carnivore.y = ny;
            }
            // Deduct energy for movement.
            carnivore.energy -= self.config.carnivore_energy_loss;
            // Check for a herbivore at the current location.
            let herbivore_indices = self.find_herbivores_at(carnivore.x, carnivore.y);
            if !herbivore_indices.is_empty() {
                let index = herbivore_indices[0];
                self.herbivores.swap_remove(index);
                carnivore.energy += self.config.carnivore_energy_gain;
                carnivores_eaten_count += 1;
            }
            // Check if the carnivore can reproduce.
            if carnivore.energy >= self.config.carnivore_reproduction_threshold
                && rng.gen::<f32>() < self.config.carnivore_reproduction_rate
            {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, carnivore.x, carnivore.y, self.width, self.height);
                let offspring_energy = carnivore.energy / 2;
                carnivore.energy -= offspring_energy;
                new_carnivores.push(Agent::new(self.next_agent_id, AgentType::Carnivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                carnivores_reproduction_count += 1;
            }
            // Check if the carnivore dies.
            if carnivore.energy <= 1 {
                carnivore.energy = 0;
                carnivores_died_count += 1;
            } else {
                updated_carnivores.push(carnivore);
            }
        }
        updated_carnivores.extend(new_carnivores);
        self.carnivores = updated_carnivores;

        let herbivores_final = self.herbivores.len();
        let carnivores_final = self.carnivores.len();
        let plants_final = self.plants.len();

        // Calculate energy statistics for herbivores.
        let (herb_energy_min, herb_energy_max, herb_energy_avg) = if !self.herbivores.is_empty() {
            let min = self.herbivores.iter().map(|h| h.energy).min().unwrap();
            let max = self.herbivores.iter().map(|h| h.energy).max().unwrap();
            let sum: i32 = self.herbivores.iter().map(|h| h.energy).sum();
            let avg = sum as f32 / self.herbivores.len() as f32;
            (Some(min), Some(max), Some(avg))
        } else {
            (None, None, None)
        };

        // Calculate energy statistics for carnivores.
        let (carn_energy_min, carn_energy_max, carn_energy_avg) = if !self.carnivores.is_empty() {
            let min = self.carnivores.iter().map(|c| c.energy).min().unwrap();
            let max = self.carnivores.iter().map(|c| c.energy).max().unwrap();
            let sum: i32 = self.carnivores.iter().map(|c| c.energy).sum();
            let avg = sum as f32 / self.carnivores.len() as f32;
            (Some(min), Some(max), Some(avg))
        } else {
            (None, None, None)
        };

        IterationStats {
            herbivores_initial,
            herbivores_final,
            carnivores_initial,
            carnivores_final,
            plants_initial,
            plants_final,
            herbivores_eaten_count,
            carnivores_eaten_count,
            herbivores_reproduction_count,
            carnivores_reproduction_count,
            herbivores_died_count,
            carnivores_died_count,
            herbivores_energy_min: herb_energy_min,
            herbivores_energy_max: herb_energy_max,
            herbivores_energy_avg: herb_energy_avg,
            carnivores_energy_min: carn_energy_min,
            carnivores_energy_max: carn_energy_max,
            carnivores_energy_avg: carn_energy_avg,
        }
    }
    
    #[allow(dead_code)]
    /// Draws a simple text-based representation of the ecosystem grid to the console.
    pub fn draw(&self) {
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for plant in &self.plants {
            grid[plant.y][plant.x] = 'P';
        }
        for herbivore in &self.herbivores {
            grid[herbivore.y][herbivore.x] = 'H';
        }
        for carnivore in &self.carnivores {
            grid[carnivore.y][carnivore.x] = 'C';
        }
        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}
