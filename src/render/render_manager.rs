//! Fichier contenant l'ensemble des interfaces relatives à l'affichage sur la fenêtre de jeu sdl2 courante.

mod renderable;

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, ttf};
use renderable::{Renderable, circle, line, rectangle};
use crate::{render::render_manager::renderable::text, types};

/// Interface vers l'affichage de la fenêtre de jeu courante.
pub struct RenderManager {
    canvas: WindowCanvas,
    window_width: u32,
    window_height: u32,
    draw_color: Color,
    stroke_color: Color,
    stroke_weight: u32,
    no_fill: bool,
    font_path: String,
    font_size: u16,
    renderable_items: Vec<Box<dyn Renderable>>
}

impl RenderManager {
    /// Construit un nouveau gestionnaire de rendue manipulant une fenêtre sdl2 pour l'affichage en
    /// interne.
    ///
    /// # Argument
    /// * `canvas` surface de rendue sdl2 sous-jacente à la fenêtre courante.
    /// * `window_width` largeur de la fenêtre sous-jacente.
    /// * `window_height` hauteur de la fenêtre sous-jacente.
    ///
    /// # Return
    /// Instance vers le nouveau gestionnaire de rendue.
    pub fn new(canvas: WindowCanvas, window_width: u32, window_height: u32) -> RenderManager {
        RenderManager { 
            canvas,
            window_width,
            window_height,
            draw_color: Color::RGB(0, 0, 0),
            stroke_color: Color::RGB(0, 0, 0),
            stroke_weight: 0u32,
            no_fill: false,
            font_path: "".to_string(),
            font_size: 0,
            renderable_items: Vec::new()
        }
    }

    /// Met à jour le gestionnaire de rendue courant, affiche l'ensemble des formes demandées au
    /// module.
    pub fn update(&mut self) {
        // On nettoie la fenêtre de rendue.
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // On charge les changements faits par l'utilisateur.
        for item in self.renderable_items.drain(..) {
            item.display(&mut self.canvas);
        }

        // Vérification importante sinon la mémoire s'accumule !!!
        debug_assert_eq!(self.renderable_items.len(), 0, "The drawing vector isn't empty :(.");

        // On les appliques.
        self.canvas.present();
    }

    /// Affiche un nouveau rectangle sur la fenêtre de jeu courante.
    ///
    /// # Arguments
    /// * `x` coordonnée en x du nouveau rectangle.
    /// * `y` coordonnée en y du nouveau rectangle.
    /// * `width` largeur du nouveau rectangle.
    /// * `height` hauteur du nouveau rectangle.
    pub fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32) {
        // On créer le nouveau rectangle plein au centre.
        if !self.no_fill && self.draw_color.a != 0 {
            self.renderable_items.push(
                Box::new(rectangle::FilledRectangle::new(x, y, width, height, self.draw_color))
            );
        }

        // On créer le nouveau rectangle vide par dessus pour faire la bordure.
        if self.stroke_color.a != 0 && self.stroke_weight != 0 {
            // On calcul les valeurs du rectangle de bordure.
            let half_stroke_size = self.stroke_weight / 2;
            let ox = x - half_stroke_size as i32;
            let oy = y - half_stroke_size as i32;
            let ow = width + 2 * half_stroke_size;
            let oh = height + 2 * half_stroke_size;

            self.renderable_items.push(
                Box::new(
                    rectangle::Rectangle::new(
                        ox, oy, ow, oh,
                        self.stroke_weight,
                        self.stroke_color
                    )
                )
            );
        }
    }

    /// Affiche un nouveau cercle sur la fenêtre de jeu courante.
    ///
    /// # Arguments
    /// * `cx` coordonnée du centre du nouveau cercle.
    /// * `cy` coordonnée du centre du nouveau cercle.
    /// * `radius` rayon du nouveau cercle (doit être positif ou nul).
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: f64) {
        // On créer le nouveau cercle plein au centre.
        if !self.no_fill && self.draw_color.a != 0 {
            self.renderable_items.push(
                Box::new(circle::FilledCircle::new(cx, cy, radius, self.draw_color))
            );
        }

        // On créer le nouveau cercle de bordure.
        if self.stroke_color.a != 0 && self.stroke_weight != 0 {
            self.renderable_items.push(
                Box::new(
                    circle::Circle::new(
                        cx, cy,
                        radius + self.stroke_weight as f64,
                        self.stroke_weight,
                        self.stroke_color
                    )
                )
            );
        }
    }

    /// Affiche une nouvelle ligne sur la fenêtre de jeu courante.
    ///
    /// # Arguments
    /// * `x1` coordonnée en x du premier point de la nouvelle ligne.
    /// * `y1` coordonnée en y du premier point de la nouvelle ligne.
    /// * `x2` coordonnée en x du second point de la nouvelle ligne.
    /// * `y2` coordonnée en y du second point de la nouvelle ligne.
    /// * `thickness` épaisseur de la nouvelle ligne.
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, thickness: u32) {
        if self.draw_color.a == 0 { return }
        self.renderable_items.push(
            Box::new(line::Line::new(x1, y1, x2, y2, thickness, self.draw_color))
        );
    }

    /// Affiche un nouveau texte sur la fenêtre sdl2.
    ///
    /// # Arguments
    /// * `text` texte à afficher.
    /// * `x` position en x du texte à afficher.
    /// * `y` position en y du texte à afficher.
    /// * `font` police d'écriture du texte à afficher.
    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, font: &ttf::Font) {
        if text.is_empty() || self.draw_color.a == 0 { return }

        let surface = font
            .render(text)
            .blended(Color::RGBA(self.draw_color.r, self.draw_color.g, self.draw_color.b, self.draw_color.a))
            .unwrap();

        let target_pos = Rect::new(x, y, surface.width(), surface.height());

        self.renderable_items.push(
            Box::new(
                text::Text::new(surface, &target_pos)
            )
        );
   }

    /// Mutateur de la couleur de rendue des objets du moteur.
    #[inline]
    pub fn set_draw_color(&mut self, color: &types::color::Color) {
        self.draw_color = Color::RGBA(color.get_r(), color.get_g(), color.get_b(), color.get_a());
    }

    /// Mutateur de la couleur des bordures des objets affichés par le moteur.
    #[inline]
    pub fn set_stroke_color(&mut self, color: &types::color::Color) {
        self.stroke_color = Color::RGBA(color.get_r(), color.get_g(), color.get_b(), color.get_a());
    }

    /// Mutateur de l'épaisseur des lignes des formes affichables.
    #[inline]
    pub fn set_stroke_weight(&mut self, stroke_weight: u32) {
        self.stroke_weight = stroke_weight;
    }

    /// Désactive le remplissage des formes dessinées par le moteur.
    #[inline]
    pub fn disable_filling(&mut self) {
        self.no_fill = true;
    }

    /// Active le remplissage des formes dessinées par le moteur.
    #[inline]
    pub fn enable_filling(&mut self) {
        self.no_fill = false;
    }

    /// Charge en mémoire la police de caractère en paramètre.
    /// Elle sera utilisée la prochaine fois qu'on veut afficher du texte.
    #[inline]
    pub fn set_font(&mut self, font_path: &str, font_size: u16) {
        self.font_path = font_path.to_string();
        self.font_size = font_size;
    }

    /// Renvoie la police d'écriture principale chargée au moment de la demande.
    #[inline]
    pub fn get_font(&self) -> (&str, u16) {
        (&self.font_path, self.font_size)
    }
}
