//! Fichier contenant l'interface publique vers la manipulation de l'affichage de la fenêtre topi.

mod render_manager;

use std::rc::Rc;
use render_manager::RenderManager;
use crate::{resource::ResourceManager};
use crate::types;
use sdl2::{rect::Rect, render::WindowCanvas};

/// Wrapper pour la création d'un contexte de rendu.
/// Ne doit pas être exporté de manière publique à l'utilisateur.
pub fn new_render_context (
    canvas: WindowCanvas,
    window_width: u32,
    window_height: u32,
    resource_manager: Rc<ResourceManager>
) -> RenderContext {
    RenderContext::new(canvas, window_width, window_height, resource_manager)
}

/// Wrapper sur la mise à jour d'un contexte de rendu.
/// Ne doit pas être exporté de manière publique à l'utilisateur.
pub fn update_render_context(renderer: &mut RenderContext) {
    renderer.update();
}

/// Structure gérant les manipulations relative à l'affichage dans topi.
/// Contient des interfaces de dessin sur l'écran mais aussi des interfaces de gestion des
/// ressources visuelles (polices, textures etc...).
pub struct RenderContext {
    resource_manager: Rc<ResourceManager>,
    render_manager: RenderManager,
}

impl RenderContext {
    /// Construit un nouveau contexte d'affichage.
    /// Doit être appeler depuis le wrapper `new_render_context` pour éviter que l'utilisateur ait
    /// accès à cette fonction car interne à la logique du moteur.
    ///
    /// # Argument
    /// * `canvas` fenêtre de rendu sdl2 sur laquelle afficher le jeu.
    /// * `window_width` largeur de la fenêtre sous-jacente.
    /// * `window_height` hauteur de la fenêtre sous-jacente.
    /// * `resource_manager` référence vers le gestionnaire de ressources de l'instance de App de topi.
    ///
    /// # Return
    /// Instance vers le nouveau contexte d'affichage.
    fn new(
        canvas: WindowCanvas,
        window_width: u32,
        window_height: u32,
        resource_manager: Rc<ResourceManager>
    ) -> Self {
        Self {
            resource_manager,
            render_manager: RenderManager::new(canvas, window_width, window_height),
        }
    }

    /// Met à jour le contexte d'affichage courant.
    /// Doit être appeler depuis le wrapper `update_render_context` pour éviter que l'utilisateur
    /// ait accès à cette méthode car interne à la logique du moteur.
    fn update(&mut self) {
        self.render_manager.update();
    }

    /// Affiche un nouveau rectangle sur la fenêtre de jeu.
    ///
    /// # Arguments
    /// * `x` position en x du haut gauche du nouveau rectangle.
    /// * `y` position en y du haut gauche du nouveau rectangle.
    /// * `width` largeur du nouveau rectangle.
    /// * `height` hauteur du nouveau rectangle.
    #[inline]
    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32) {
        self.render_manager.draw_rect(x, y, width, height);
    }

    /// Affiche un nouveau cercle sur la fenêtre de jeu.
    ///
    /// # Arguments
    /// * `cx` position en x du centre du nouveau cercle.
    /// * `cy` position en y du centre du nouveau cercle.
    /// * `radius` rayon du nouveau cercle, doit être positif ou nul.
    #[inline]
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: f64) {
        self.render_manager.draw_circle(cx, cy, radius);
    }

    /// Affiche une nouvelle ligne sur la fenêtre de jeu.
    ///
    /// # Arguments
    /// * `x1` position en x du premier point de la nouvelle ligne.
    /// * `y1` position en y du premier point de la nouvelle ligne.
    /// * `x2` position en x du second point de la nouvelle ligne.
    /// * `y2` position en y du second point de la nouvelle ligne.
    /// * `thickness` largeur de la nouvelle ligne.
    #[inline]
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, thickness: u32) {
        self.render_manager.draw_line(x1, y1, x2, y2, thickness);
    }

    /// Affiche un texte sur la fenêtre de jeu.
    /// Si aucune police n'est chargée, plante intentionnelement.
    ///
    /// # Arguments
    /// * `text` texte à afficher sur la fenêtre de jeu.
    /// * `x` position en x du texte à afficher.
    /// * `y` position en y du texte à afficher.
    #[inline]
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32) {
        let (font_path, font_size) = self.render_manager.get_font();
        assert!(!font_path.is_empty(), "No font loaded to draw the text.");

        let surface = self.resource_manager.new_text_surface(font_path, font_size, text, self.render_manager.get_draw_color()).unwrap();

        self.render_manager.draw_text(surface, x, y);
    }

    /// Affiche une image en fonction de sa position cible.
    ///
    /// # Arguments
    /// * `img_path` chemin vers l'image à afficher.
    /// * `x` position en x de l'image affichée.
    /// * `y` position en y de l'image affichée.
    #[inline]
    pub fn draw_image(&mut self, img_path: &str, x: i32, y: i32) {
        let surface = self.resource_manager.new_img_surface(img_path).unwrap();
        let dst_rect = Rect::new(x, y, surface.width(), surface.height());
        self.render_manager.draw_image(surface, None, dst_rect);
    }

    /// Affiche une image en fonction de sa position cible et de son échelle de rendu.
    ///
    /// # Arguments
    /// * `img_path` chemin vers l'image à afficher.
    /// * `x` position en x de l'image à afficher.
    /// * `y` position en y de l'image à afficher.
    /// * `scale` échelle de rendu de l'image à afficher.
    #[inline]
    pub fn draw_scaled_image(&mut self, img_path: &str, x: i32, y: i32, scale: f64) {
        assert!(scale >= 0f64, "The image scale can't be negative.");
        let surface = self.resource_manager.new_img_surface(img_path).unwrap();
        let dst_rect = Rect::new(x, y, (surface.width() as f64 * scale) as u32, (surface.height() as f64 * scale) as u32);
        self.render_manager.draw_image(surface, None, dst_rect);
    }

    /// Affiche une portion de l'image dont le chemin est en paramètre.
    ///
    /// # Arguments
    /// * `img_path` chemin vers l'image dont on veut afficher une portion.
    /// * `x` position gauche sur la fenêtre de jeu de l'image qu'on veut afficher.
    /// * `y` position en hauteur sur la fenêtre de jeu de l'image qu'on veut afficher.
    /// * `scale` échelle de rendue de l'image sur la fenêtre de jeu.
    /// * `target_img` cible la portion qu'on veut afficher sur l'image dont on a le chemin en paramètre.
    #[inline]
    pub fn draw_image_part(&mut self, img_path: &str, x: i32, y: i32, scale: f64, target_img: &crate::types::rect::Rect) {
        assert!(scale >= 0f64, "The image scale can't be negative.");
        let surface = self.resource_manager.new_img_surface(img_path).unwrap();
        let dst_rect = Rect::new(x, y, (target_img.width() as f64 * scale) as u32, (target_img.height() as f64 * scale) as u32);
        self.render_manager.draw_image(surface, Some(*target_img), dst_rect);
    }

    /// Change la couleur d'affichage globale pour celle en paramètre.
    ///
    /// # Argument
    /// * `color` nouvelle couleur d'affichage principale du moteur.
    #[inline]
    pub fn set_draw_color(&mut self, color: &types::color::Color) {
        self.render_manager.set_draw_color(color);
    }

    /// Change la couleur d'affichage des lignes externes aux formes affichées par le moteur.
    ///
    /// # Argument
    /// * `color` nouvelle couleur attribuée aux bords des formes affichées par le moteur.
    #[inline]
    pub fn set_stroke_color(&mut self, color: &types::color::Color) {
        self.render_manager.set_stroke_color(color);
    }

    /// Change l'épaisseur des lignes de bords des formes affichables par le moteur.
    ///
    /// # Argument
    /// * `stroke_weight` nouvelle épaisseur de ligne.
    #[inline]
    pub fn set_stroke_weight(&mut self, stroke_weight: u32) {
        self.render_manager.set_stroke_weight(stroke_weight);
    }

    /// Active l'affichage des formes pleines.
    #[inline]
    pub fn enable_filling(&mut self) {
        self.render_manager.enable_filling();
    }

    /// Désactive l'affichage des formes pleines, seul la ligne de bord des formes va être affichée.
    #[inline]
    pub fn disable_filling(&mut self) {
        self.render_manager.disable_filling();
    }

    // Charge la police de caractères en paramètre comme police principale pour le rendu.
    //
    // # Arguments
    // * `font_path` chemin vers la police à charger.
    // * `font_size` taille du texte de la police à charger.
    #[inline]
    pub fn set_font(&mut self, font_path: &str, font_size: u16) {
        self.render_manager.set_font(font_path, font_size);
    }
}
