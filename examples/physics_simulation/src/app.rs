//! Fichier contenant la logique de la simulation de physique.

mod ball;

use topi::AppLogic;
use topi::types::point::Point2f;
use topi::types::vector::Vector2D;
use topi::toolkit::rand;

const MIN_BALL_SIZE: f64 = 20f64;
const MAX_BALL_SIZE: f64 = 100f64;

/// Structure contenant les éléments de la simulation.
pub struct App {
    window_width: u32,
    window_height: u32,
    gravity: f64,
    next_ball: ball::Ball,
    balls: Vec<ball::Ball>,
    grabbed_ball: Option<usize>,
    is_in_spawn_mode: bool
}

impl App {
    /// Construit une nouvelle instance vers la simulation.
    /// 
    /// # Arguments
    /// * `window_width` largeur de la fenêtre de simulation courante.
    /// * `window_height` hauteur de la fenêtre de simulation courante.
    pub fn new(window_width: u32, window_height: u32) -> Self {
        Self {
            window_width,
            window_height,
            gravity: 2000f64,
            next_ball: ball::Ball::new(
                0f64, 0f64,
                rand::randrange(MIN_BALL_SIZE, MAX_BALL_SIZE)
            ),
            balls: Vec::new(),
            grabbed_ball: None,
            is_in_spawn_mode: true
        }
    }

    /// Ajoute la prochaine balle aux balles existantes et en construit une nouvelle pour la
    /// prévisualiation.
    /// Ne fait rien si on n'est pas en mode d'apparition de balle.
    fn spawn_new_ball(&mut self) {
        if !self.is_in_spawn_mode { return }

        let new_pos = self.next_ball.pos;
        self.balls.push(self.next_ball);
        self.next_ball = ball::Ball::new(
            new_pos.get_x(),
            new_pos.get_y(),
            rand::randrange(MIN_BALL_SIZE, MAX_BALL_SIZE)
        )
    }

    /// Met à jour les positions des balles ainsi que leurs vecteurs vitesse en fonctions de la
    /// gravitée de la simulation et des collisions entre les balles.
    fn resolve_collisions(&mut self, dt: f64) {
        // Déplacement et collisions murs, sol.
        for ball in &mut self.balls {
            // On calcul la gravitée pour la balle courante.
            ball.speed += Vector2D::new(0.0, self.gravity * dt);

            // On met à jour la position de la balle courante en fonction de son vecteur vitesse.
            ball.update_pos(dt);

            // Si la balle courante atteint le sol, on l'empêche de rentrer dedans et de
            // continuer à tomber.
            let floor_height = self.window_height as f64 - ball.get_radius();
            if ball.pos.get_y() > floor_height {
                ball.pos.set_y(floor_height);
                if ball.speed.get_y() > 0f64 {
                    ball.speed.set_y(0f64);
                }
            }

            // Si la balle courante atteint un bord gauche ou droite, on l'empêchde de sortir.
            if ball.pos.get_x() < ball.get_radius() {
                ball.pos.set_x(ball.get_radius());
                if ball.speed.get_x() < 0f64 {
                    ball.speed.set_x(0f64);
                }
            }

            let left_wall_pos_x = self.window_width as f64 - ball.get_radius();
            if ball.pos.get_x() > left_wall_pos_x {
                ball.pos.set_x(left_wall_pos_x);
                if ball.speed.get_x() > 0f64 {
                    ball.speed.set_x(0f64);
                }
            }
        }

        // Collisions entre balles.
        // constantes de résolution des collisions
        let epsilon = 0.001; // Tolérance dans la détection des collisions de balles.
        let correction_precision = 0.8; // Facteur de correction des positions des balles après collision.
        let restitution = 0.5; // Facteur de restitution des forces de collision pour corriger les vecteurs vitesse.

        let nb_ball = self.balls.len();
        for _ in 0..20 { // <- Affine les calculs de collision.

            // On choisit une balle et on parcours le reste des balles pour vérifier les collisions.
            for i in 0..nb_ball {
                for j in i+1..nb_ball {
                    // On vérifie la collision entre celle courante et les restes des balles.
                    if self.balls[i].is_colliding_with_another_ball(&self.balls[j], epsilon) {
                        // On calcul les vecteurs de corrections.
                        let (pos_correction, speed_correction) = self.balls[i]
                            .calculate_collision_correction(&self.balls[j], correction_precision, restitution, epsilon);

                        // Puis, on les appliques.
                        self.balls[i].correct_pos_and_speed(&(-1f64 * pos_correction), &(-1f64 * speed_correction));
                        self.balls[j].correct_pos_and_speed(&pos_correction, &speed_correction);
                    }
                }
            }
        }
    }

    /// Saisie la balle la plus proche de la souris dans laquelle la souris est et la stoque dans l'instance courante.
    /// Ne fait rien si on est en mode d'apprarition de balle.
    fn grab_closest_ball(&mut self, mouse_pos: &Point2f) {
        if self.is_in_spawn_mode { return }

        let mut closest_ball: Option<usize> = None;
        let mut min_dist = f64::MAX;

        for (i, ball) in self.balls.iter().enumerate() {
            let dist = Point2f::dist(&ball.pos, mouse_pos);
            if dist <= ball.get_radius() && dist < min_dist {
                closest_ball = Some(i);
                min_dist = dist;
            }
        }

        self.grabbed_ball = closest_ball;
    }

    /// Déplace la balle saisie en fonction de la position de la souris.
    /// Ne fait rien si on est en mode d'apparition de balle.
    fn drag_grabbed_ball(&mut self, mouse_pos: &Point2f) {
        if self.is_in_spawn_mode { return }

        if let Some(i) = self.grabbed_ball {
            self.balls[i].pos = *mouse_pos;
            self.balls[i].speed.set_x(0f64);
            self.balls[i].speed.set_y(0f64);
        }
    }

    /// Supprime la balle saisie de la simulation.
    /// Ne fait rien si on est en mode d'apparition de balle, pareille si aucune balle n'est saisie.
    fn destroy_grabbed_ball(&mut self) {
        if self.is_in_spawn_mode { return }

        if let Some(i) = self.grabbed_ball {
            self.balls.remove(i);
        }

        self.grabbed_ball = None;
    }

    /// Relâche la balle saisie par l'utilisateur dans la simulation.
    #[inline]
    fn release_grabbed_ball(&mut self) {
        self.grabbed_ball = None;
    }
}

/// Interface générique vers topi.
impl AppLogic for App {
    fn update(&mut self, dt: f64, input: &topi::InputManager) {
        // On base la position de la prochaine balle à tomber sur celle de la souris.
        let mouse_pos = Point2f::new(input.get_mouse_pos().get_x() as f64, input.get_mouse_pos().get_y() as f64);
        self.next_ball.pos = mouse_pos;

        // On échange de mode en appuyant sur la touche shift.
        if input.is_key_pressed(topi::keycode::KEY_LSHIFT) {
            self.is_in_spawn_mode = !self.is_in_spawn_mode;
        }

        // Si on est pas en mode d'apparition, alors on est mode d'interaction.
        if !self.is_in_spawn_mode {
            // Si on clique en mode de saisie, on attrape la balle la plus proche.
            if input.is_left_mouse_button_clicked() {
                self.grab_closest_ball(&mouse_pos);
            }

            // Si on maintient le clique, on déplace la balle saisie.
            if input.is_left_mouse_button_held() {
                self.drag_grabbed_ball(&mouse_pos);
            }

            // Si on relâche le clique gauche, on arrête de saisir.
            if input.is_left_mouse_button_released() {
                self.release_grabbed_ball();
            }

            // Si on clique droit, on supprime la balle la plus proche de la souris.
            if input.is_right_mouse_button_clicked() {
                self.grab_closest_ball(&mouse_pos);
                self.destroy_grabbed_ball();
            }
        }
        // Sinon, on est en mode d'appraition de balle.
        else {
            // Si on clique gauche dans ce mode, on fait apparaitre une balle.
            if input.is_left_mouse_button_clicked() {
                self.spawn_new_ball();
            }

            // On ne peut pas tenir de balle dans ce mode, on lâche toute éventuelle balle saisie.
            self.release_grabbed_ball();
        }

        // On gère les collisions entre les balles.
        self.resolve_collisions(dt);
    }

    fn display(&self, renderer: &mut topi::RenderContext) {
        // On notifie l'utilisateur du mode dans lequel il est et ce qu'il peut faire en fonction du
        // mode dans lequel il est.
        if self.is_in_spawn_mode {
            renderer.set_draw_color(&topi::types::color::WHITE);
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);

            renderer.draw_text("You are in spawn mode.", 0, 0);
            renderer.draw_text("Click left to spawn a new ball.", 0, 40);
            renderer.draw_text("Press LSHIFT to switch to interactive mode.", 0, 80);
        }
        else {
            renderer.set_draw_color(&topi::types::color::WHITE);
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);

            renderer.draw_text("You are in interactive mode.", 0, 0);
            renderer.draw_text("Click left on a ball to grab and drag it.", 0, 40);
            renderer.draw_text("Click right on a ball to destroy it.", 0, 80);
            renderer.draw_text("Press LSHIFT to switch to spawn mode.", 0, 120);
        }

        // On affiche les balles.
        for ball in &self.balls {
            ball.display(renderer);
        }

        // On affiche la prochaine balle à tomber si on est en mode d'apparition de balle.
        if self.is_in_spawn_mode {
            let mut color = self.next_ball.get_color();
            color.set_a(100);
            renderer.set_draw_color(&color);
            renderer.draw_circle(self.next_ball.pos.get_x() as i32, self.next_ball.pos.get_y() as i32, self.next_ball.get_radius());
        }
    }
}
