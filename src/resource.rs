//! Fichier contenant les interfaces de gestion des ressources de topi.

mod font_manager;
mod image_manager;

use sdl2::{ttf::Sdl2TtfContext, surface::Surface, pixels::Color};
use std::rc::Rc;
use font_manager::FontManager;
use image_manager::ImageManager;

/// Interface unifiée vers la gestion des ressources dans topi.
pub struct ResourceManager {
    font_manager: FontManager,
    image_manager: ImageManager,
    ttf_context: Sdl2TtfContext,
}

impl ResourceManager {
    /// Construit un nouveau gestionnaire de ressources contenant les éléments critiques des
    /// ressources de topi (contexte sdl2 tff et créteur de textures).
    ///
    /// # Return
    /// Instance vers le nouveau gestionnaire de ressources.
    pub fn new() -> Self {
        Self {
            font_manager: FontManager::new(),
            image_manager: ImageManager::new(),
            ttf_context: sdl2::ttf::init().unwrap(),
        }
    }

    /// Charge une nouvelle surface de rendue de texte sdl2 à partir d'un texte et d'une police
    /// d'écriture.
    ///
    /// # Arguments
    /// * `font_path` chemin vers la police d'écriture cible.
    /// * `font_size` taille du texte de la police d'écriture.
    /// * `text` texte à afficher.
    /// * `color` couleur du texte à afficher.
    ///
    /// # Return
    /// Ok() référence vers la surface en cache.
    /// Err() erreur de ressource liée au chargement de la nouvelle surface.
    #[inline]
    pub fn new_text_surface(&self, font_path: &str, font_size: u16, text: &str, color: Color) -> Result<Rc<Surface<'static>>, ResourceError> {
        self.font_manager.load_font(font_path, font_size, text, color, &self.ttf_context)
    }

    /// Charge une nouvelle surface d'image sdl2.
    ///
    /// # Argument
    /// * `img_path` chemin vers l'image à charger.
    ///
    /// # Return
    /// Ok() référence vers la surface d'image.
    /// Err() erreur de ressource liée au chargement de la nouvelle texture.
    #[inline]
    pub fn new_img_surface(&self, img_path: &str) -> Result<Rc<Surface<'static>>, ResourceError> {
        self.image_manager.load_image(img_path)
    }
}

/// Type gérant les erreurs liées à la manipulation des ressources dans topi.
#[derive(Debug)]
pub enum ResourceError {
    LoadFont(String),
    LoadImage(String),
    CreateSurface(String)
}

impl core::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::LoadFont(reason) => {
                write!(f,"Failed to load the required font {reason}")
            },

            ResourceError::LoadImage(reason) => {
                write!(f, "Failed to load the required image {reason}")
            },

            ResourceError::CreateSurface(reason) => {
                write!(f, "Failed to create the required surface {reason}")
            }
        }
    }
}

impl std::error::Error for ResourceError {}
