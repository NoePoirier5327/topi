//! Fichier contenant la manipulation des images comme élément affichable par le moteur topi.

use std::rc::Rc;
use super::Renderable;
use sdl2::{render::WindowCanvas, surface::Surface, rect::Rect};

pub struct Image {
    surface: Rc<Surface<'static>>,
    src_rect: Option<Rect>,
    dst_rect: Rect
}

impl Image {
    /// Construit une nouvelle image affichable par topi.
    ///
    /// # Arguments
    /// * `surface` surface de l'image à afficher.
    /// * `src_rect` rectangle ciblant la portion de l'image à afficher.
    /// * `dst_rect` rectangle ciblant la position finale de la texture affichée sur la fenêtre sdl2.
    /// 
    /// # Return
    /// Instance vers la nouvell image affichable.
    pub fn new(surface: Rc<Surface<'static>>, src_rect: Option<Rect>, dst_rect: Rect) -> Self {
        Self { surface, src_rect, dst_rect }
    }
}

/// Interface générique vers l'affichage.
impl Renderable for Image {
    fn display(&self, canvas: &mut WindowCanvas) {
        let texture_creator = canvas.texture_creator();
        if let Ok(texture) = texture_creator.create_texture_from_surface(&*self.surface) {
            let _ = canvas.copy(&texture, self.src_rect, self.dst_rect);
        }
    }
}
