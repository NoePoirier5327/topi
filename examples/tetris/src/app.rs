//! Fichier mettant en relation une instance de tetris et les éléments de menu de l'application.

mod game;
use game::{Game, MAP_DISPLAY_X_OFFSET};

pub struct App {
    game: Game,
    is_in_initial_state: bool
}

impl App {
    /// Construit une nouvelle instance de l'application accessible pour topi.
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            is_in_initial_state: true
        }
    }
}

/// Interface générique vers topi.
impl topi::AppLogic for App {
    fn update(&mut self, _dt: f64, input: &topi::InputManager) {
        // Tant qu'on est pas en game over et qu'on n'est pas dans l'état initial de l'appli,
        // on continue de jouer.
        if !self.game.game_over() && !self.is_in_initial_state {
            self.game.handle_inputs(input);
            self.game.update();
        }

        // Sinon si on est en game over et qu'on appuie sur espace, on relance le jeu.
        else if self.game.game_over() && input.is_key_pressed(topi::keycode::KEY_SPACE) {
            self.game = Game::new();
            self.is_in_initial_state = true;
        }

        // Sinon, on est dans l'état initial et on attend une pression quelconque du clavier pour
        // jouer.
        else {
            if input.is_any_key_pressed() {
                self.is_in_initial_state = false;
            }
        }
    }

    fn display(&self, renderer: &mut topi::RenderContext) {
        self.game.display(renderer);

        // On affiche l'information sur l'état initial au joueur.
        if self.is_in_initial_state {
            renderer.enable_filling();
            renderer.set_stroke_weight(0);
            renderer.set_draw_color(&topi::types::color::BLACK);
            renderer.draw_rect(MAP_DISPLAY_X_OFFSET, 256, 320, 128);

            // On affiche le titre du jeu.
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);
            renderer.set_draw_color(&topi::types::color::RED);
            renderer.draw_text("T", 100 + MAP_DISPLAY_X_OFFSET, 285);
            renderer.set_draw_color(&topi::types::color::ORANGE);
            renderer.draw_text("E", 117 + MAP_DISPLAY_X_OFFSET, 285);
            renderer.set_draw_color(&topi::types::color::YELLOW);
            renderer.draw_text("T", 134 + MAP_DISPLAY_X_OFFSET, 285);
            renderer.set_draw_color(&topi::types::color::GREEN);
            renderer.draw_text("R", 151 + MAP_DISPLAY_X_OFFSET, 285);
            renderer.set_draw_color(&topi::types::color::CYAN);
            renderer.draw_text("I", 168 + MAP_DISPLAY_X_OFFSET, 285);
            renderer.set_draw_color(&topi::types::color::BLUE);
            renderer.draw_text("S", 185 + MAP_DISPLAY_X_OFFSET, 285);

            // On informe l'utilisateur qu'il doit appuyer sur n'importe quelle touche pour jouer.
            renderer.set_draw_color(&topi::types::color::WHITE);
            renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 25);
            renderer.draw_text("any key to play", 45 + MAP_DISPLAY_X_OFFSET, 325);
        }
    }
}
