use crate::chat::component::ChatComponent;

pub struct ChatMessage {
    pub components: Vec<ChatComponent>,
}

impl ChatMessage {
    pub fn empty() -> Self {
        ChatMessage { components: vec![] }
    }
    pub fn add(mut self, other: ChatComponent) -> Self {
        self.components.push(other);
        self
    }
    pub fn build() {}
}
