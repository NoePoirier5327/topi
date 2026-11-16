//! Fichier contenant la structure de données gérant l'affichage du texte.

use sdl2::{rect::Rect, render::WindowCanvas, surface::Surface};
use super::Renderable;

/// Gère l'affichage d'un texte sur la fenêtre sdl2.
pub struct Text {
    surface: Surface<'static>,
    pos: Rect
}

impl Text {
    /// Construit un nouveau texte affichable par le moteur topi.
    ///
    /// # Arguments
    /// * `surface` surface du texte à afficher par topi.
    /// * `pos` position de la texture à afficher par topi.
    ///
    /// # Return
    /// Instance vers le nouveau texte affichable par topi.
    pub fn new(surface: Surface<'static>, pos: &Rect) -> Self {
        Self {
            surface,
            pos: *pos
        }
    }
}

/// Interface générique pour topi.
impl<'txt> Renderable for Text {
    fn display(&self, canvas: &mut WindowCanvas) {
        let texture_creator = canvas.texture_creator();
        if let Ok(texture) = texture_creator.create_texture_from_surface(&self.surface) {
            let _ = canvas.copy(&texture, None, Some(self.pos));
        }
    }
}
