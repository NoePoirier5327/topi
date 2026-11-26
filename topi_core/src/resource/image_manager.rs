//! Fichier contenant la manipulation d'un cache de surface d'images.

use std::collections::HashMap;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use std::rc::Rc;
use std::cell::RefCell;
use super::ResourceError;

const MAX_IMG_CACHE_SIZE: usize = 30;

pub struct ImageManager {
    image_cache: RefCell<HashMap<String, Rc<Surface<'static>>>>,
}

impl ImageManager {
    /// Construit un nouveau gestionnaire d'image pour topi.
    ///
    /// # Return
    /// Instance associée au nouveau gestionnaire de textures.
    pub fn new() -> Self {
        let mut image_cache: HashMap<String, Rc<Surface<'static>>>= HashMap::new();
        image_cache.reserve(MAX_IMG_CACHE_SIZE);

        Self {
            image_cache: RefCell::new(image_cache)
        }
    }

    /// Charge une nouvelle image dans le cache et renvoie une référence vers sa texture.
    ///
    /// # Argument
    /// * `img_path` chemin vers l'image à charger.
    ///
    /// # Return
    /// Ok() référence vers la surface chargée par la méthode.
    /// Err() FailedToLoad.
    pub fn load_image(
        &self,
        img_path: &str,
    ) -> Result<Rc<Surface<'static>>, ResourceError> {
        if let Some(texture) = self.image_cache.borrow().get(img_path) {
            return Ok(texture.clone())
        }

        self.handle_img_cache_size();

        let texture = Surface::from_file(img_path).map_err(ResourceError::LoadImage)?;
        let texture = Rc::new(texture);

        self.image_cache.borrow_mut().insert(img_path.to_string(), texture.clone());

        Ok(texture)
    }

    /// Gère le dépassement mémoire du cache des images chargées par le TextureManager.
    /// Si images.len() >= MAX_IMG_CACHE_SIZE alors on décharge self.images.len() -
    /// MAX_IMG_CACHE_SIZE + 1 images du cache pour être à MAX_IMG_CACHE_SIZE - 1 images dans le
    /// cache et avoir de la place pour en allouer une nouvelle.
    fn handle_img_cache_size(&self) {
        let nb_img = self.image_cache.borrow().len();
        if nb_img >= MAX_IMG_CACHE_SIZE {
            let nb_img_to_unload = nb_img - MAX_IMG_CACHE_SIZE + 1;
            let imgs_to_unload = self.image_cache
                .borrow()
                .keys()
                .take(nb_img_to_unload)
                .cloned()
                .collect::<Vec<String>>();

            for img in imgs_to_unload {
                self.image_cache.borrow_mut().remove(&img);
            }
        }

        debug_assert!(self.image_cache.borrow().len() <= MAX_IMG_CACHE_SIZE, "Image cache overload detected.");
    }
}
