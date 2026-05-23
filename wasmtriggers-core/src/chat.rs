#[derive(Debug)]
pub enum ChatType<'a> {
    Game { message: &'a str },
    Player { player: &'a str, message: &'a str },
}

impl<'a> ChatType<'a> {
    pub fn get_message(&self) -> &'a str {
        match self {
            Self::Game { message } => message,
            ChatType::Player { player: _, message } => message,
        }
    }
}
