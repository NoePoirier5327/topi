use topi::RenderContext;
use topi::types::color::Color;
use topi::types::point::Point2f;
use topi::types::vector::Vector2D;
use topi::toolkit::rand;

const MIN_INITIAL_HORIZONTAL_SPEED: f64 = -10f64;
const MAX_INITIAL_HORIZONTAL_SPEED: f64 =  10f64;

/// Correspond à une balle dans la simulation de physique.
/// Elle a une position et un vecteur vitesse.
#[derive(Clone, Copy)]
pub struct Ball {
    pub pos: Point2f,
    pub speed: Vector2D,
    radius: f64,
    color: Color
}

impl Ball {
    /// Construit une nouvelle balle.
    /// Son vecteur vitesse d'origine est initialisé à 0.
    ///
    /// # Arguments
    /// * `x` position en x de la nouvelle balle.
    /// * `y` position en y de la nouvelle balle.
    /// * `radius` rayon de la nouvelle balle.
    ///
    /// # Return
    /// Instance vers la nouvelle balle.
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        assert!(radius > 0f64, "A ball radius can't be negative or null.");

        Self {
            pos: Point2f::new(x, y),
            speed: Vector2D::new(
                rand::randrange(MIN_INITIAL_HORIZONTAL_SPEED, MAX_INITIAL_HORIZONTAL_SPEED),
                0f64,
            ),
            radius,
            color: Color::new_rgb(
                rand::randrange(0f64, 255f64) as u8,
                rand::randrange(0f64, 255f64) as u8,
                rand::randrange(0f64, 255f64) as u8
            )
        }
    }

    /// Renvoie le rayon de la balle courante.
    #[inline]
    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// Affiche la balle courante sur la fenêtre de topi.
    pub fn display(&self, renderer: &mut RenderContext) {
        renderer.set_draw_color(&self.color);
        renderer.draw_circle(self.pos.get_x() as i32, self.pos.get_y() as i32, self.radius);
    }

    /// Met à jour la position interne de la balle en fonction de son vecteur vitesse.
    #[inline]
    pub fn update_pos(&mut self, dt: f64) {
        self.pos.set_x(self.pos.get_x() + self.speed.get_x() * dt);
        self.pos.set_y(self.pos.get_y() + self.speed.get_y() * dt);
    }

    /// Détecte une collision entre la balle courante et celle en pramètre.
    ///
    /// # Arguments
    /// * `ball` balle avec laquelle on veut savoir si celle courante est en collision.
    /// * `epsilon` facteur de tolérance des collisions (au plus il est grand, au plus on détecte mal les collisions et inversement).
    ///
    /// # Return
    /// `true` la balle en paramètre est en collision avec celle courante, `false` sinon.
    pub fn is_colliding_with_another_ball(&self, ball: &Ball, epsilon: f64) -> bool {
        assert!((0f64..=1f64).contains(&epsilon), "The tolerance factor for balls collision detection should be between 0 and 1.");
        let dx = ball.pos.get_x() - self.pos.get_x();
        let dy = ball.pos.get_y() - self.pos.get_y();
        let squared_dist = dx * dx + dy * dy;
        let radius_sum = self.get_radius() + ball.get_radius();

        squared_dist < radius_sum * radius_sum - epsilon && squared_dist > 0f64
    }

    /// Calcule et renvoie les vecteurs de corrections des position et vitesse de la balle courante
    /// à partir de la collision avec la balle en paramètre.
    ///
    /// La balle courante doit entrer en collision avec celle en paramètre.
    ///
    /// # Arguments
    /// * `ball` balle avec laquelle celle courante est en collision.
    /// * `correction_precision` facteur de précision de la correction renvoyé par la méthode.
    /// * `restitution` facteur de restitution de la force de contact.
    /// * `epsilon` facteur de tolérance des collisions (au plus il est grand, au plus on détecte mal les collision et inversement).
    ///
    /// # Return
    /// En première position du tuple, le vecteur de correction de la position de la balle courante et en deuxième
    /// position le vecteur de correction de la vitesse de la balle courante.
    pub fn calculate_collision_correction(&self, ball: &Ball, correction_precision: f64, restitution: f64, epsilon: f64) -> (Vector2D, Vector2D) {
        // Préconditions
        assert!((0f64..=1f64).contains(&correction_precision), "The correction precision factor for collisions correction should be between 0 and 1.");
        assert!((0f64..=1f64).contains(&restitution), "The restitution factor for collisions correction should be between 0 and 1.");
        assert!(self.is_colliding_with_another_ball(ball, epsilon), "To correct ball collisions, they should be colliding.");

        // On calcul le vecteur normal aux balles.
        let dx = ball.pos.get_x() - self.pos.get_x();
        let dy = ball.pos.get_y() - self.pos.get_y();
        let dist = (dx * dx + dy * dy).sqrt();
        let normal = Vector2D::new(dx / dist, dy / dist);

        // On calcul le facteur de correction des positions.
        let overlapping = self.get_radius() + ball.get_radius() - dist;
        let pos_correction_factor = overlapping * 0.5 * correction_precision;

        // On calcul la vitesse des balles relative à leur vecteur normal.
        let vel_dif = ball.speed - self.speed;
        let vel_along_normal = vel_dif.get_x() * normal.get_x() + vel_dif.get_y() * normal.get_y();

        // Si elle est négative, on calcul la force d'impulsion,
        // sinon, on la met à 0.
        let impulse = {
            if vel_along_normal < 0f64 {
                -(1f64 + restitution) * vel_along_normal / 2f64
            } else { 0f64 }
        };

        // On renvoie les vecteurs de correction pour la balle courante.
        (normal * pos_correction_factor, normal * impulse)
    }

    /// Applique les vecteurs de correction de position et vitesse à la balle courante.
    /// Est surtout utilisée pour gérer les collisions entre balle.
    ///
    /// # Arguments
    /// * `pos_correction` vecteur de correction de la position de la balle courante.
    /// * `speed_correction` vecteur de correction de la vitesse de la balle courante.
    pub fn correct_pos_and_speed(&mut self, pos_correction: &Vector2D, speed_correction: &Vector2D) {
        self.pos.set_x(self.pos.get_x() + pos_correction.get_x());
        self.pos.set_y(self.pos.get_y() + pos_correction.get_y());
        self.speed += *speed_correction;
    }
}
