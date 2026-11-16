//! Fichier contenant l'implémentation de l'affichage du ligne.

use sdl2::pixels::Color;
use sdl2::rect::FPoint;
use sdl2::render::{Vertex, WindowCanvas};
use super::Renderable;

/// Ligne affichable via topi.
pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    thickness: u32,
    color: Color
}

impl Line {
    /// Construit une nouvelle ligne affichable par topi.
    ///
    /// # Arguments
    /// * `x1` coordonnée en x du premier point de la nouvelle ligne.
    /// * `y1` coordonnée en y du premier point de la nouvelle ligne.
    /// * `x2` coordonnée en x du second point de la nouvelle ligne.
    /// * `y2` coordonnée en y du second point de la nouvelle ligne.
    /// * `thickness` largeur totale de la nouvelle ligne.
    /// * `color` couleur de la nouvelle ligne à afficher.
    ///
    /// # Return
    /// Instance vers la nouvelle ligne.
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32, thickness: u32, color: Color) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            thickness,
            color
        }
    }
}

/// Implémentation de l'interface de rendue générique pour topi.
impl Renderable for Line {
    fn display(&self, canvas: &mut WindowCanvas) {
        let dx = (self.x2 - self.x1) as f32;
        let dy = (self.y2 - self.y1) as f32;
        let length = (dx*dx + dy*dy).sqrt();

        if length == 0f32 { return }

        let half_thickness = (self.thickness) as f32 * 0.5f32;
        let ox = -(dy / length) * half_thickness;
        let oy = (dx / length)  * half_thickness;

        let vertices = [
            Vertex {
                position: FPoint::new((self.x1) as f32 + ox, (self.y1) as f32 + oy),
                tex_coord: FPoint::new(0f32, 0f32),
                color: self.color
            },

            Vertex {
                position: FPoint::new((self.x1) as f32 - ox, (self.y1) as f32 - oy),
                tex_coord: FPoint::new(0f32, 0f32),
                color: self.color
            },

            Vertex {
                position: FPoint::new((self.x2) as f32 - ox, (self.y2) as f32 - oy),
                tex_coord: FPoint::new(0f32, 0f32),
                color: self.color
            },

            Vertex {
                position: FPoint::new((self.x2) as f32 + ox, (self.y2) as f32 + oy),
                tex_coord: FPoint::new(0f32, 0f32),
                color: self.color
            }
        ];

        let indices = [0, 1, 2, 0, 2, 3];
        canvas.render_geometry(&vertices, None, &indices)
            .expect("Failed to render the required line.");
    }
}
