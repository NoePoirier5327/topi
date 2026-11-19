pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
pub const ORANGE: Color = Color { r: 251, g: 108, b: 38, a: 255 };
pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };
pub const CYAN: Color = Color { r: 0, g: 255, b: 241, a: 255 };
pub const PINK: Color = Color { r: 232, g: 107, b: 255, a: 255 };
pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

/// Correspond à une couleur dans le moteur topi.
/// La couleur courante est au format RGBA.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    /// Construit une nouvelle couleur au format RGB.
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Construit une nouvelle couleur au format RGBA.
    pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Mutateur de la partie rouge de la couleur courante.
    #[inline]
    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    /// Mutateur de la partie verte de la couleur courante.
    #[inline]
    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    /// Mutateur de la parte bleu de la couleur courante.
    #[inline]
    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    /// Mutateur de la partie transparente de la couleur courante.
    #[inline]
    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    /// Accesseur de la partie rouge de la couleur courante.
    #[inline]
    pub fn get_r(&self) -> u8 {
        self.r
    }

    /// Accesseur de la partie verte de la couleur courante.
    #[inline]
    pub fn get_g(&self) -> u8 {
        self.g
    }

    /// Accesseur de la partie bleu de la couleur courante.
    #[inline]
    pub fn get_b(&self) -> u8 {
        self.b
    }

    /// Accesseur de la partie transparente de la couleur courante.
    #[inline]
    pub fn get_a(&self) -> u8 {
        self.a
    }
}
