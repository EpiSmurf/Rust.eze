use crate::config::SimulationConfig;
use crate::species::{Agent, AgentType};
use rand::Rng;

#[allow(dead_code)]
pub struct IterationStats {
    pub herbivores_initial: usize,
    pub herbivores_final: usize,
    pub plants_initial: usize,
    pub plants_final: usize,
    pub eaten_count: usize,
    pub reproduction_count: usize,
    pub died_count: usize,
    pub energy_min: Option<i32>,
    pub energy_max: Option<i32>,
    pub energy_avg: Option<f32>,
}

#[allow(dead_code)]
impl IterationStats {
    pub fn print(&self, total_eaten: usize, total_reproduction: usize, total_died: usize) {
        println!(
            "Iteration Stats: Eaten: {} (total: {}), Reproduction: {} (total: {}), Deaths: {} (total: {})",
            self.eaten_count, total_eaten, self.reproduction_count, total_reproduction, self.died_count, total_died
        );
    }
}

#[derive(Clone)]
pub struct Ecosystem {
    pub width: usize,
    pub height: usize,
    pub plants: Vec<Agent>,
    pub herbivores: Vec<Agent>,
    pub config: SimulationConfig,
    pub next_agent_id: u32,
}

impl Ecosystem {
    pub fn new_custom(config: SimulationConfig) -> Self {
        let width = config.grid_width;
        let height = config.grid_height;
        let mut rng = rand::thread_rng();
        let mut plants = Vec::new();
        let mut herbivores = Vec::new();
        let mut next_agent_id: u32 = 0;

        // Placement aléatoire des plantes
        for _ in 0..config.initial_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::Plant, x, y, 0));
            next_agent_id += 1;
        }

        // Placement aléatoire des herbivores
        for _ in 0..config.initial_herbivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            herbivores.push(Agent::new(
                next_agent_id,
                AgentType::Herbivore,
                x,
                y,
                config.herbivore_initial_energy,
            ));
            next_agent_id += 1;
        }

        Ecosystem {
            width,
            height,
            plants,
            herbivores,
            config,
            next_agent_id,
        }
    }

    fn find_plants_at(&self, x: usize, y: usize) -> Vec<usize> {
        self.plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| plant.x == x && plant.y == y)
            .map(|(i, _)| i)
            .collect()
    }

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

    pub fn step(&mut self) -> IterationStats {
        let mut rng = rand::thread_rng();
        let plants_initial = self.plants.len();
        let herbivores_initial = self.herbivores.len();

        // Cloner la liste des plantes pour éviter les conflits d'emprunt
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

        let mut eaten_count = 0;
        let mut reproduction_count = 0;
        let mut died_count = 0;
        let current_herbivores = std::mem::take(&mut self.herbivores);
        let mut updated_herbivores = Vec::new();
        let mut new_herbivores = Vec::new();
        for mut herb in current_herbivores {
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, herb.x, herb.y, self.width, self.height);
                herb.x = nx;
                herb.y = ny;
            }
            herb.energy -= self.config.herbivore_energy_loss;
            let plant_indices = self.find_plants_at(herb.x, herb.y);
            if !plant_indices.is_empty() {
                let index = plant_indices[0];
                self.plants.swap_remove(index);
                herb.energy += self.config.herbivore_energy_gain;
                eaten_count += 1;
            }
            if herb.energy >= self.config.herbivore_reproduction_threshold
                && rng.gen::<f32>() < self.config.herbivore_reproduction_rate
            {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, herb.x, herb.y, self.width, self.height);
                let offspring_energy = herb.energy / 2;
                herb.energy -= offspring_energy;
                new_herbivores.push(Agent::new(self.next_agent_id, AgentType::Herbivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                reproduction_count += 1;
            }
            // Considérer l'herbivore mort si son énergie <= 1 et fixer son énergie à 0.
            if herb.energy <= 1 {
                herb.energy = 0;
                died_count += 1;
            } else {
                updated_herbivores.push(herb);
            }
        }
        updated_herbivores.extend(new_herbivores);
        self.herbivores = updated_herbivores;
        let herbivores_final = self.herbivores.len();
        let plants_final = self.plants.len();
        let (energy_min, energy_max, energy_avg) = if !self.herbivores.is_empty() {
            let min = self.herbivores.iter().map(|h| h.energy).min().unwrap();
            let max = self.herbivores.iter().map(|h| h.energy).max().unwrap();
            let sum: i32 = self.herbivores.iter().map(|h| h.energy).sum();
            let avg = sum as f32 / self.herbivores.len() as f32;
            (Some(min), Some(max), Some(avg))
        } else {
            (None, None, None)
        };
        IterationStats {
            herbivores_initial,
            herbivores_final,
            plants_initial,
            plants_final,
            eaten_count,
            reproduction_count,
            died_count,
            energy_min,
            energy_max,
            energy_avg,
        }
    }
    
    #[allow(dead_code)]
    pub fn draw(&self) {
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for plant in &self.plants {
            grid[plant.y][plant.x] = 'P';
        }
        for herb in &self.herbivores {
            grid[herb.y][herb.x] = 'H';
        }
        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}
