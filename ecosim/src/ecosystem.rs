// src/ecosystem.rs

use crate::config::*;
use crate::species::{Agent, AgentType};
use rand::Rng;

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

impl IterationStats {
    /// Affiche les statistiques de l'itération, en affichant à côté (entre parenthèses) les totaux cumulés.
    pub fn print(&self, total_eaten: usize, total_reproduction: usize, total_died: usize) {
        println!("\x1B[1m--- Statistiques de l'itération ---\x1B[0m");
        println!(
            "\x1B[1mPlantes:\x1B[0m initial: {}, final: {} (diff: {})",
            self.plants_initial,
            self.plants_final,
            self.plants_final as isize - self.plants_initial as isize
        );
        println!(
            "\x1B[1mHerbivores:\x1B[0m initial: {}, final: {} (diff: {})",
            self.herbivores_initial,
            self.herbivores_final,
            self.herbivores_final as isize - self.herbivores_initial as isize
        );
        println!(
            "\x1B[1mPlantes mangées:\x1B[0m {} (total: {})",
            self.eaten_count, total_eaten
        );
        println!(
            "\x1B[1mReproductions:\x1B[0m {} (total: {})",
            self.reproduction_count, total_reproduction
        );
        println!(
            "\x1B[1mHerbivores décédés:\x1B[0m {} (total: {})",
            self.died_count, total_died
        );
        if let (Some(min), Some(max), Some(avg)) = (self.energy_min, self.energy_max, self.energy_avg)
        {
            println!(
                "\x1B[1mÉnergie des herbivores:\x1B[0m min: {}, max: {}, moyenne: {:.2}",
                min, max, avg
            );
        } else {
            println!("Aucun herbivore survivant.");
        }
        println!("\x1B[1m-----------------------------------\x1B[0m");
    }
}

pub struct Ecosystem {
    pub width: usize,
    pub height: usize,
    pub plants: Vec<Agent>,
    pub herbivores: Vec<Agent>,
}

impl Ecosystem {
    /// Initialise l'écosystème en plaçant aléatoirement les plantes et herbivores.
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut plants = Vec::new();
        let mut herbivores = Vec::new();

        // Placement aléatoire des plantes
        for _ in 0..INITIAL_PLANTS {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            plants.push(Agent::new(AgentType::Plant, x, y, 0));
        }

        // Placement aléatoire des herbivores avec énergie initiale
        for _ in 0..INITIAL_HERBIVORES {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            herbivores.push(Agent::new(AgentType::Herbivore, x, y, HERBIVORE_INITIAL_ENERGY));
        }

        Ecosystem {
            width,
            height,
            plants,
            herbivores,
        }
    }

    /// Fonction auxiliaire ne nécessitant pas d'emprunter `self` pour calculer des coordonnées adjacentes.
    fn random_adjacent_aux(
        rng: &mut impl Rng,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> (usize, usize) {
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

    /// Recherche les indices des plantes présentes en (x, y)
    fn find_plants_at(&self, x: usize, y: usize) -> Vec<usize> {
        self.plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| plant.x == x && plant.y == y)
            .map(|(i, _)| i)
            .collect()
    }

    /// Effectue une itération de la simulation et retourne des statistiques sur l'itération.
    pub fn step(&mut self) -> IterationStats {
        let mut rng = rand::thread_rng();

        // Statistiques initiaux
        let plants_initial = self.plants.len();
        let herbivores_initial = self.herbivores.len();

        // 1. Reproduction des plantes (limitation à 1 plante par cellule)
        let mut new_plants: Vec<Agent> = Vec::new();
        for plant in &self.plants {
            if rng.gen::<f32>() < PLANT_GROWTH_RATE {
                // Essai de reproduction dans une cellule adjacente
                let (new_x, new_y) =
                    Self::random_adjacent_aux(&mut rng, plant.x, plant.y, self.width, self.height);
                if self.find_plants_at(new_x, new_y).is_empty()
                    && !new_plants.iter().any(|p: &Agent| p.x == new_x && p.y == new_y)
                {
                    new_plants.push(Agent::new(AgentType::Plant, new_x, new_y, 0));
                }
            }
        }
        self.plants.extend(new_plants);

        // 2. Actions des herbivores
        let mut eaten_count = 0;
        let mut reproduction_count = 0;
        let mut died_count = 0;
        let width = self.width;
        let height = self.height;

        // Retirer la liste des herbivores pour la traiter sans emprunter `self`
        let current_herbivores = std::mem::take(&mut self.herbivores);
        let mut updated_herbivores = Vec::new();
        let mut new_herbivores = Vec::new();

        for mut herbivore in current_herbivores {
            // Déplacement avec une probabilité de 80%
            if rng.gen::<f32>() < 0.8 {
                let (new_x, new_y) =
                    Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, width, height);
                herbivore.x = new_x;
                herbivore.y = new_y;
            }
            // Perte d'énergie due au déplacement
            herbivore.energy -= HERBIVORE_ENERGY_LOSS;

            // S'il y a une plante dans la cellule, l'herbivore la mange
            let plant_indices = self.find_plants_at(herbivore.x, herbivore.y);
            if !plant_indices.is_empty() {
                let index = plant_indices[0];
                self.plants.swap_remove(index);
                herbivore.energy += HERBIVORE_ENERGY_GAIN;
                eaten_count += 1;
            }

            // Reproduction : si l'énergie est suffisante et avec une certaine probabilité
            if herbivore.energy >= HERBIVORE_REPRODUCTION_THRESHOLD
                && rng.gen::<f32>() < HERBIVORE_REPRODUCTION_RATE
            {
                let (offspring_x, offspring_y) =
                    Self::random_adjacent_aux(&mut rng, herbivore.x, herbivore.y, width, height);
                let offspring_energy = herbivore.energy / 2;
                herbivore.energy -= offspring_energy;
                new_herbivores.push(Agent::new(
                    AgentType::Herbivore,
                    offspring_x,
                    offspring_y,
                    offspring_energy,
                ));
                reproduction_count += 1;
            }

            // L'herbivore survit tant que son énergie reste positive
            if herbivore.energy > 0 {
                updated_herbivores.push(herbivore);
            } else {
                died_count += 1;
            }
        }
        // Ajout des nouveaux herbivores issus de la reproduction
        updated_herbivores.extend(new_herbivores);
        self.herbivores = updated_herbivores;

        let herbivores_final = self.herbivores.len();
        let plants_final = self.plants.len();

        // Calcul des statistiques d'énergie des herbivores
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

    /// Affiche l'état de l'écosystème sous forme de grille dans la console.
    ///
    /// - 'H' pour une cellule contenant au moins un herbivore
    /// - 'P' pour une cellule contenant au moins une plante (et aucun herbivore)
    /// - '.' pour une cellule vide
    pub fn draw(&self) {
        // Création d'une grille vide
        let mut grid = vec![vec!['.'; self.width]; self.height];

        // On place d'abord les symboles pour les plantes
        for plant in &self.plants {
            grid[plant.y][plant.x] = 'P';
        }

        // Puis, on écrase (si nécessaire) avec le symbole 'H' en cas de présence d'herbivores
        for herbivore in &self.herbivores {
            grid[herbivore.y][herbivore.x] = 'H';
        }

        // Affichage de la grille
        for row in grid {
            let line: String = row.into_iter().collect();
            println!("{}", line);
        }
    }
}
