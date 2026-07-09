use crate::types::{Message, Role, Session};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SessionManager {
    sessions: HashMap<Uuid, Session>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, tenant_id: String, agent_id: Uuid) -> Session {
        let session = Session::new(tenant_id, agent_id);
        self.sessions.insert(session.id, session.clone());
        session
    }

    pub fn get_session(&self, id: Uuid) -> Option<&Session> {
        self.sessions.get(&id)
    }

    pub fn list_sessions(&self, tenant_id: &str) -> Vec<&Session> {
        self.sessions
            .values()
            .filter(|s| s.tenant_id == tenant_id)
            .collect()
    }

    pub fn add_message(
        &mut self,
        session_id: Uuid,
        role: Role,
        content: String,
    ) -> Result<Message, String> {
        let session = self
            .sessions
            .get_mut(&session_id)
            .ok_or("Session not found")?;

        let token_count = content.len() / 4;
        let message = Message {
            id: Uuid::new_v4(),
            role,
            content,
            tool_calls: None,
            token_count,
            timestamp: chrono::Utc::now(),
        };

        session.messages.push(message.clone());
        session.updated_at = chrono::Utc::now();

        Ok(message)
    }

    pub fn get_messages(&self, session_id: Uuid) -> Result<Vec<&Message>, String> {
        let session = self
            .sessions
            .get(&session_id)
            .ok_or("Session not found")?;
        Ok(session.messages.iter().collect())
    }

    pub fn delete_session(&mut self, id: Uuid) -> bool {
        self.sessions.remove(&id).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let mut manager = SessionManager::new();
        let session = manager.create_session("org1".into(), Uuid::new_v4());
        assert_eq!(session.tenant_id, "org1");
    }

    #[test]
    fn test_tenant_isolation() {
        let mut manager = SessionManager::new();
        let agent_id = Uuid::new_v4();
        manager.create_session("org1".into(), agent_id);
        manager.create_session("org2".into(), agent_id);
        manager.create_session("org1".into(), agent_id);

        let org1_sessions = manager.list_sessions("org1");
        let org2_sessions = manager.list_sessions("org2");
        assert_eq!(org1_sessions.len(), 2);
        assert_eq!(org2_sessions.len(), 1);
    }

    #[test]
    fn test_add_message() {
        let mut manager = SessionManager::new();
        let session = manager.create_session("org1".into(), Uuid::new_v4());
        let msg = manager
            .add_message(session.id, Role::User, "Hello".into())
            .unwrap();
        assert_eq!(msg.role, Role::User);
    }
}
