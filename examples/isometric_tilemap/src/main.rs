use topi::types::rect::Rect;
use std::collections::HashMap;

fn main() {
    topi::App::new(
        "Example - Isometric tileset",
        1500, 600,
        App::new()
    ).run();
}

/// Structure passée à topi et exécutée à chaque tour de boucles.
/// Contient la tilemap sous forme statique affichée pour l'exemple.
struct App {
    tilemap: String,
    map_width: u32,
    _map_height: u32,
    tileset_path: String,
    tileset: HashMap<char, Rect>,
    camera_x: i32,
    camera_y: i32,
    camera_zoom: f64,
    camera_speed: f64
}

impl App {
    /// Construit une nouvelle instance de l'app manipulée par topi, il charge en mémoire la tilemap
    /// affichée pour l'exemple.
    pub fn new() -> Self {
        let tilemap =  "ggggggggggssssgggsswwssgggssssgggggggggg".to_string();

        let mut tileset: HashMap<char, Rect> = HashMap::new();
        tileset.insert('g', Rect::new(0, 0, 32, 32));
        tileset.insert('s', Rect::new(32, 0, 32, 32));
        tileset.insert('w', Rect::new(64, 0, 32, 32));

        Self {
            tilemap,
            map_width: 8,
            _map_height: 5,
            tileset_path: "./tileset.png".to_string(),
            tileset,
            camera_x: -148,
            camera_y: -10,
            camera_zoom: 4f64,
            camera_speed: 200f64
        }
    }
}

impl topi::AppLogic for App {
    fn update(&mut self, dt: f64, input: &topi::InputManager) {
        if input.is_key_held(topi::keycode::KEY_Z) {
            self.camera_y += (self.camera_speed * dt) as i32;
        }

        if input.is_key_held(topi::keycode::KEY_S) {
            self.camera_y -= (self.camera_speed * dt) as i32;
        }

        if input.is_key_held(topi::keycode::KEY_D) {
            self.camera_x -= (self.camera_speed * dt) as i32;
        }

        if input.is_key_held(topi::keycode::KEY_Q) {
            self.camera_x += (self.camera_speed * dt) as i32;
        }

        if input.is_key_held(topi::keycode::KEY_O) {
            self.camera_zoom += 0.1;
        }

        if input.is_key_held(topi::keycode::KEY_P) && self.camera_zoom - 0.1 > 0f64 {
            self.camera_zoom -= 0.1;
        }
    }

    fn display(&self, renderer: &mut topi::RenderContext) {
        // On affiche la position et le taux de zoom de la caméra sur l'application.
        renderer.set_draw_color(&topi::types::color::WHITE);
        renderer.set_font("./JetBrainsMonoNerdFont-Bold.ttf", 30);
        renderer.draw_text("Camera", 0, 0);
        renderer.draw_text(&format!("x: {} y: {}", self.camera_x, self.camera_y), 0, 30);
        renderer.draw_text(&format!("zoom: {}", self.camera_zoom), 0, 60);

        // On affiche le tilemap isométrique.
        let tile_w = 32.0;
        let tile_h = 16.0;

        for (i, tile) in self.tilemap.chars().enumerate() {
            let x = (i as u32 % self.map_width) as f64;
            let y = (i as u32 / self.map_width) as f64;

            let iso_x = (x - y) * (tile_w / 2.0);
            let iso_y = (x + y) * (tile_h / 2.0);

            let screen_x = ((iso_x - self.camera_x as f64) * self.camera_zoom) as i32;
            let screen_y = ((iso_y - self.camera_y as f64) * self.camera_zoom) as i32;

            if let Some(src) = self.tileset.get(&tile) {
                renderer.draw_image_part(
                    &self.tileset_path,
                    screen_x,
                    screen_y,
                    self.camera_zoom,
                    src
                );
            }
        }
    }
}
