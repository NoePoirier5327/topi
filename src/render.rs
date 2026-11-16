//! Fichier contenant l'interface publique vers la manipulation de l'affichage de la fenêtre topi.

mod render_manager;
mod font_manager;

use render_manager::RenderManager;
use font_manager::FontManager;
use sdl2::{render::WindowCanvas, ttf::Sdl2TtfContext};

use crate::types;

/// Wrapper pour la création d'un contexte de rendu.
/// Ne doit pas être exporté de manière publique à l'utilisateur.
pub fn new_render_context(
    canvas: WindowCanvas,
    window_width: u32,
    window_height: u32,
    ttf_context: &'static Sdl2TtfContext
) -> RenderContext {
    RenderContext::new(canvas, window_width, window_height, ttf_context)
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
    render_manager: RenderManager,
    font_manager: FontManager
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
    /// * `ttf_context` contexte de création de polices de caractères.
    ///
    /// # Return
    /// Instance vers le nouveau contexte d'affichage.
    fn new(
        canvas: WindowCanvas,
        window_width: u32,
        window_height: u32,
        ttf_context: &'static Sdl2TtfContext
    ) -> Self {
        Self {
            render_manager: RenderManager::new(canvas, window_width, window_height),
            font_manager: FontManager::new(ttf_context)
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

        let font = self.font_manager.load_font(font_path, font_size).unwrap();
        self.render_manager.draw_text(text, x, y, font);
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
    #[inline]
    pub fn set_font(&mut self, font_path: &str, font_size: u16) {
        // On charge la police dans le cache pour faciliter la détection des erreurs si elle n'est
        // pas valide avant de passer à la suite.
        self.font_manager.load_font(font_path, font_size).unwrap();
        self.render_manager.set_font(font_path, font_size);
    }
}
