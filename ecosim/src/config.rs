// src/config.rs

pub const GRID_WIDTH: usize = 40;
pub const GRID_HEIGHT: usize = 20;
pub const INITIAL_PLANTS: usize = 50;
pub const INITIAL_HERBIVORES: usize = 10;
pub const PLANT_GROWTH_RATE: f32 = 0.2; // Probabilité pour une plante de produire une nouvelle plante
pub const HERBIVORE_REPRODUCTION_RATE: f32 = 0.1; // Probabilité de reproduction si l'énergie est suffisante
pub const HERBIVORE_ENERGY_GAIN: i32 = 5;         // Gain d'énergie lorsqu'un herbivore mange une plante
pub const HERBIVORE_ENERGY_LOSS: i32 = 1;         // Perte d'énergie à chaque déplacement
pub const HERBIVORE_INITIAL_ENERGY: i32 = 10;
pub const HERBIVORE_REPRODUCTION_THRESHOLD: i32 = 15; // Seuil d'énergie pour tenter de se reproduire
