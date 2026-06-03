use crate::chat::{click_event::ClickEvent, color::Color, message::ChatMessage};

pub struct ChatComponent {
    pub text: (String, Color),
    pub hover: Option<(String, Color)>,
    pub on_click: Option<ClickEvent>,
}

impl ChatComponent {
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.text.1 = color.into();
        self
    }
    pub fn hover(mut self, hover: impl Into<String>) -> Self {
        self.hover = Some((hover.into(), Color::new(255, 255, 255)));
        self
    }
    pub fn hover_color(mut self, color: impl Into<Color>) -> Self {
        if let Some(hover) = self.hover.as_mut() {
            hover.1 = color.into();
        }
        self
    }
    pub fn click(mut self, on_click: impl Into<ClickEvent>) -> Self {
        self.on_click = Some(on_click.into());
        self
    }
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: ChatComponent) -> ChatMessage {
        let message: ChatMessage = self.into();
        message.add(other)
    }
}
impl From<ChatComponent> for ChatMessage {
    fn from(item: ChatComponent) -> ChatMessage {
        let mut temp = ChatMessage::empty();
        temp.components.push(item);
        temp
    }
}

pub fn literal(msg: impl Into<String>) -> ChatComponent {
    ChatComponent {
        text: (msg.into(), Color::new(255, 255, 255)),
        hover: None,
        on_click: None,
    }
}
