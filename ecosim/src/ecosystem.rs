// ecosystem.rs
use crate::config::{SimulationConfig, Agent, AgentType};
use rand::Rng;

#[derive(Default, Clone)]
pub struct SimulationStats {
    pub plant_births: usize,
    pub herbivore_births: usize,
    pub carnivore_births: usize,
    pub omnivore_births: usize,
    pub water_births: usize,
    pub tree_births: usize,
    pub plant_deaths: usize,
    pub herbivore_deaths: usize,
    pub carnivore_deaths: usize,
    pub omnivore_deaths: usize,
    pub water_deaths: usize,
    pub tree_deaths: usize,
    pub herbivore_consumptions: usize,
    pub carnivore_consumptions: usize,
    pub omnivore_consumptions_plants: usize,
    pub omnivore_consumptions_herbivores: usize,
}

#[derive(Clone)]
pub struct Ecosystem {
    pub width: usize,
    pub height: usize,
    pub plants: Vec<Agent>,
    pub herbivores: Vec<Agent>,
    pub carnivores: Vec<Agent>,
    pub omnivores: Vec<Agent>,
    pub waters: Vec<Agent>,
    pub trees: Vec<Agent>,
    pub config: SimulationConfig,
    pub next_agent_id: u32,
    pub iteration_count: usize,
}

impl Ecosystem {
    pub fn new_custom(config: SimulationConfig) -> Self {
        let width = config.grid_width;
        let height = config.grid_height;
        let mut rng = rand::thread_rng();
        let mut plants = Vec::new();
        let mut herbivores = Vec::new();
        let mut carnivores = Vec::new();
        let mut omnivores = Vec::new();
        let waters = Vec::new();
        let trees = Vec::new();
        let mut next_agent_id: u32 = 0;
        for _ in 0..config.initial_light_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::LightPlant, x, y, 0));
            next_agent_id += 1;
        }
        for _ in 0..config.initial_dark_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::DarkPlant, x, y, 0));
            next_agent_id += 1;
        }
        for _ in 0..config.initial_herbivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            herbivores.push(Agent::new(next_agent_id, AgentType::Herbivore, x, y, config.herbivore_initial_energy));
            next_agent_id += 1;
        }
        for _ in 0..config.initial_carnivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            carnivores.push(Agent::new(next_agent_id, AgentType::Carnivore, x, y, config.carnivore_initial_energy));
            next_agent_id += 1;
        }
        for _ in 0..config.initial_omnivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            omnivores.push(Agent::new(next_agent_id, AgentType::Omnivore, x, y, config.omnivore_initial_energy));
            next_agent_id += 1;
        }
        Ecosystem {
            width,
            height,
            plants,
            herbivores,
            carnivores,
            omnivores,
            waters,
            trees,
            config,
            next_agent_id,
            iteration_count: 0,
        }
    }

    fn random_adjacent_aux(rng: &mut impl Rng, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        let dx: i32 = rng.gen_range(-1..=1);
        let dy: i32 = rng.gen_range(-1..=1);
        let new_x = if dx < 0 { x.saturating_sub(dx.abs() as usize) } else { std::cmp::min(x + dx as usize, width - 1) };
        let new_y = if dy < 0 { y.saturating_sub(dy.abs() as usize) } else { std::cmp::min(y + dy as usize, height - 1) };
        (new_x, new_y)
    }

    fn maybe_spawn_water(&mut self, stats: &mut SimulationStats) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.config.water_spawn_chance {
            let x = rng.gen_range(1..(self.width - 1));
            let y = rng.gen_range(1..(self.height - 1));
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let wx = (x as i32 + dx) as usize;
                    let wy = (y as i32 + dy) as usize;
                    self.plants.retain(|p| !(p.x == wx && p.y == wy));
                    self.herbivores.retain(|h| !(h.x == wx && h.y == wy));
                    self.carnivores.retain(|c| !(c.x == wx && c.y == wy));
                    self.omnivores.retain(|o| !(o.x == wx && o.y == wy));
                    self.trees.retain(|t| !(t.x == wx && t.y == wy));
                    let water = Agent::new_water(self.next_agent_id, wx, wy, self.iteration_count);
                    self.next_agent_id += 1;
                    self.waters.push(water);
                    stats.water_births += 1;
                }
            }
        }
    }

    fn evaporate_water(&mut self, stats: &mut SimulationStats) {
        let current_it = self.iteration_count;
        let before = self.waters.len();
        self.waters.retain(|w| {
            if let Some(birth) = w.birth_iteration {
                (current_it - birth) < self.config.water_lifespan
            } else {
                true
            }
        });
        let after = self.waters.len();
        stats.water_deaths += before - after;
    }

    fn handle_water_influence(&mut self, stats: &mut SimulationStats) {
        let mut rng = rand::thread_rng();
        for w in &self.waters {
            let w_x = w.x as i32;
            let w_y = w.y as i32;
            for dx in -5..=5 {
                for dy in -5..=5 {
                    let nx = w_x + dx;
                    let ny = w_y + dy;
                    if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                        continue;
                    }
                    let ux = nx as usize;
                    let uy = ny as usize;
                    let before = self.plants.len();
                    self.plants.retain(|p| !(p.x == ux && p.y == uy && p.agent_type == AgentType::DarkPlant));
                    let after = self.plants.len();
                    stats.plant_deaths += before - after;
                    if rng.gen::<f32>() < (self.config.plant_growth_rate * 3.0) {
                        let no_plant = !self.plants.iter().any(|p| p.x == ux && p.y == uy);
                        let no_water = !self.waters.iter().any(|wa| wa.x == ux && wa.y == uy);
                        if no_plant && no_water {
                            let new_l = Agent::new(self.next_agent_id, AgentType::LightPlant, ux, uy, 0);
                            self.next_agent_id += 1;
                            self.plants.push(new_l);
                            stats.plant_births += 1;
                        }
                    }
                }
            }
        }
    }

    fn maybe_spawn_tree(&mut self, stats: &mut SimulationStats) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.config.tree_spawn_chance {
            let x = rng.gen_range(0..(self.width - 1));
            let y = rng.gen_range(0..(self.height - 1));
            for dx in 0..2 {
                for dy in 0..2 {
                    let tx = x + dx;
                    let ty = y + dy;
                    self.plants.retain(|p| !(p.x == tx && p.y == ty));
                    self.herbivores.retain(|h| !(h.x == tx && h.y == ty));
                    self.carnivores.retain(|c| !(c.x == tx && c.y == ty));
                    self.omnivores.retain(|o| !(o.x == tx && o.y == ty));
                    self.waters.retain(|w| !(w.x == tx && w.y == ty));
                    let tree = Agent::new_tree(self.next_agent_id, tx, ty, self.iteration_count);
                    self.next_agent_id += 1;
                    self.trees.push(tree);
                    stats.tree_births += 1;
                }
            }
        }
    }

    fn evaporate_trees(&mut self, stats: &mut SimulationStats) {
        let current_it = self.iteration_count;
        let before = self.trees.len();
        self.trees.retain(|t| {
            if let Some(birth) = t.birth_iteration {
                (current_it - birth) < self.config.tree_lifespan
            } else {
                true
            }
        });
        let after = self.trees.len();
        stats.tree_deaths += before - after;
    }

    fn handle_tree_influence(&mut self, stats: &mut SimulationStats) {
        let mut rng = rand::thread_rng();
        for t in &self.trees {
            let t_x = t.x as i32;
            let t_y = t.y as i32;
            for dx in -5..=5 {
                for dy in -5..=5 {
                    let nx = t_x + dx;
                    let ny = t_y + dy;
                    if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                        continue;
                    }
                    let ux = nx as usize;
                    let uy = ny as usize;
                    let before = self.plants.len();
                    self.plants.retain(|p| !(p.x == ux && p.y == uy && p.agent_type == AgentType::LightPlant));
                    let after = self.plants.len();
                    stats.plant_deaths += before - after;
                    if rng.gen::<f32>() < 0.5 {
                        let no_plant = !self.plants.iter().any(|p| p.x == ux && p.y == uy);
                        let no_water = !self.waters.iter().any(|w| w.x == ux && w.y == uy);
                        let no_tree = !self.trees.iter().any(|tt| tt.x == ux && tt.y == uy);
                        if no_plant && no_water && no_tree {
                            let dplant = Agent::new(self.next_agent_id, AgentType::DarkPlant, ux, uy, 0);
                            self.next_agent_id += 1;
                            self.plants.push(dplant);
                            stats.plant_births += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn step(&mut self, stats: &mut SimulationStats) {
        self.iteration_count += 1;
        self.maybe_spawn_water(stats);
        self.evaporate_water(stats);
        self.handle_water_influence(stats);
        self.maybe_spawn_tree(stats);
        self.evaporate_trees(stats);
        self.handle_tree_influence(stats);
        let mut rng = rand::thread_rng();
        let plants_snapshot = self.plants.clone();
        let mut new_plants = Vec::new();
        for _plant in &plants_snapshot {
            if rng.gen::<f32>() < self.config.plant_growth_rate {
                let nx = rng.gen_range(0..self.width);
                let ny = rng.gen_range(0..self.height);
                if self.waters.iter().any(|w| w.x == nx && w.y == ny) || self.trees.iter().any(|t| t.x == nx && t.y == ny) {
                    continue;
                }
                if let Some(existing_index) = self.plants.iter().position(|p| p.x == nx && p.y == ny) {
                    let new_type = match self.plants[existing_index].agent_type {
                        AgentType::LightPlant => AgentType::DarkPlant,
                        AgentType::DarkPlant => AgentType::LightPlant,
                        _ => continue,
                    };
                    let old_id = self.plants[existing_index].id;
                    self.plants[existing_index] = Agent::new(old_id, new_type, nx, ny, 0);
                } else {
                    new_plants.push(Agent::new(self.next_agent_id, if rng.gen::<f32>() < 0.5 { AgentType::LightPlant } else { AgentType::DarkPlant }, nx, ny, 0));
                    self.next_agent_id += 1;
                    stats.plant_births += 1;
                }
            }
        }
        self.plants.extend(new_plants);
        let current_herbivores = std::mem::take(&mut self.herbivores);
        let mut updated_herbivores = Vec::new();
        let mut new_herbivores = Vec::new();
        for mut herbivore in current_herbivores {
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, self.width, self.height);
                herbivore.x = nx;
                herbivore.y = ny;
            }
            herbivore.energy -= self.config.herbivore_energy_loss;
            if self.waters.iter().any(|w| w.x == herbivore.x && w.y == herbivore.y)
                || self.trees.iter().any(|t| t.x == herbivore.x && t.y == herbivore.y) {
                herbivore.energy = 0;
                herbivore.pending_death = true;
                herbivore.death_cause = Some("Overridden by Water/Tree".to_string());
            } else if let Some(index) = self.plants.iter().position(|p| p.x == herbivore.x && p.y == herbivore.y) {
                self.plants.swap_remove(index);
                herbivore.energy += self.config.herbivore_energy_gain;
                stats.herbivore_consumptions += 1;
                stats.plant_deaths += 1;
            }
            if herbivore.energy >= self.config.herbivore_reproduction_threshold && rng.gen::<f32>() < self.config.herbivore_reproduction_rate {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, self.width, self.height);
                let offspring_energy = herbivore.energy / 2;
                herbivore.energy -= offspring_energy;
                new_herbivores.push(Agent::new(self.next_agent_id, AgentType::Herbivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                stats.herbivore_births += 1;
            }
            if herbivore.energy <= 0 {
                if !herbivore.pending_death {
                    herbivore.pending_death = true;
                    herbivore.death_cause = Some("Lack of Energy".to_string());
                    stats.herbivore_deaths += 1;
                    updated_herbivores.push(herbivore);
                }
            } else {
                herbivore.pending_death = false;
                herbivore.death_cause = None;
                updated_herbivores.push(herbivore);
            }
        }
        updated_herbivores.extend(new_herbivores);
        self.herbivores = updated_herbivores;
        let current_carnivores = std::mem::take(&mut self.carnivores);
        let mut updated_carnivores = Vec::new();
        let mut new_carnivores = Vec::new();
        for mut carnivore in current_carnivores {
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, carnivore.x, carnivore.y, self.width, self.height);
                carnivore.x = nx;
                carnivore.y = ny;
            }
            carnivore.energy -= self.config.carnivore_energy_loss;
            if self.waters.iter().any(|w| w.x == carnivore.x && w.y == carnivore.y)
                || self.trees.iter().any(|t| t.x == carnivore.x && t.y == carnivore.y) {
                carnivore.energy = 0;
                carnivore.pending_death = true;
                carnivore.death_cause = Some("Overridden by Water/Tree".to_string());
            } else if let Some(index) = self.herbivores.iter().position(|h| h.x == carnivore.x && h.y == carnivore.y) {
                let mut prey = self.herbivores.swap_remove(index);
                prey.energy = 0;
                prey.pending_death = true;
                prey.death_cause = Some("Eaten by Carnivore".to_string());
                self.herbivores.push(prey);
                carnivore.energy += self.config.carnivore_energy_gain;
                stats.carnivore_consumptions += 1;
                stats.herbivore_deaths += 1;
            }
            if carnivore.energy >= self.config.carnivore_reproduction_threshold && rng.gen::<f32>() < self.config.carnivore_reproduction_rate {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, carnivore.x, carnivore.y, self.width, self.height);
                let offspring_energy = carnivore.energy / 2;
                carnivore.energy -= offspring_energy;
                new_carnivores.push(Agent::new(self.next_agent_id, AgentType::Carnivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                stats.carnivore_births += 1;
            }
            if carnivore.energy <= 0 {
                if !carnivore.pending_death {
                    carnivore.pending_death = true;
                    carnivore.death_cause = Some("Lack of Energy".to_string());
                    stats.carnivore_deaths += 1;
                    updated_carnivores.push(carnivore);
                }
            } else {
                carnivore.pending_death = false;
                carnivore.death_cause = None;
                updated_carnivores.push(carnivore);
            }
        }
        updated_carnivores.extend(new_carnivores);
        self.carnivores = updated_carnivores;
        let current_omnivores = std::mem::take(&mut self.omnivores);
        let mut updated_omnivores = Vec::new();
        let mut new_omnivores = Vec::new();
        for mut omnivore in current_omnivores {
            if rng.gen::<f32>() < 0.8 {
                let (nx, ny) = Self::random_adjacent_aux(&mut rng, omnivore.x, omnivore.y, self.width, self.height);
                omnivore.x = nx;
                omnivore.y = ny;
            }
            omnivore.energy -= self.config.omnivore_energy_loss;
            if self.waters.iter().any(|w| w.x == omnivore.x && w.y == omnivore.y)
                || self.trees.iter().any(|t| t.x == omnivore.x && t.y == omnivore.y) {
                omnivore.energy = 0;
                omnivore.pending_death = true;
                omnivore.death_cause = Some("Overridden by Water/Tree".to_string());
            } else {
                if let Some(index) = self.herbivores.iter().position(|h| h.x == omnivore.x && h.y == omnivore.y) {
                    let mut prey = self.herbivores.swap_remove(index);
                    prey.energy = 0;
                    prey.pending_death = true;
                    prey.death_cause = Some("Eaten by Omnivore".to_string());
                    self.herbivores.push(prey);
                    omnivore.energy += self.config.omnivore_energy_gain_herbivores;
                    stats.omnivore_consumptions_herbivores += 1;
                    stats.herbivore_deaths += 1;
                } else if let Some(index) = self.plants.iter().position(|p| p.x == omnivore.x && p.y == omnivore.y) {
                    self.plants.swap_remove(index);
                    omnivore.energy += self.config.omnivore_energy_gain_plants;
                    stats.omnivore_consumptions_plants += 1;
                    stats.plant_deaths += 1;
                }
                if omnivore.energy >= self.config.omnivore_reproduction_threshold && rng.gen::<f32>() < self.config.omnivore_reproduction_rate {
                    let (ox, oy) = Self::random_adjacent_aux(&mut rng, omnivore.x, omnivore.y, self.width, self.height);
                    let offspring_energy = omnivore.energy / 2;
                    omnivore.energy -= offspring_energy;
                    new_omnivores.push(Agent::new(self.next_agent_id, AgentType::Omnivore, ox, oy, offspring_energy));
                    self.next_agent_id += 1;
                    stats.omnivore_births += 1;
                }
            }
            if omnivore.energy <= 0 {
                if !omnivore.pending_death {
                    omnivore.pending_death = true;
                    omnivore.death_cause = Some("Lack of Energy".to_string());
                    stats.omnivore_deaths += 1;
                    updated_omnivores.push(omnivore);
                }
            } else {
                omnivore.pending_death = false;
                omnivore.death_cause = None;
                updated_omnivores.push(omnivore);
            }
        }
        updated_omnivores.extend(new_omnivores);
        self.omnivores = updated_omnivores;
        let before_trees = self.trees.len();
        let mut trees_died_count = 0;
        self.trees.retain(|t| {
            if let Some(birth) = t.birth_iteration {
                if (self.iteration_count - birth) >= self.config.tree_lifespan {
                    trees_died_count += 1;
                    false
                } else {
                    true
                }
            } else {
                true
            }
        });
        let after_trees = self.trees.len();
        stats.tree_deaths += before_trees - after_trees;
    }
}
