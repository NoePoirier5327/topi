//! Fichier contenant les structures de données régissant l'affichage des rectangles dans le moteur.

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use super::Renderable;
use super::line;

/// Affiche un rectangle plein d'une certaine couleur.
pub struct FilledRectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    color: Color
}

impl FilledRectangle {
    /// Construit un nouveau rectangle plein.
    ///
    /// # Arguments
    /// * `x` coordonnée en x du nouveau rectangle.
    /// * `y` coordonnée en y du nouveau rectangle.
    /// * `width` largeur du nouveau rectangle.
    /// * `height` hauteur du nouveau rectangle.
    /// * `color` couleur de remplissage du nouveau rectangle.
    ///
    /// # Return
    /// Instance vers le nouveau rectangle plein.
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color
        }
    }
}

/// Implémentation de l'interface de rendue générique du moteur.
impl Renderable for FilledRectangle {
    fn display(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(self.x, self.y, self.width, self.height))
            .expect("Failed to render the filled rectangle.");
    }
}

/// Affiche un rectangle non plein d'une certaine couleur.
pub struct Rectangle {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    line_thickness: u32,
    color: Color
}

impl Rectangle {
    /// Construit un nouveau rectangle non plein.
    ///
    /// # Arguments
    /// * `x` coordonnée en x du nouveau rectangle.
    /// * `y` coordonnée en y du nouveau rectangle.
    /// * `width` largeur du nouveau rectangle.
    /// * `height` hauteur du nouveau rectangle.
    /// * `line_thickness` épaisseur de la ligne du nouveau rectangle.
    /// * `color` couleur de rendue du nouveau rectangle.
    ///
    /// # Return
    /// Instance vers le nouveau rectangle plein.
    pub fn new(x: i32, y: i32, width: u32, height: u32, line_thickness: u32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            line_thickness,
            color
        }
    }
}

/// Interface générique pour l'affichage.
impl Renderable for Rectangle {
    fn display(&self, canvas: &mut WindowCanvas) {
        let half_line_thickness = (self.line_thickness / 2) as i32;

        line::Line::new(
            self.x - half_line_thickness,
            self.y,
            self.x + self.width as i32 + half_line_thickness,
            self.y,
            self.line_thickness,
            self.color
        ).display(canvas);

        line::Line::new(
            self.x,
            self.y,
            self.x,
            self.y + self.height as i32,
            self.line_thickness,
            self.color
        ).display(canvas);

        line::Line::new(
            self.x - half_line_thickness,
            self.y + self.height as i32,
            self.x + self.width as i32 + half_line_thickness,
            self.y + self.height as i32,
            self.line_thickness,
            self.color
        ).display(canvas);

        line::Line::new(
            self.x + self.width as i32,
            self.y,
            self.x + self.width as i32,
            self.y + self.height as i32,
            self.line_thickness,
            self.color
        ).display(canvas);
    }
}
