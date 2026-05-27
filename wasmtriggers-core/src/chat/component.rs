use crate::chat::{click_event::ClickEvent, color::Color, message::ChatMessage};

pub struct ChatComponent {
    pub text: (String, Color),
    pub hover: Option<(String, Color)>,
    pub on_click: Option<ClickEvent>,
}

impl ChatComponent {
    pub fn hover(mut self, hover: impl Into<String>, color: impl Into<Color>) -> Self {
        self.hover = Some((hover.into(), color.into()));
        self
    }
    pub fn click(mut self, on_click: ClickEvent) -> Self {
        self.on_click = Some(on_click.into());
        self
    }
}
impl Into<ChatMessage> for ChatComponent {
    fn into(self) -> ChatMessage {
        ChatMessage::empty().add(self)
    }
}

pub fn literal(msg: impl Into<String>, color: Color) -> ChatComponent {
    ChatComponent {
        text: (msg.into(), color),
        hover: None,
        on_click: None,
    }
}
