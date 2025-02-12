// src/main.rs

mod config;
mod species;
mod ecosystem;
mod simulation;

fn main() {
    use config::{GRID_WIDTH, GRID_HEIGHT};

    let mut sim = simulation::Simulation::new(GRID_WIDTH, GRID_HEIGHT);
    sim.run();
}
