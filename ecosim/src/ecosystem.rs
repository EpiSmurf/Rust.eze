/*
    Ecosystem Simulation Module

    Ce module définit la structure Ecosystem,
    qui représente la grille de simulation et
    contient les collections de plantes claires
    et foncées, d'herbivores, de carnivores,
    d'omnivores, de points d'eau et d'arbres.
    Il implémente aussi la logique d'une
    itération (step) et la collecte de stats.
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
    pub omnivores_initial: usize,
    pub omnivores_final: usize,
    pub trees_initial: usize,
    pub trees_final: usize,
    pub plants_initial: usize,
    pub plants_final: usize,

    pub herbivores_eaten_count: usize,
    pub carnivores_eaten_count: usize,
    pub omnivores_eaten_count: usize,

    pub herbivores_reproduction_count: usize,
    pub carnivores_reproduction_count: usize,
    pub omnivores_reproduction_count: usize,

    pub herbivores_died_count: usize,
    pub carnivores_died_count: usize,
    pub omnivores_died_count: usize,
    pub trees_died_count: usize,

    pub herbivores_energy_min: Option<i32>,
    pub herbivores_energy_max: Option<i32>,
    pub herbivores_energy_avg: Option<f32>,

    pub carnivores_energy_min: Option<i32>,
    pub carnivores_energy_max: Option<i32>,
    pub carnivores_energy_avg: Option<f32>,

    pub omnivores_energy_min: Option<i32>,
    pub omnivores_energy_max: Option<i32>,
    pub omnivores_energy_avg: Option<f32>,
}

#[allow(dead_code)]
impl IterationStats {
    /// Prints the iteration statistics in a formatted output.
    pub fn print(
        &self,
        total_herb_eaten: usize,
        total_carn_eaten: usize,
        total_omni_eaten: usize,
        total_herb_rep: usize,
        total_carn_rep: usize,
        total_omni_rep: usize,
        total_herb_died: usize,
        total_carn_died: usize,
        total_omni_died: usize,
        total_tree_died: usize,
    ) {
        println!(
            "Iteration Stats:\n\
             Plants: {} -> {},\n\
             Herbivores: {} -> {} (Eaten: {} total: {}),\n\
             Carnivores: {} -> {} (Eaten: {} total: {}),\n\
             Omnivores: {} -> {} (Eaten: {} total: {}),\n\
             Trees: {} -> {} (Died: {} total),\n\
             Reproduction: Herbivores: {} (total: {}), Carnivores: {} (total: {}), Omnivores: {} (total: {}),\n\
             Deaths: Herbivores: {} (total: {}), Carnivores: {} (total: {}), Omnivores: {} (total: {})",
            self.plants_initial,
            self.plants_final,
            self.herbivores_initial,
            self.herbivores_final,
            self.herbivores_eaten_count,
            total_herb_eaten,
            self.carnivores_initial,
            self.carnivores_final,
            self.carnivores_eaten_count,
            total_carn_eaten,
            self.omnivores_initial,
            self.omnivores_final,
            self.omnivores_eaten_count,
            total_omni_eaten,
            self.trees_initial,
            self.trees_final,
            total_tree_died,
            self.herbivores_reproduction_count,
            total_herb_rep,
            self.carnivores_reproduction_count,
            total_carn_rep,
            self.omnivores_reproduction_count,
            total_omni_rep,
            self.herbivores_died_count,
            total_herb_died,
            self.carnivores_died_count,
            total_carn_died,
            self.omnivores_died_count,
            total_omni_died
        );
    }
}

#[derive(Clone)]
/// Représente l’écosystème complet, y compris la grille et les listes d’agents.
pub struct Ecosystem {
    /// Largeur de la grille de simulation.
    pub width: usize,
    /// Hauteur de la grille de simulation.
    pub height: usize,
    /// Vecteur contenant toutes les plantes (claires et foncées).
    pub plants: Vec<Agent>,
    /// Vecteur contenant les herbivores.
    pub herbivores: Vec<Agent>,
    /// Vecteur contenant les carnivores.
    pub carnivores: Vec<Agent>,
    /// Vecteur contenant les omnivores.
    pub omnivores: Vec<Agent>,
    /// Vecteur contenant les patchs d'eau.
    pub waters: Vec<Agent>,
    /// Vecteur contenant les arbres.
    pub trees: Vec<Agent>,

    /// Paramètres de configuration de la simulation.
    pub config: SimulationConfig,
    /// Prochain identifiant d’agent unique.
    pub next_agent_id: u32,

    /// Nombre d’itérations (steps) déjà effectuées.
    pub iteration_count: usize,
}

impl Ecosystem {
    /// Crée un nouvel écosystème avec une config personnalisée.
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

        // Place initial light plants.
        for _ in 0..config.initial_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::LightPlant, x, y, 0));
            next_agent_id += 1;
        }
        // Place initial dark plants.
        for _ in 0..config.initial_dark_plants {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(next_agent_id, AgentType::DarkPlant, x, y, 0));
            next_agent_id += 1;
        }
        // Place initial herbivores.
        for _ in 0..config.initial_herbivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            herbivores.push(Agent::new(next_agent_id, AgentType::Herbivore, x, y, config.herbivore_initial_energy));
            next_agent_id += 1;
        }
        // Place initial carnivores.
        for _ in 0..config.initial_carnivores {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            carnivores.push(Agent::new(next_agent_id, AgentType::Carnivore, x, y, config.carnivore_initial_energy));
            next_agent_id += 1;
        }
        // Place initial omnivores.
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

    /// Retourne une coordonnée voisine aléatoire (y compris la cellule courante).
    fn random_adjacent_aux(rng: &mut impl Rng, x: usize, y: usize, width: usize, height: usize) -> (usize, usize) {
        let dx: i32 = rng.gen_range(-1..=1);
        let dy: i32 = rng.gen_range(-1..=1);
        let new_x = if dx < 0 { x.saturating_sub(dx.abs() as usize) } else { std::cmp::min(x + dx as usize, width - 1) };
        let new_y = if dy < 0 { y.saturating_sub(dy.abs() as usize) } else { std::cmp::min(y + dy as usize, height - 1) };
        (new_x, new_y)
    }

    /// Possibilité de faire apparaître un patch d'eau (3x3).
    fn maybe_spawn_water(&mut self) {
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
                }
            }
        }
    }

    /// Fait disparaître l'eau trop ancienne.
    fn evaporate_water(&mut self) {
        let current_it = self.iteration_count;
        self.waters.retain(|w| {
            if let Some(birth) = w.birth_iteration {
                (current_it - birth) < self.config.water_lifespan
            } else {
                true
            }
        });
    }

    /// Autour de l'eau (rayon 5), enlève les DarkPlants et favorise l’apparition de LightPlant.
    fn handle_water_influence(&mut self) {
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
                    self.plants.retain(|p| !(p.x == ux && p.y == uy && p.agent_type == AgentType::DarkPlant));
                    if rng.gen::<f32>() < (self.config.plant_growth_rate * 3.0) {
                        let no_plant = !self.plants.iter().any(|p| p.x == ux && p.y == uy);
                        let no_water = !self.waters.iter().any(|wa| wa.x == ux && wa.y == uy);
                        if no_plant && no_water {
                            let new_l = Agent::new(self.next_agent_id, AgentType::LightPlant, ux, uy, 0);
                            self.next_agent_id += 1;
                            self.plants.push(new_l);
                        }
                    }
                }
            }
        }
    }

    /// Possibilité de faire apparaître un patch d’arbres (2x2).
    fn maybe_spawn_tree(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < self.config.tree_spawn_chance {
            // On vérifie qu'on a la place pour 2x2
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
                }
            }
        }
    }

    /// Fait disparaître les arbres trop vieux.
    fn evaporate_trees(&mut self) {
        let current_it = self.iteration_count;
        self.trees.retain(|t| {
            if let Some(birth) = t.birth_iteration {
                (current_it - birth) < self.config.tree_lifespan
            } else {
                true
            }
        });
    }

    /// Dans un rayon de 5 autour de chaque arbre, retire les LightPlants
    /// et provoque une "énorme prolifération" de DarkPlants.
    fn handle_tree_influence(&mut self) {
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
                    // Supprime LightPlant
                    self.plants.retain(|p| !(p.x == ux && p.y == uy && p.agent_type == AgentType::LightPlant));

                    // "Énorme prolifération" de DarkPlant (ex: 50% de chance)
                    if rng.gen::<f32>() < 0.5 {
                        let no_plant = !self.plants.iter().any(|p| p.x == ux && p.y == uy);
                        let no_water = !self.waters.iter().any(|w| w.x == ux && w.y == uy);
                        let no_tree = !self.trees.iter().any(|tt| tt.x == ux && tt.y == uy);
                        if no_plant && no_water && no_tree {
                            let dplant = Agent::new(self.next_agent_id, AgentType::DarkPlant, ux, uy, 0);
                            self.next_agent_id += 1;
                            self.plants.push(dplant);
                        }
                    }
                }
            }
        }
    }

    /// Avance la simulation d'un pas et renvoie les statistiques d'itération.
    pub fn step(&mut self) -> IterationStats {
        self.iteration_count += 1;

        // Eau
        self.maybe_spawn_water();
        self.evaporate_water();
        self.handle_water_influence();

        // Arbres
        self.maybe_spawn_tree();
        self.evaporate_trees();
        self.handle_tree_influence();

        let mut rng = rand::thread_rng();
        let plants_initial = self.plants.len();
        let herbivores_initial = self.herbivores.len();
        let carnivores_initial = self.carnivores.len();
        let omnivores_initial = self.omnivores.len();
        // Pour éviter le warning, on marque cette variable comme volontairement inutilisée :
        let _trees_initial = self.trees.len();

        // Croissance des plantes
        let plants_snapshot = self.plants.clone();
        let mut new_plants = Vec::new();
        for _plant in &plants_snapshot {
            if rng.gen::<f32>() < self.config.plant_growth_rate {
                let nx = rng.gen_range(0..self.width);
                let ny = rng.gen_range(0..self.height);
                // Pas de plante si eau ou arbre
                if self.waters.iter().any(|w| w.x == nx && w.y == ny) || self.trees.iter().any(|t| t.x == nx && t.y == ny) {
                    continue;
                }
                // Si déjà une plante, on la remplace par l’autre type (Light <-> Dark)
                if let Some(existing_index) = self.plants.iter().position(|p| p.x == nx && p.y == ny) {
                    let new_type = match self.plants[existing_index].agent_type {
                        AgentType::LightPlant => AgentType::DarkPlant,
                        AgentType::DarkPlant => AgentType::LightPlant,
                        _ => continue,
                    };
                    let old_id = self.plants[existing_index].id;
                    self.plants[existing_index] = Agent::new(old_id, new_type, nx, ny, 0);
                } else {
                    let plant_type = if rng.gen::<f32>() < 0.5 {
                        AgentType::LightPlant
                    } else {
                        AgentType::DarkPlant
                    };
                    new_plants.push(Agent::new(self.next_agent_id, plant_type, nx, ny, 0));
                    self.next_agent_id += 1;
                }
            }
        }
        self.plants.extend(new_plants);

        // Herbivores
        let mut herbivores_eaten_count = 0;
        let mut herbivores_reproduction_count = 0;
        let mut herbivores_died_count = 0;
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
            }
            else if let Some(index) = self.plants.iter().position(|p| p.x == herbivore.x && p.y == herbivore.y) {
                self.plants.swap_remove(index);
                herbivore.energy += self.config.herbivore_energy_gain;
                herbivores_eaten_count += 1;
            }
            if herbivore.energy >= self.config.herbivore_reproduction_threshold && rng.gen::<f32>() < self.config.herbivore_reproduction_rate {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, self.width, self.height);
                let offspring_energy = herbivore.energy / 2;
                herbivore.energy -= offspring_energy;
                new_herbivores.push(Agent::new(self.next_agent_id, AgentType::Herbivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                herbivores_reproduction_count += 1;
            }
            if herbivore.energy <= 0 {
                if !herbivore.pending_death {
                    herbivore.pending_death = true;
                    herbivore.death_cause = Some("Lack of Energy".to_string());
                    updated_herbivores.push(herbivore);
                } else {
                    herbivores_died_count += 1;
                }
            } else {
                herbivore.pending_death = false;
                herbivore.death_cause = None;
                updated_herbivores.push(herbivore);
            }
        }
        updated_herbivores.extend(new_herbivores);
        self.herbivores = updated_herbivores;

        // Carnivores
        let mut carnivores_eaten_count = 0;
        let mut carnivores_reproduction_count = 0;
        let mut carnivores_died_count = 0;
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
            }
            else if let Some(index) = self.herbivores.iter().position(|h| h.x == carnivore.x && h.y == carnivore.y) {
                let mut prey = self.herbivores.swap_remove(index);
                prey.energy = 0;
                prey.pending_death = true;
                prey.death_cause = Some("Eaten by Carnivore".to_string());
                self.herbivores.push(prey);
                carnivore.energy += self.config.carnivore_energy_gain;
                carnivores_eaten_count += 1;
            }
            if carnivore.energy >= self.config.carnivore_reproduction_threshold && rng.gen::<f32>() < self.config.carnivore_reproduction_rate {
                let (ox, oy) = Self::random_adjacent_aux(&mut rng, carnivore.x, carnivore.y, self.width, self.height);
                let offspring_energy = carnivore.energy / 2;
                carnivore.energy -= offspring_energy;
                new_carnivores.push(Agent::new(self.next_agent_id, AgentType::Carnivore, ox, oy, offspring_energy));
                self.next_agent_id += 1;
                carnivores_reproduction_count += 1;
            }
            if carnivore.energy <= 0 {
                if !carnivore.pending_death {
                    carnivore.pending_death = true;
                    carnivore.death_cause = Some("Lack of Energy".to_string());
                    updated_carnivores.push(carnivore);
                } else {
                    carnivores_died_count += 1;
                }
            } else {
                carnivore.pending_death = false;
                carnivore.death_cause = None;
                updated_carnivores.push(carnivore);
            }
        }
        updated_carnivores.extend(new_carnivores);
        self.carnivores = updated_carnivores;

        // Omnivores
        let mut omnivores_eaten_count = 0;
        let mut omnivores_reproduction_count = 0;
        let mut omnivores_died_count = 0;
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
                    omnivores_eaten_count += 1;
                }
                else if let Some(index) = self.plants.iter().position(|p| p.x == omnivore.x && p.y == omnivore.y) {
                    self.plants.swap_remove(index);
                    omnivore.energy += self.config.omnivore_energy_gain_plants;
                    omnivores_eaten_count += 1;
                }
                if omnivore.energy >= self.config.omnivore_reproduction_threshold && rng.gen::<f32>() < self.config.omnivore_reproduction_rate {
                    let (ox, oy) = Self::random_adjacent_aux(&mut rng, omnivore.x, omnivore.y, self.width, self.height);
                    let offspring_energy = omnivore.energy / 2;
                    omnivore.energy -= offspring_energy;
                    new_omnivores.push(Agent::new(self.next_agent_id, AgentType::Omnivore, ox, oy, offspring_energy));
                    self.next_agent_id += 1;
                    omnivores_reproduction_count += 1;
                }
            }
            if omnivore.energy <= 0 {
                if !omnivore.pending_death {
                    omnivore.pending_death = true;
                    omnivore.death_cause = Some("Lack of Energy".to_string());
                    updated_omnivores.push(omnivore);
                } else {
                    omnivores_died_count += 1;
                }
            } else {
                omnivore.pending_death = false;
                omnivore.death_cause = None;
                updated_omnivores.push(omnivore);
            }
        }
        updated_omnivores.extend(new_omnivores);
        self.omnivores = updated_omnivores;

        // Comptage final des arbres (utilisé pour stats)
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

        let plants_final = self.plants.len();
        let herbivores_final = self.herbivores.len();
        let carnivores_final = self.carnivores.len();
        let omnivores_final = self.omnivores.len();
        let trees_final = self.trees.len();

        let herb_energy_min = self.herbivores.iter().map(|h| h.energy).min();
        let herb_energy_max = self.herbivores.iter().map(|h| h.energy).max();
        let herb_energy_avg = if !self.herbivores.is_empty() {
            Some(self.herbivores.iter().map(|h| h.energy).sum::<i32>() as f32 / self.herbivores.len() as f32)
        } else { None };

        let carn_energy_min = self.carnivores.iter().map(|c| c.energy).min();
        let carn_energy_max = self.carnivores.iter().map(|c| c.energy).max();
        let carn_energy_avg = if !self.carnivores.is_empty() {
            Some(self.carnivores.iter().map(|c| c.energy).sum::<i32>() as f32 / self.carnivores.len() as f32)
        } else { None };

        let omni_energy_min = self.omnivores.iter().map(|o| o.energy).min();
        let omni_energy_max = self.omnivores.iter().map(|o| o.energy).max();
        let omni_energy_avg = if !self.omnivores.is_empty() {
            Some(self.omnivores.iter().map(|o| o.energy).sum::<i32>() as f32 / self.omnivores.len() as f32)
        } else { None };

        IterationStats {
            herbivores_initial,
            herbivores_final,
            carnivores_initial,
            carnivores_final,
            omnivores_initial,
            omnivores_final,
            // On peut remettre la valeur initiale si souhaité dans le rapport final
            // Ici, on n'affiche pas _trees_initial mais on calcule plus bas trees_final
            trees_initial: _trees_initial,
            trees_final,
            plants_initial,
            plants_final,
            herbivores_eaten_count,
            carnivores_eaten_count,
            omnivores_eaten_count,
            herbivores_reproduction_count,
            carnivores_reproduction_count,
            omnivores_reproduction_count,
            herbivores_died_count,
            carnivores_died_count,
            omnivores_died_count,
            trees_died_count,
            herbivores_energy_min: herb_energy_min,
            herbivores_energy_max: herb_energy_max,
            herbivores_energy_avg: herb_energy_avg,
            carnivores_energy_min: carn_energy_min,
            carnivores_energy_max: carn_energy_max,
            carnivores_energy_avg: carn_energy_avg,
            omnivores_energy_min: omni_energy_min,
            omnivores_energy_max: omni_energy_max,
            omnivores_energy_avg: omni_energy_avg,
        }
    }

    #[allow(dead_code)]
    /// Dessine une représentation textuelle de la grille dans la console.
    pub fn draw(&self) {
        let mut grid = vec![vec!['.'; self.width]; self.height];
        for plant in &self.plants {
            match plant.agent_type {
                AgentType::LightPlant => grid[plant.y][plant.x] = 'L',
                AgentType::DarkPlant => grid[plant.y][plant.x] = 'D',
                _ => (),
            }
        }
        for herbivore in &self.herbivores {
            grid[herbivore.y][herbivore.x] = 'H';
        }
        for carnivore in &self.carnivores {
            grid[carnivore.y][carnivore.x] = 'C';
        }
        for omnivore in &self.omnivores {
            grid[omnivore.y][omnivore.x] = 'O';
        }
        for water in &self.waters {
            grid[water.y][water.x] = 'W';
        }
        for tree in &self.trees {
            grid[tree.y][tree.x] = 'T';
        }
        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }
    }
}
