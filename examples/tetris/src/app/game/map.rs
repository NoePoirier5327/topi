//! Fichier contenant la manipulation de la tilemap du jeu.

use super::tetromino::Tetromino;

pub const MAP_WIDTH: usize = 10usize;
pub const MAP_HEIGHT: usize = 20usize;

pub const MAP_DISPLAY_X_OFFSET: i32 = 200;

/// Représente les tuiles manipulables par la tilemap du module.
/// Si on veut représenter une tuile vide, on met un None.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    I, O, T, J, L, S, Z
}

/// Tilemap contenant l'état des tetrominos posés dans la logique du jeu.
pub struct Map {
    content: [Option<Tile>; MAP_WIDTH * MAP_HEIGHT]
}

impl Map {
    /// Construit une nouvelle carte vide.
    #[inline]
    pub fn new() -> Self {
        Self { content: [None; MAP_WIDTH * MAP_HEIGHT] }
    }

    /// Accesseur des tuiles de la carte, l'appelant doit vérifier que les coordonnées renvoient
    /// bien vers une tuile accessible en mémoire.
    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        debug_assert!(x < MAP_WIDTH && y < MAP_HEIGHT, "Tilemap content index out of range");
        self.content[x + MAP_WIDTH * y]
    }

    /// Mutateur des tuiles de la carte, l'appelant doit vérifier que les coordonnées renvoient bien
    /// vers une tuile accessible en mémoire.
    #[inline]
    fn set(&mut self, x: usize, y: usize, tile: Option<Tile>) {
        debug_assert!(x < MAP_WIDTH && y < MAP_HEIGHT, "Tilemap content index out of range.");
        self.content[x + MAP_WIDTH * y] = tile;
    }

    /// Insert le tetromino en paramètre dans la carte courante.
    pub fn insert_tetro(&mut self, tetromino: &Tetromino) {
        let tetro_size = tetromino.get_size();
        for x in 0..tetro_size {
            for y in 0..tetro_size {
                if tetromino.get(x, y).is_none() { continue }
                self.set((tetromino.get_pos().get_x() + x as i32) as usize, (tetromino.get_pos().get_y() - y as i32) as usize, tetromino.get(x, y));
            }
        }
    }

    /// Détermine l'index de la première ligne pleine détectée en partant du bas de la carte.
    /// Si None, aucune ligne détectée, sinon, index de la ligne détectée.
    pub fn get_full_line_index(&self) -> Option<usize> {
        let mut index = None;

        let mut line = 0;
        while line < MAP_HEIGHT && index.is_none() {
            let mut line_is_full = true;

            let mut column = 0;
            while column < MAP_WIDTH && line_is_full {
                line_is_full &= self.get(column, line).is_some();
                column += 1;
            }

            if line_is_full {
                index = Some(line);
            }

            line += 1;
        }

        index
    }

    /// Détruis la ligne en paramètre, la ligne est accessible par son index, son index doit être
    /// dans la carte sinon une erreur est levée.
    pub fn destroy_line(&mut self, line: usize) {
        debug_assert!(line < MAP_HEIGHT, "The line to destroy should be in the map.");

        // On vide la ligne ciblée par l'index en paramètre.
        for column in 0..MAP_WIDTH {
            self.set(column, line, None);
        }
        
        // On la déplace en haut de la carte.
        for line2 in (line + 1)..MAP_HEIGHT {
            self.exchange_lines(line2 - 1, line2);
        }
    }

    /// Echange les deux lignes accessible par les indexs en paramètre.
    /// line1 < MAP_HEIGHT && line2 < MAP_WIDTH
    fn exchange_lines(&mut self, line1: usize, line2: usize) {
        debug_assert!(line1 < MAP_HEIGHT, "Line1 index out of map range.");
        debug_assert!(line2 < MAP_HEIGHT, "Line2 index out of map range.");

        for column in 0..MAP_WIDTH {
            let temp = self.get(column, line1);
            self.set(column, line1, self.get(column, line2));
            self.set(column, line2, temp);
        }
    }
}
