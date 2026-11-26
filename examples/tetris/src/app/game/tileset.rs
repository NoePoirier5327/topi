//! Fichier contenant le tileset de la tilemap du jeu.

use std::collections::HashMap;
use std::sync::LazyLock;
use super::map::Tile;
use topi::types::color::Color;

/// Couleur de l'intérieur des tetrominos en fonction de leurs types.
pub static TILES_COLOR: LazyLock<HashMap<Option<Tile>, Color>> = LazyLock::new(|| {
    HashMap::from([
        (None,          Color::RGB(18, 32, 32)),
        (Some(Tile::I), Color::RGB(32, 214, 199)),
        (Some(Tile::O), Color::RGB(255, 213, 65)),
        (Some(Tile::T), Color::RGB(121, 58, 128)),
        (Some(Tile::J), Color::RGB(20, 52, 100)),
        (Some(Tile::L), Color::RGB(250, 106, 10)),
        (Some(Tile::S), Color::RGB(26, 122, 62)),
        (Some(Tile::Z), Color::RGB(115, 23, 45))
    ])
});

/// Couleur des bordures des tetrominos en fonction de leurs types.
pub static TILES_OUTLINE_COLOR: LazyLock<HashMap<Option<Tile>, Color>> = LazyLock::new(|| {
    HashMap::from([
        (None,          Color::RGB(51, 57, 65)),
        (Some(Tile::I), Color::RGB(166, 252, 219)),
        (Some(Tile::O), Color::RGB(255, 252, 64)),
        (Some(Tile::T), Color::RGB(188, 74, 155)),
        (Some(Tile::J), Color::RGB(40, 92, 196)),
        (Some(Tile::L), Color::RGB(249, 163, 27)),
        (Some(Tile::S), Color::RGB(89, 193, 53)),
        (Some(Tile::Z), Color::RGB(180, 32, 42))
    ])
});

pub const TILES_WIDTH: u32 = 32;
pub const TILES_HEIGHT: u32 = 32;
pub const TILES_OUTLINE_THICKNESS: u32 = 4;
