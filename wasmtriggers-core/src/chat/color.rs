pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub const WHITE: Color = Color::new(255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0);
    pub const CYAN: Color = Color::new(0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255);
    pub const GRAY: Color = Color::new(128, 128, 128);
    pub const ORANGE: Color = Color::new(255, 165, 0);
    pub const PINK: Color = Color::new(255, 192, 203);
    pub const PURPLE: Color = Color::new(128, 0, 128);
    pub const DARK_RED: Color = Color::new(139, 0, 0);
    pub const DARK_GREEN: Color = Color::new(0, 100, 0);
    pub const DARK_BLUE: Color = Color::new(0, 0, 139);
    pub const GOLD: Color = Color::new(255, 215, 0);
    pub const SILVER: Color = Color::new(192, 192, 192);
    pub const BROWN: Color = Color::new(165, 42, 42);
    pub const NAVY: Color = Color::new(0, 0, 128);
    pub const TEAL: Color = Color::new(0, 128, 128);
}
