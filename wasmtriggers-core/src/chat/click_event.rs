pub enum ClickEvent {
    RunCommand(String),
    SuggestCommand(String),
    OpenUrl(String),
    Copy(String),
}

impl ClickEvent {
    pub const fn text(&self) -> &String {
        match self {
            ClickEvent::RunCommand(text) => text,
            ClickEvent::SuggestCommand(text) => text,
            ClickEvent::OpenUrl(text) => text,
            ClickEvent::Copy(text) => text,
        }
    }
    pub const fn action_id(&self) -> u32 {
        match self {
            ClickEvent::RunCommand(_) => 1,
            ClickEvent::SuggestCommand(_) => 2,
            ClickEvent::OpenUrl(_) => 3,
            ClickEvent::Copy(_) => 4,
        }
    }
}
