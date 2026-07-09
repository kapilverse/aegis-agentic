use crate::types::AgentInfo;
use std::collections::HashMap;

pub struct Supervisor {
    agents: HashMap<uuid::Uuid, AgentInfo>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register_agent(&mut self, agent: AgentInfo) {
        self.agents.insert(agent.id, agent);
    }

    pub fn find_agent_for_task(&self, required_capability: &str) -> Option<&AgentInfo> {
        self.agents
            .values()
            .find(|a| a.capabilities.contains(&required_capability.to_string()))
    }

    pub fn list_agents(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    pub fn remove_agent(&mut self, id: uuid::Uuid) -> bool {
        self.agents.remove(&id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_find() {
        let mut supervisor = Supervisor::new();
        let agent = AgentInfo {
            id: uuid::Uuid::new_v4(),
            name: "researcher".into(),
            capabilities: vec!["search".into(), "analyze".into()],
        };
        supervisor.register_agent(agent);
        assert!(supervisor.find_agent_for_task("search").is_some());
        assert!(supervisor.find_agent_for_task("coding").is_none());
    }
}
