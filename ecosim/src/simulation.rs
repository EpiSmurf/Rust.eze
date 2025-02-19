// src/simulation.rs

use crate::ecosystem::{Ecosystem, IterationStats};

pub struct Simulation {
    pub ecosystem: Ecosystem,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Simulation {
            ecosystem: Ecosystem::new(width, height),
        }
    }

    pub fn run(&mut self) {
        let mut step = 0;
        let mut total_eaten = 0;
        let mut total_reproduction = 0;
        let mut total_died = 0;
        loop {
            // Titre de l'itération en gras
            println!("\x1B[1m=== Étape {} ===\x1B[0m", step);
            self.ecosystem.draw();

            // Espacement entre la grille et les statistiques
            println!("\n");

            let stats: IterationStats = self.ecosystem.step();

            // Mise à jour des totaux cumulés
            total_eaten += stats.eaten_count;
            total_reproduction += stats.reproduction_count;
            total_died += stats.died_count;

            // Affichage des statistiques de l'itération avec les totaux
            stats.print(total_eaten, total_reproduction, total_died);

            println!("\nAppuyez sur Entrée pour passer à l'étape suivante ou tapez 'q' pour quitter...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().eq_ignore_ascii_case("q") {
                break;
            }

            // Effacement de l'écran (séquence ANSI – à tester selon votre terminal Windows)
            print!("\x1B[2J\x1B[1;1H");

            step += 1;
        }
    }
}
