use topi::{App, AppLogic, InputManager, RenderContext};
use topi::types::{vector::Vector2D, point::Point2f, color::Color};
use topi::toolkit::rand;

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 1000;

fn main() {
    App::new(
        "Example - Bouncing balls",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        BouncingBalls::new(50, -500, 500, 30f64, 50f64)
    ).run();
}

/// Structure correspondant à la logique interne de la simulation.
struct BouncingBalls {
    balls_speed: Vec<Vector2D>,
    balls_position: Vec<Point2f>,
    balls_color: Vec<Color>,
    balls_radius: Vec<f64>,
    stroke_weight: u32
}

impl BouncingBalls {
    /// Construit une nouvelle instance de la simulation.
    ///
    /// # Arguments
    /// * `nb_ball` nombre de balles à simuler.
    /// * `min_ball_speed` vitesse minimale des balles à simuler, s_max >= s_min.
    /// * `max_ball_speed` vitesse maximale des balles à simuler, s_max >= s_min.
    /// * `min_ball_radius` rayon minimale des balles à simuler, r_min > 0.
    /// * `max_ball_radius` rayon maximale des balles à simuler, r_max <= WINDOW_HEIGHT && r_max <= WINDOW_WIDTH && r_max >= r_min;
    pub fn new(
        nb_ball: usize,
        min_ball_speed: i32,
        max_ball_speed: i32,
        min_ball_radius: f64,
        max_ball_radius: f64
    ) -> Self {
        // Vérification des préconditions.
        assert!(max_ball_speed >= min_ball_speed, "The upper bound of the speed range should be greater than the lower one.");
        assert!(min_ball_radius > 0f64, "A ball can't have a negative or null radius.");
        assert!(max_ball_radius >= min_ball_radius, "The upper bound of the radius range should be greater than the lower one.");
        assert!(2f64 * max_ball_radius <= WINDOW_WIDTH as f64, "A ball size can't be greater than the window.");
        assert!(2f64 * max_ball_radius <= WINDOW_WIDTH as f64, "A ball size can't be greater then the window.");

        // Composantes de la simulation initialisés à vide.
        let mut balls_speed: Vec<Vector2D> = Vec::new();
        let mut balls_position: Vec<Point2f> = Vec::new();
        let mut balls_color: Vec<Color> = Vec::new();
        let mut balls_radius: Vec<f64> = Vec::new();

        // On construit les listes contenant les valeurs des balles simulées par l'application.
        for _ in 0..nb_ball {
            // Rayon de la nouvelle balle.
            let ball_radius = rand::randrange(min_ball_radius..=max_ball_radius);
            balls_radius.push(ball_radius);

            // Vecteur vitesse de la nouvelle balle.
            balls_speed.push(
                Vector2D::new(
                    rand::randrange(min_ball_speed..=max_ball_speed) as f64,
                    rand::randrange(min_ball_speed..=max_ball_speed) as f64
                )
            );

            // Centre de la nouvelle balle.
            balls_position.push(
                Point2f::new(
                    rand::randrange(ball_radius..=WINDOW_WIDTH as f64 - ball_radius),
                    rand::randrange(ball_radius..=WINDOW_HEIGHT as f64 - ball_radius)
                )
            );

            // Couleur de la nouvelle balle.
            balls_color.push(
                Color::RGB(
                    rand::randrange(0u8..=255u8),
                    rand::randrange(0u8..=255u8),
                    rand::randrange(0u8..=255u8)
                )
            );
        }

        Self {
            balls_speed,
            balls_position,
            balls_color,
            balls_radius,
            stroke_weight: 10
        }
    }
}

impl AppLogic for BouncingBalls {
    fn update(&mut self, dt: f64, _input: &InputManager) {
        for i in 0..self.balls_position.len() {
            // Composante de la balle courante.
            let p = &mut self.balls_position[i];
            let v = &mut self.balls_speed[i];
            let r = self.balls_radius[i];

            // On fait se déplacer les balles en fonction de leurs vecteurs vitesses.
            p.set_x(p.get_x() + v.get_x() * dt);
            p.set_y(p.get_y() + v.get_y() * dt);

            // On calcul les collisions avec les bords gauche et droite.
            if p.get_x() < r && v.get_x() < 0f64 {
                p.set_x(r);
                v.set_x(-v.get_x());
            }
            else if p.get_x() > WINDOW_WIDTH as f64 - r && v.get_x() > 0f64 {
                p.set_x(WINDOW_WIDTH as f64 - r);
                v.set_x(-v.get_x());
            }

            // On calcul les collisions avec les bords haut et bas.
            if p.get_y() < r && v.get_y() < 0f64 {
                p.set_y(r);
                v.set_y(-v.get_y());
            }
            else if p.get_y() > WINDOW_HEIGHT as f64 - r && v.get_y() > 0f64 {
                p.set_y(WINDOW_HEIGHT as f64 - r);
                v.set_y(-v.get_y());
            }
        }
    }

    fn display(&self, renderer: &mut RenderContext) {
        for i in 0..self.balls_position.len() {
            let ball_pos = self.balls_position[i];
            let ball_radius = self.balls_radius[i];

            // On affiche les balles plaines.
            if i.is_multiple_of(2) {
                renderer.set_draw_color(&self.balls_color[i]);
                renderer.set_stroke_weight(0);
                renderer.enable_filling();
                renderer.draw_circle(ball_pos.get_x() as i32, ball_pos.get_y() as i32, ball_radius);
            }

            // On affiche les balles vides.
            else {
                renderer.set_stroke_color(&self.balls_color[i]);
                renderer.set_stroke_weight(self.stroke_weight);
                renderer.disable_filling();
                renderer.draw_circle(ball_pos.get_x() as i32, ball_pos.get_y() as i32, ball_radius - self.stroke_weight as f64);
            }
        }
    }
}
