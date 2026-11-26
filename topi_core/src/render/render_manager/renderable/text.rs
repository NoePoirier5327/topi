//! Fichier contenant la structure de données gérant l'affichage du texte.

use sdl2::{rect::Rect, render::{TextureCreator, WindowCanvas}, surface::Surface, video::WindowContext};
use super::Renderable;
use std::rc::Rc;

/// Gère l'affichage d'un texte sur la fenêtre sdl2.
pub struct Text {
    surface: Rc<Surface<'static>>,
    pos: Rect,
    texture_creator: Rc<TextureCreator<WindowContext>>
}

impl Text {
    /// Construit un nouveau texte affichable par le moteur topi.
    ///
    /// # Arguments
    /// * `surface` surface du texte à afficher par topi.
    /// * `pos` position de la texture à afficher par topi.
    /// * `texture_creator` créateur de texture sdl2 sous lequel faire vivre la texture du texte.
    ///
    /// # Return
    /// Instance vers le nouveau texte affichable par topi.
    pub fn new(surface: Rc<Surface<'static>>, pos: &Rect, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self {
        Self {
            surface,
            pos: *pos,
            texture_creator
        }
    }
}

/// Interface générique pour topi.
impl Renderable for Text {
    fn display(&self, canvas: &mut WindowCanvas) {
        if let Ok(texture) = self.texture_creator.create_texture_from_surface(&*self.surface) {
            let _ = canvas.copy(&texture, None, Some(self.pos));
        }
    }
}
