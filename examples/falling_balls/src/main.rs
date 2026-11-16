//! Fichier principal de la simulation de physique.
//! Quand on clique sur la fenêtre on fait apparaître une balle soumise à une certaine attraction et
//! pouvant entrer en collision avec les autres balles de la fenêtre.

mod app;

const WINDOW_WIDTH: u32 = 1500;
const WINDOW_HEIGHT: u32 = 1200;

fn main() {
    topi::App::new(
        "Example - Falling balls".to_string(),
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        app::App::new(WINDOW_WIDTH, WINDOW_HEIGHT)
    ).run();
}
