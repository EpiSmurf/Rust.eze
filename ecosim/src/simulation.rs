// src/simulation.rs

use crate::config::SIMULATION_STEPS;
use crate::ecosystem::Ecosystem;

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
        for step in 0..SIMULATION_STEPS {
            println!("=== Étape {} ===", step);
            self.ecosystem.draw();
            self.ecosystem.step();

            println!("\nAppuyez sur Entrée pour passer à l'étape suivante...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            // Effacement de l'écran (séquence ANSI — à tester selon votre terminal Windows)
            print!("\x1B[2J\x1B[1;1H");
        }
    }
}
