use crate::types::AgentMessage;
use tokio::sync::mpsc;

pub struct MessageBus {
    sender: mpsc::UnboundedSender<AgentMessage>,
    receiver: mpsc::UnboundedReceiver<AgentMessage>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self { sender, receiver }
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<AgentMessage> {
        self.sender.clone()
    }

    pub async fn receive(&mut self) -> Option<AgentMessage> {
        self.receiver.recv().await
    }
}
