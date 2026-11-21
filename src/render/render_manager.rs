//! Fichier contenant l'ensemble des interfaces relatives à l'affichage sur la fenêtre de jeu sdl2 courante.

mod renderable;

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, surface::Surface};
use std::rc::Rc;
use renderable::{Renderable, circle, line, rectangle};
use crate::{render::render_manager::renderable::{image, text}, types};

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

        //println!("{}", self.renderable_items.len());

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
        if !self.no_fill && self.draw_color.a != 0 && Self::rect_is_in_window(x, y, width, height, self.window_width, self.window_height){
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

            if Self::rect_is_in_window(ox, oy, ow, oh, self.window_width, self.window_height) {
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
    }

    /// Affiche un nouveau cercle sur la fenêtre de jeu courante.
    ///
    /// # Arguments
    /// * `cx` coordonnée du centre du nouveau cercle.
    /// * `cy` coordonnée du centre du nouveau cercle.
    /// * `radius` rayon du nouveau cercle (doit être positif ou nul).
    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: f64) {
        // On créer le nouveau cercle plein au centre.
        if !self.no_fill && self.draw_color.a != 0 && Self::circle_is_in_window(cx, cy, radius, self.window_width, self.window_height) {
            self.renderable_items.push(
                Box::new(circle::FilledCircle::new(cx, cy, radius, self.draw_color))
            );
        }

        // On créer le nouveau cercle de bordure.
        if self.stroke_color.a != 0 && self.stroke_weight != 0 {
            let out_radius = radius + self.stroke_weight as f64;
            if Self::circle_is_in_window(cx, cy, out_radius, self.window_width, self.window_height) {
                self.renderable_items.push(
                    Box::new(
                        circle::Circle::new(
                            cx, cy,
                            out_radius,
                            self.stroke_weight,
                            self.stroke_color
                        )
                    )
                );
            }
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
        if !Self::line_is_in_window(x1, y1, x2, y2, thickness, self.window_width, self.window_height) { return }

        self.renderable_items.push(
            Box::new(line::Line::new(x1, y1, x2, y2, thickness, self.draw_color))
        );
    }

    /// Affiche un nouveau texte sur la fenêtre sdl2.
    ///
    /// # Arguments
    /// * `surface` surface du texte à afficher.
    /// * `x` position en x du texte à afficher.
    /// * `y` position en y du texte à afficher.
    pub fn draw_text(&mut self, surface: Rc<Surface<'static>>, x: i32, y: i32) {
        if self.draw_color.a == 0 { return }

        let target_pos = Rect::new(x, y, surface.width(), surface.height());
        if !Self::rect_is_in_window(x, y, surface.width(), surface.height(), self.window_width, self.window_height) { return }

        self.renderable_items.push(
            Box::new(
                text::Text::new(surface, &target_pos)
            )
        );
    }

    /// Affiche une image sur la fenêtre sdl2 sous-jacente à partir de sa future position simple.
    ///
    /// # Arguments
    /// * `surface` référence vers la surface de l'image à afficher.
    /// * `src_rect` cible sur la portion de l'image à afficher, None affiche toute l'image.
    /// * `dst_rect` cible sur la position et la taille final de l'image sur la fenêtre de rendue sdl2.
    pub fn draw_image(&mut self, surface: Rc<Surface<'static>>, src_rect: Option<Rect>, dst_rect: Rect) {
        if !Self::rect_is_in_window(dst_rect.x, dst_rect.y, dst_rect.w as u32, dst_rect.h as u32, self.window_width, self.window_height) { return }

        self .renderable_items.push(
            Box::new(
                image::Image::new(surface, src_rect, dst_rect)
            )
        );
    }

    /// Mutateur de la couleur de rendue des objets du moteur.
    #[inline]
    pub fn set_draw_color(&mut self, color: &types::color::Color) {
        self.draw_color = Color::RGBA(color.get_r(), color.get_g(), color.get_b(), color.get_a());
    }

    /// Renvoie la couleur d'affichage courante.
    #[inline]
    pub fn get_draw_color(&self) -> Color {
        self.draw_color
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

    /// Vérifie que le rectangle en paramètre est entièrement ou partiellement dans la fenêtre de
    /// jeu.
    ///
    /// # Arguments
    /// * `x` position de la gauche du rectangle.
    /// * `y` position du haut du rectangle.
    /// * `w` largeur du rectangle.
    /// * `h` hauteur du rectangle.
    /// * `window_width` largeur de la fenêtre de jeu.
    /// * `window_height` hauteur de la fenêtre de jeu.
    ///
    /// # Return
    /// `true` le rectangle est dans la fenêtre, `false` sinon.
    #[inline]
    fn rect_is_in_window(x: i32, y: i32, w: u32, h: u32, window_width: u32, window_height: u32) -> bool {
        let (w, h) = (w as i32, h as i32);
        let (win_width, win_height) = (window_width as i32, window_height as i32);

        !(
            x + w <= 0
            || x >= win_width
            || y + h <= 0
            || y >= win_height
        )
    }

    /// Vérifie que le cercle en paramètre en entièrement ou partiellement dans la fenêtre de jeu en
    /// paramètre.
    ///
    /// # Arguments
    /// * `cx` position en x du centre du cercle.
    /// * `cy` position en y du centre du cercle.
    /// * `radius` rayon du cercle.
    /// * `window_width` largeur de la fenêtre de jeu.
    /// * `window_height` hauteur de la fenêtre de jeu.
    ///
    /// # Return
    /// `true` le cercle est dans la fenêtre, `false` sinon.
    #[inline]
    fn circle_is_in_window(cx: i32, cy: i32, radius: f64, window_width: u32, window_height: u32) -> bool {
        let (cx, cy) = (cx as f64, cy as f64);

        // Point de la fenêtre le plus proche du cercle.
        let closest_x = cx.clamp(0f64, window_width as f64);
        let closest_y = cy.clamp(0f64, window_height as f64);

        let dx = cx - closest_x;
        let dy = cy - closest_y;

        dx * dx + dy * dy < radius * radius
    }

    /// Vérifie que la ligne en paramètre est dans la fenêtre de jeu en paramètre.
    /// Fonction générée par Claude Haiku 5.5.
    ///
    /// # Arguments
    /// * `x1` coordonnée en x du premier point caractérisant la ligne.
    /// * `y1` coordonnée en y du premier point caractérisant la ligne.
    /// * `x2` coordonnée en x du second point caractérisant la ligne.
    /// * `y2` coordonnée en y du second point caractérisant la ligne.
    /// * `thickness` épaisseur de la ligne.
    /// * `window_width` largeur de la fenêtre de jeu.
    /// * `window_height` hauteur de la fenêtre de jeu.
    ///
    /// # Return
    /// `true` la ligne est dans la fenêtre, `false` sinon.
    fn line_is_in_window(x1: i32, y1: i32, x2: i32, y2: i32, thickness: u32, window_width: u32, window_height: u32) -> bool {
        let half = thickness as f64 * 0.5;
        let (x1, y1, x2, y2) = (x1 as f64, y1 as f64, x2 as f64, y2 as f64);
        let (dx, dy) = (x2 - x1, y2 - y1);

        let len = (dx * dx + dy * dy).sqrt();
        let (margin_x, margin_y) = if len == 0f64 {
            (half, half)
        } else {
            (half * dy.abs() / len, half * dx.abs() / len)
        };

        let (x_min, y_min) = (-margin_x, -margin_y);
        let (x_max, y_max) = (window_width as f64 + margin_x, window_height as f64 + margin_y);

        let p = [-dx, dx, -dy, dy];
        let q = [x1 - x_min, x_max - x1, y1 - y_min, y_max - y1];

        let (mut t0, mut t1) = (0f64, 1f64);

        for i in 0..4 {
            if p[i] == 0f64 {
                if q[i] <= 0f64 { return false }
            } else {
                let r = q[i] / p[i];
                if p[i] < 0f64 {
                    if r >= t1 { return false }
                    t0 = t0.max(r);
                } else {
                    if r <= t0 { return false }
                    t1 = t1.min(r);
                }
            }
        }

        t0 < t1
    }
}
