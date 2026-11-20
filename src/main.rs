//! Programme de test des fonctionnalitées implémentées dans topi.

use topi::{App, AppLogic, RenderContext, InputManager};
use topi::types::color::Color;

fn main() {
    App::new(
        "Test",
        600, 600,
        AppTest
    ).run();
}

struct AppTest;

impl AppLogic for AppTest {
    fn update(&mut self, _dt: f64, _input: &InputManager) { }

    fn display(&self, renderer: &mut RenderContext) {
        renderer.set_stroke_color(&Color::new_rgba(150, 150, 255, 255));
        renderer.set_draw_color(&Color::new_rgba(150, 255, 150, 255));
        renderer.set_stroke_weight(20);
        renderer.draw_rect(100, 100, 100, 100);
        renderer.draw_circle(200, 200, 50f64);
        renderer.draw_line(50, -100, 50, 700, 10);
    }
}
