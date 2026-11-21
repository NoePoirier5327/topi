/// Type générique des couleurs dans topi.
pub type Color = sdl2::pixels::Color;

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
