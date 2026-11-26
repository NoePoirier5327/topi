//! Fichier contenant la manipulation des tetrominos (pièces du jeu).

use topi::types::point::Point2i;
use super::map::Tile;
use super::map::{MAP_WIDTH, MAP_HEIGHT};

/// Détermine le type du tetromino courant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum TetrominoKind {
    I, O, T, J, L, S, Z
}

impl TryFrom<usize> for TetrominoKind {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::I),
            1 => Ok(Self::O),
            2 => Ok(Self::T),
            3 => Ok(Self::J),
            4 => Ok(Self::L),
            5 => Ok(Self::S),
            6 => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

/// Formes initiale des tetrominos.
const TETROMINOS: [Option<Tile>; 65usize] = [
    // I
    None, Some(Tile::I), None, None,
    None, Some(Tile::I), None, None,
    None, Some(Tile::I), None, None,
    None, Some(Tile::I), None, None,

    // O
    Some(Tile::O), Some(Tile::O),
    Some(Tile::O), Some(Tile::O),

    // T
    Some(Tile::T), Some(Tile::T), Some(Tile::T),
    None, Some(Tile::T), None,
    None, None, None,

    // J
    None, None, Some(Tile::J),
    None, None, Some(Tile::J),
    None, Some(Tile::J), Some(Tile::J),

    // L
    Some(Tile::L), None, None,
    Some(Tile::L), None, None,
    Some(Tile::L), Some(Tile::L), None,

    // S
    None, Some(Tile::S), Some(Tile::S),
    Some(Tile::S), Some(Tile::S), None,
    None, None, None,

    // Z
    Some(Tile::Z), Some(Tile::Z), None,
    None, Some(Tile::Z), Some(Tile::Z),
    None, None, None
];

/// Tailles des tetrominos ciblés à la création.
const TETROMINOS_SIZE: [usize; 7usize] = [4, 2, 3, 3, 3, 3, 3];

// Offset d'accès aux tetrominos dans `TETROMINOS`.
const OFFSETS: [usize; 7usize] = [0, 16, 20, 29, 38, 47, 56];

/// Tetromino manipulé par `Game`.
#[derive(Copy, Clone)]
pub struct Tetromino {
    state: [Option<Tile>; 16usize],
    size: usize,
    pos: Point2i,
    kind: TetrominoKind
}

impl Tetromino {
    /// Construit un nouveau tetromino pour le jeu à partir d'un dont on a passé le type en
    /// paramètre.
    ///
    /// # Argument
    /// * `kind` type du tetromino à allouer.
    ///
    /// # Return
    /// Instance vers le nouveau tetromino correspondant au type passé en paramètre.
    pub fn new(kind: TetrominoKind) -> Self {
        // On charge les métadonnées pour charger le tetromino cible.
        let index = kind as usize;
        let size = TETROMINOS_SIZE[index];
        let offset = OFFSETS[index];

        // On charge la cible en mémoire.
        let mut state = [None; 16usize];
        for x in 0..size {
            for y in 0..size {
                state[y * size + x] = TETROMINOS[y * size + x + offset];
            }
        }

        // On détermine la position d'apparition.
        let pos = Point2i::new(
            if size.is_multiple_of(2) { ((MAP_WIDTH as f32 - size as f32) / 2f32) as i32 } else {(MAP_WIDTH as f32 / 2f32) as i32 - 1i32},
            MAP_HEIGHT as i32 - 1i32
        );

        Self {
            state,
            size,
            pos,
            kind
        }
    }

    /// Fait tomber le tetromino courant sur la carte courante, avant de faire tomber le tetromino,
    /// il faut vérifier s'il peut tomber.
    #[inline]
    pub fn fall(&mut self) {
        self.pos.set_y(self.pos.get_y() - 1);
    }

    /// Fait tourner le tetromino courant, l'appelant doit vérifier qu'il puisse tourner avant
    /// d'appeler la méthode.
    pub fn rotate(&mut self) {
        let mut temp = [None; 16usize];
        
        for x in 0..self.size {
            for y in 0..self.size {
                temp[x + self.size * y] = self.state[y + self.size * (self.size - x - 1)];
            }
        }

        for x in 0..self.size {
            for y in 0..self.size {
                self.state[x + self.size * y] = temp[x + self.size * y];
            }
        }
    }

    /// Fait se déplacer le tetromino vers la droite, l'appelant doit vérifier que l'action est
    /// possible avant de la réaliser.
    #[inline]
    pub fn move_right(&mut self) {
        self.pos.set_x(self.pos.get_x() - 1);
    }

    /// Fait se déplacer le tetromino vers la gauche, l'appelant doit vérifier que l'action est
    /// possible avant de la réaliser.
    #[inline]
    pub fn move_left(&mut self) {
        self.pos.set_x(self.pos.get_x() + 1);
    }

    /// Accesseur de la position courante du tetromino.
    #[inline]
    pub fn get_pos(&self) -> Point2i {
        self.pos
    }

    /// Accesseur de la taille courante du tetromino.
    #[inline]
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Accesseur du type du tetromino courant.
    #[inline]
    pub fn get_kind(&self) -> TetrominoKind {
        self.kind
    }

    /// Accesseur sur les tuiles contenues dans le champ `state` dans l'instance courante.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        debug_assert!(x < self.size && y < self.size, "Tetromino content index out of range.");
        self.state[x + self.size * y]
    }
}
