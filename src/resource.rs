//! Fichier contenant les interfaces de gestion des ressources de topi.

pub mod font_manager;

use sdl2::{render::{TextureCreator, WindowCanvas}, ttf::Sdl2TtfContext, video::WindowContext, surface::Surface, pixels::Color};
use std::rc::Rc;
use font_manager::FontManager;

/// Interface unifiée vers la gestion des ressources dans topi.
pub struct ResourceManager {
    font_manager: FontManager,
    ttf_context: Sdl2TtfContext,
}

impl ResourceManager {
    /// Construit un nouveau gestionnaire de ressources contenant les éléments critiques des
    /// ressources de topi (contexte sdl2 tff et créteur de textures).
    ///
    /// # Argument
    /// * `canvas` référence vers le canvas pour lequel gérer les textures.
    ///
    /// # Return
    /// Instance vers le nouveau gestionnaire de ressources.
    pub fn new(canvas: &WindowCanvas) -> Self {
        Self {
            font_manager: FontManager::new(),
            ttf_context: sdl2::ttf::init().unwrap(),
        }
    }

    #[inline]
    pub fn new_text_surface(&self, font_path: &str, font_size: u16, text: &str, color: Color) -> Result<Rc<Surface<'static>>, ResourceError> {
        self.font_manager.load_font(font_path, font_size, text, color, &self.ttf_context)
    }
}

/// Type gérant les erreurs liées à la manipulation des ressources dans topi.
#[derive(Debug)]
pub enum ResourceError {
    FailedToLoad(String),
    FailedToCreateSurface(String)
}
