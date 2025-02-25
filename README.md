# EcoSim Simulation Project

This project simulates an ecosystem with various agents—plants, herbivores, and carnivores—using Rust and Macroquad for visualization. It provides an interactive graphical interface for configuring simulation parameters and observing the simulation in real time.

## Prerequisites

- **Rust and Cargo:** Make sure you have the latest stable Rust toolchain installed. You can download it from [rustup.rs](https://rustup.rs/).
- **Dependencies:** The project uses the `macroquad` and `rand` crates. These will be automatically downloaded when you compile the project.

## Compilation

To compile the project, open a terminal in the project directory and run:

```bash
cargo build
```

You can also run the project directly in debug mode with:

```bash
cargo run
```

For an optimized build, use:

```bash
cargo run --release
```

## Running the Simulation

When you run the project, a simulation window will open with a configuration menu. Use the following controls:

### Configuration Menu

- **Arrow Keys (Up/Down):** Navigate through configuration fields.
- **Numeric Keys and `.`:** Modify field values.
- **Backspace:** Delete the last character.
- **Enter:** Confirm settings and start the simulation.
- **Escape:** Quit the program.

### During Simulation

- **Left/Right Arrow Keys:** Step backward/forward in the simulation history.
- **Spacebar:** Continuously update the simulation.
- **Left Mouse Click:** Select and track an agent on the grid.
- **Escape:** Exit the simulation.

## Project Structure

- **config.rs:** Defines simulation configuration parameters and agent types.
- **ecosystem.rs:** Implements the ecosystem simulation logic, including agent interactions, simulation steps, and statistics.
- **main.rs:** Initializes the Macroquad window, handles the configuration menu, user input, simulation state updates, and rendering.