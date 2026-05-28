use crate::chat::component::ChatComponent;

pub struct ChatMessage {
    pub components: Vec<ChatComponent>,
}

impl ChatMessage {
    pub const fn empty() -> Self {
        ChatMessage { components: vec![] }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, other: impl Into<ChatMessage>) -> Self {
        for component in other.into().components {
            self.components.push(component);
        }
        self
    }
}
