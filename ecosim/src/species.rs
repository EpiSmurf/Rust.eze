// src/species.rs

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AgentType {
    Plant,
    Herbivore,
    // D'autres types pourront être ajoutés ultérieurement.
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Agent {
    pub agent_type: AgentType,
    pub x: usize,
    pub y: usize,
    /// Pour les herbivores : énergie restante. Pour les plantes, cette valeur n'est pas utilisée.
    pub energy: i32,
}

impl Agent {
    pub fn new(agent_type: AgentType, x: usize, y: usize, energy: i32) -> Self {
        Agent {
            agent_type,
            x,
            y,
            energy,
        }
    }
}
