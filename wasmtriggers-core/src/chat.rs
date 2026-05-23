#[derive(Debug)]
pub enum ChatType<'a> {
    Game { message: &'a str },
    Player { player: &'a str, message: &'a str },
}
