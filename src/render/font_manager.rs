//! Fichier contenant un gestionnaire de polices d'écriture (les charges et les stockes dans un
//! cache pour gagner du temps au chargement).

use core::fmt;
use std::collections::HashMap;
use sdl2::{ttf, ttf::Sdl2TtfContext};

const MAX_CACHE_SIZE: usize = 30;
const CACHE_SIZE_PER_ELM: usize = 10;

/// Gère un cache de polices d'écritures pour l'affichage.
pub struct FontManager {
    ttf_context: &'static Sdl2TtfContext,
    fonts: HashMap<FontKey, ttf::Font<'static, 'static>>
}

impl FontManager {
    /// Construit un nouveau FontManager dont le cache est vide.
    ///
    /// # Argument
    /// * `ttf_context` chargeur de polices sdl2.
    ///
    /// # Return
    /// Instance vers le nouveau FontManager.
    pub fn new(ttf_context: &'static Sdl2TtfContext) -> Self {
        let mut fonts: HashMap<FontKey, ttf::Font<'static, 'static>> = HashMap::new();
        fonts.reserve(MAX_CACHE_SIZE);

        Self {
            ttf_context,
            fonts
        }
    }

    /// Charge dans le cache la nouvelle police d'écriture.
    ///
    /// # Arguments
    /// * `font_path` chemin vers la police à charger.
    /// * `font_size` taille de la police à charger.
    ///
    /// # Return
    /// Ok() référence vers la police qu'on vient de charger.
    /// Err() `FontManagerError::FailedToLoad` si erreur dans le chargement.
    pub fn load_font(
        &mut self,
        font_path: &str,
        font_size: u16
    ) -> Result<&ttf::Font<'static, 'static>, FontManagerError> {
        let font_key = FontKey {path: font_path.to_string(), size: font_size};

        if !self.fonts.contains_key(&font_key) {
            self.handle_cache_size(&font_key.path);
            let font = self.ttf_context
                .load_font(&font_key.path, font_key.size)
                .map_err(FontManagerError::FailedToLoad)?;
            self.fonts.insert(font_key.clone(), font);
        }

        Ok(self.fonts.get(&font_key).unwrap())
    }

    /// Décharge la police en paramètre si chargée.
    ///
    /// # Argument
    /// * `key` police à décharger.
    fn unload_font_by_key(&mut self, key: &FontKey) {
        self.fonts.remove(key);
    }

    /// Compte le nombre d'éléments avec le path en paramètre dans le cache.
    fn nb_elm_with_path(&self, path: &String) -> usize {
        let mut nb = 0;
        for key in self.fonts.keys() {
            if key.path == *path {
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
    fn handle_cache_size(&mut self, font_path: &String) {
        let nb_elm_with_same_path = self.nb_elm_with_path(font_path);
        if nb_elm_with_same_path >= CACHE_SIZE_PER_ELM {
            let nb_font_to_unload = nb_elm_with_same_path - CACHE_SIZE_PER_ELM + 1;
            let fonts_to_unload = self.fonts
                .keys()
                .filter(|val| val.path == *font_path)
                .take(nb_font_to_unload)
                .cloned()
                .collect::<Vec<FontKey>>();

            for font_to_unload in fonts_to_unload {
                self.unload_font_by_key(&font_to_unload);
            }
        }

        let cache_size = self.fonts.len();
        if cache_size >= MAX_CACHE_SIZE {
            let nb_font_to_unload = cache_size - MAX_CACHE_SIZE + 1;
            let fonts_to_unload = self.fonts
                .keys()
                .take(nb_font_to_unload)
                .cloned()
                .collect::<Vec<FontKey>>();

            for font_to_unload in fonts_to_unload {
                self.unload_font_by_key(&font_to_unload);
            }
        }

        debug_assert!(self.fonts.len() <= MAX_CACHE_SIZE, "Font cache overload detected.");
    }
}

/// Clée pour les polices dans le cache.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FontKey {
    path: String,
    size: u16
}

#[derive(Debug)]
pub enum FontManagerError {
    FailedToLoad(String)
}

impl fmt::Display for FontManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FontManagerError::FailedToLoad(reason) => {
                write!(f,"Failed to load the required font {reason}")
            }
        }
    }
}

impl std::error::Error for FontManagerError {}
