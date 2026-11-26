//! Fichier contenant la manipulation des images comme élément affichable par le moteur topi.

use std::rc::Rc;
use super::Renderable;
use sdl2::{rect::Rect, render::{TextureCreator, WindowCanvas}, surface::Surface, video::WindowContext};

pub struct Image {
    surface: Rc<Surface<'static>>,
    src_rect: Option<Rect>,
    dst_rect: Rect,
    texture_creator: Rc<TextureCreator<WindowContext>>
}

impl Image {
    /// Construit une nouvelle image affichable par topi.
    ///
    /// # Arguments
    /// * `surface` surface de l'image à afficher.
    /// * `src_rect` rectangle ciblant la portion de l'image à afficher.
    /// * `dst_rect` rectangle ciblant la position finale de la texture affichée sur la fenêtre sdl2.
    /// * `texture_creator` créateur de texture sdl2 sous lequel faire vivre l'image courante.
    /// 
    /// # Return
    /// Instance vers la nouvell image affichable.
    pub fn new(surface: Rc<Surface<'static>>, src_rect: Option<Rect>, dst_rect: Rect, texture_creator: Rc<TextureCreator<WindowContext>>) -> Self {
        Self { surface, src_rect, dst_rect, texture_creator }
    }
}

/// Interface générique vers l'affichage.
impl Renderable for Image {
    fn display(&self, canvas: &mut WindowCanvas) {
        if let Ok(texture) = self.texture_creator.create_texture_from_surface(&*self.surface) {
            let _ = canvas.copy(&texture, self.src_rect, self.dst_rect);
        }
    }
}
