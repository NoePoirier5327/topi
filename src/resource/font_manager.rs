//! Fichier contenant un gestionnaire de polices d'écriture (les charges et les stockes dans un
//! cache pour gagner du temps au chargement).

use std::collections::HashMap;
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::pixels::Color;
use std::rc::Rc;
use std::cell::RefCell;
use super::ResourceError;

const MAX_CACHE_SIZE: usize = 30;
const CACHE_SIZE_PER_ELM: usize = 10;

/// Gère un cache de polices d'écritures pour l'affichage.
pub struct FontManager {
    surface_cache: RefCell<HashMap<SurfaceKey, Rc<Surface<'static>>>>
}

impl FontManager {
    /// Construit un nouveau FontManager dont le cache est vide.
    ///
    /// # Return
    /// Instance vers le nouveau FontManager.
    pub fn new() -> Self {
        let mut surface_cache: HashMap<SurfaceKey, Rc<Surface<'static>>> = HashMap::new();
        surface_cache.reserve(MAX_CACHE_SIZE);

        Self {
            surface_cache: RefCell::new(surface_cache)
        }
    }

    /// Charge dans le cache la nouvelle police d'écriture.
    ///
    /// # Arguments
    /// * `font_path` chemin vers la police à charger.
    /// * `font_size` taille de la police à charger.
    /// * `ttf_context` contexte des polices d'écriture de sdl2.
    ///
    /// # Return
    /// Ok() référence vers la police qu'on vient de charger.
    /// Err() `FontManagerError::FailedToLoad` si erreur dans le chargement.
    pub fn load_font(
        &self,
        font_path: &str,
        font_size: u16,
        text: &str,
        color: Color,
        ttf_context: &Sdl2TtfContext
    ) -> Result<Rc<Surface<'static>>, ResourceError> {
        let font_key = SurfaceKey {font_path: font_path.to_string(), font_size, text: text.to_string(), color};
        if let Some(font) = self.surface_cache.borrow().get(&font_key) {
            return Ok(font.clone())
        }

        self.handle_cache_size(font_path);

        let font = ttf_context
            .load_font(font_path, font_size)
            .map_err(ResourceError::FailedToLoad)?;
        let font = font
            .render(text)
            .blended(color)
            .map_err(|e| ResourceError::FailedToCreateSurface(e.to_string()))?;

        let font = Rc::new(font);

        self.surface_cache.borrow_mut().insert(font_key, font.clone());

        Ok(font)
    }

    /// Décharge la police en paramètre si chargée.
    ///
    /// # Argument
    /// * `key` police à décharger.
    fn unload_font_by_key(&self, key: &SurfaceKey) {
        self.surface_cache.borrow_mut().remove(key);
    }

    /// Compte le nombre d'éléments avec le path en paramètre dans le cache.
    fn nb_elm_with_path(&self, path: &str) -> usize {
        let mut nb = 0;
        for key in self.surface_cache.borrow().keys() {
            if key.font_path == *path {
                nb += 1;
            }
        }
        nb
    }

    /// `fonts` est composée d'élément chargés par `load_font()`.
    ///
    /// Si le nombre d'éléments de même key.path que celui en paramètre dépasse CACHE_SIZE_PER_ELM,
    /// on en décharge un.
    /// Sinon, si fonts.size >= CACHE_SIZE alors on décharge le premier accessible.
    /// Sinon, on fait rien.
    fn handle_cache_size(&self, font_path: &str) {
        let nb_elm_with_same_path = self.nb_elm_with_path(font_path);
        if nb_elm_with_same_path >= CACHE_SIZE_PER_ELM {
            let nb_font_to_unload = nb_elm_with_same_path - CACHE_SIZE_PER_ELM + 1;
            let fonts_to_unload = self.surface_cache
                .borrow()
                .keys()
                .filter(|val| val.font_path == *font_path)
                .take(nb_font_to_unload)
                .cloned()
                .collect::<Vec<SurfaceKey>>();

            for font_to_unload in fonts_to_unload {
                self.unload_font_by_key(&font_to_unload);
            }
        }

        let cache_size = self.surface_cache.borrow().len();
        if cache_size >= MAX_CACHE_SIZE {
            let nb_font_to_unload = cache_size - MAX_CACHE_SIZE + 1;
            let fonts_to_unload = self.surface_cache
                .borrow()
                .keys()
                .take(nb_font_to_unload)
                .cloned()
                .collect::<Vec<SurfaceKey>>();

            for font_to_unload in fonts_to_unload {
                self.unload_font_by_key(&font_to_unload);
            }
        }

        debug_assert!(self.surface_cache.borrow().len() <= MAX_CACHE_SIZE, "Font cache overload detected.");
    }
}

/// Clée pour les polices dans le cache.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SurfaceKey {
    font_path: String,
    font_size: u16,
    text: String,
    color: Color
}
