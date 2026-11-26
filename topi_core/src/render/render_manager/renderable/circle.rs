//! Fichier contenant les structures régissant l'affichage de cercle par le moteur.

use std::cmp::max;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect;
use super::Renderable;

/// Affiche un cercle plein dans le moteur.
pub struct FilledCircle {
    x: i32,
    y: i32,
    radius: f64,
    color: Color
}

impl FilledCircle {
    /// Construit un nouveau cercle coloré plein.
    ///
    /// # Arguments
    /// * `x` coordonnée du centre du nouveau cercle.
    /// * `y` coordonnée du centre du nouveau cercle.
    /// * `radius` rayon du nouveau cercle.
    /// * `color` couleur du nouveau cercle.
    ///
    /// # Return
    /// Instance vers le nouveau cercle.
    pub fn new(x: i32, y: i32, radius: f64, color: Color) -> Self {
        assert!(radius >= 0f64, "Filled circle radius shouldn't be negative.");

        Self {
            x,
            y,
            radius,
            color
        }
    }
}

/// Interface générique d'affichage du moteur.
impl Renderable for FilledCircle {
    fn display(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        let radius = self.radius as i32;
        for dy in 1..=radius {
            let dy_f = dy as f64;
            let dx = ((2f64 * self.radius * dy_f) - (dy_f * dy_f)).sqrt().floor() as i32;

            let y_bottom = self.y + dy - radius;
            let y_top = self.y - dy + radius;

            canvas.draw_line(
                rect::Point::new(self.x - dx, y_bottom),
                rect::Point::new(self.x + dx, y_bottom)
            ).expect("Failed to renderer filled circle's line.");

            canvas.draw_line(
                rect::Point::new(self.x - dx, y_top),
                rect::Point::new(self.x + dx, y_top)
            ).expect("Failed to renderer filled circle's line.");
        }
    }
}

/// Affiche un cercle coloré non plein.
pub struct Circle {
    x: i32,
    y: i32,
    radius: f64,
    line_thickness: u32,
    color: Color
}

impl Circle {
    /// Construit un nouveau cercle coloré non plein affichable par topi.
    ///
    /// # Arguments
    /// * `x` coordonnée du centre du nouveau cercle.
    /// * `y` coordonnée du centre du nouveau cercle.
    /// * `radius` rayon du nouveau cercle.
    pub fn new(x: i32, y: i32, radius: f64, line_thickness: u32, color: Color) -> Self {
        assert!(radius >= 0f64, "Circle's border radius should be negative.");

        Self {
            x,
            y,
            radius,
            line_thickness,
            color
        }
    }
}

/// Interface générique pour l'affichage sur topi.
impl Renderable for Circle {
    fn display(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        let radius = self.radius as i32;
        let thickness = self.line_thickness as i32;
        let inner_radius = max(0, radius - thickness);

        let squared_radius = radius * radius;
        let squared_inner_radius = inner_radius * inner_radius;

        for y in -radius..=radius {
            for x in -radius..=radius {
                let distance = x * x + y * y;
                if distance <= squared_radius && distance >= squared_inner_radius {
                    canvas.draw_point(rect::Point::new(self.x + x, self.y + y))
                        .expect("Failed to render circle's border.");
                }
            }
        }
    }
}
