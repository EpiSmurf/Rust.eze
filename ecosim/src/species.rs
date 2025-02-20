#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AgentType {
    Plant,
    Herbivore,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: u32,
    pub agent_type: AgentType,
    pub x: usize,
    pub y: usize,
    pub energy: i32,
}

impl Agent {
    pub fn new(id: u32, agent_type: AgentType, x: usize, y: usize, energy: i32) -> Self {
        Agent { id, agent_type, x, y, energy }
    }
}
