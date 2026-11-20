//! Fichier principal de la librairie topi.
//! Il réexporte uniquement les éléments visibles de l'api (RenderManager, App constructor etc ...).

// Dépendances externes.
extern crate sdl2;
use sdl2::{EventPump, event::Event, mouse::MouseButton};
use std::rc::Rc;

// Modules de la librairie.
mod render;
mod input;
mod resource;
pub mod toolkit;
pub mod types;

// Dépendances internes.
use resource::ResourceManager;

// Exports externes.
pub use render::RenderContext;
pub use input::{InputManager, keycode};

/// Réprésente l'instance d'une application générée par topi.
pub struct App {
    renderer: RenderContext,
    input: InputManager,
    resource: Rc<ResourceManager>,
    event_pump: EventPump,
    game_object: Box<dyn AppLogic>,
}

impl App {
    /// Construit une nouvelle fenêtre de jeu.
    ///
    /// # Arguments
    /// * `title` titre de la nouvelle fenêtre de jeu.
    /// * `width` largeur de la nouvelle fenêtre de jeu.
    /// * `height` hauteur de la nouvelle fenêtre de jeu.
    /// * `game_object` logique de jeu interne à la nouvelle application.
    ///
    /// # Return
    /// Instance vers la nouvelle fenêtre de jeu.
    pub fn new<G: AppLogic + 'static>(title: String, width: u32, height: u32, game_object: G) -> Self {
        // On initialise la sdl2.
        let sdl_context = sdl2::init().expect("Failed to load sdl2 context.");
        let video_subsystem = sdl_context.video().expect("Failed to init the sdl2 video subsystem.");

        // On créer la nouvelle fenêtre.
        let window = video_subsystem
            .window(&title, width, height)
            .position_centered()
            .allow_highdpi()
            .build()
            .expect("Failed to open the app window.");

        // Puis la nouvelle surface de rendue.
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Failed to create sdl2 render canvas.");

        // On active les couleurs transparentes.
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let resource = Rc::new(ResourceManager::new(&canvas));

        Self {
            renderer: render::new_render_context(canvas, width, height, resource.clone()),
            input: input::new_input_manager(),
            resource,
            event_pump: sdl_context.event_pump().expect("Failed to create sdl2 event pump."),
            game_object: Box::new(game_object),
        }
    }

    /// Lance la boucle de jeu.
    pub fn run(&mut self) {
        // Calcul du delta time.
        let target_fps = 60f64;
        let max_dt = 1f64 / target_fps;
        let target_frame_time = std::time::Duration::from_secs_f64(max_dt);
        let mut last_tick = std::time::Instant::now();

        'running: loop {
            let frame_start = std::time::Instant::now();

            let mut last_kb_input = '\0';
            let mut mouse_pos = self.input.get_mouse_pos();
            let mut mouse_left_click = self.input.is_left_mouse_button_held();
            let mut mouse_right_click = self.input.is_right_mouse_button_held();
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,

                    Event::TextInput { text, .. } => last_kb_input = text.chars().next().unwrap_or('\0'),

                    Event::MouseMotion { x, y, .. } => mouse_pos = types::point::Point2i::new(x, y),

                    Event::MouseButtonDown { mouse_btn, .. } => {
                        match mouse_btn {
                            MouseButton::Left => mouse_left_click = true,
                            MouseButton::Right => mouse_right_click = true,
                            _ => {}
                        }
                    }

                    Event::MouseButtonUp { mouse_btn, .. } => {
                        match mouse_btn {
                            MouseButton::Left => mouse_left_click = false,
                            MouseButton::Right => mouse_right_click = false,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            let current_tick = std::time::Instant::now();
            let dt = current_tick.duration_since(last_tick);
            last_tick = current_tick;

            let mut dt_seconds = dt.as_secs_f64();
            if dt_seconds > max_dt {
                dt_seconds = max_dt;
            }

            // On met à jour les buffers d'entrées utilisateur.
            input::update_input_manager(
                &mut self.input,
                last_kb_input,
                mouse_pos,
                mouse_left_click,
                mouse_right_click
            );

            // On lance le code utilisateur de mise à jour.
            self.game_object.update(dt_seconds, &self.input);

            // On lance le code utilisateur d'affichage.
            self.game_object.display(&mut self.renderer);

            render::update_render_context(&mut self.renderer);

            // Limite de l'exécution en fonction du delta time.
            let frame_duration = frame_start.elapsed();
            if frame_duration < target_frame_time {
                std::thread::sleep(target_frame_time - frame_duration);
            }
        }
    }
}

/// Instance de la logique de jeu associée à une instance de App.
pub trait AppLogic {
    /// Interface de mise à jour de la logique de jeu interne.
    ///
    /// # Argument
    /// * `dt` delta time courant à l'instance App sous-jacente.
    /// * `input` gestionnaire d'entrées de la fenêtre de jeu courante.
    fn update(&mut self, dt: f64, input: &InputManager);

    /// Interface d'affichage du jeu courant.
    ///
    /// # Argument
    /// * `renderer` gestionnaire d'affichage de topi.
    fn display(&self, renderer: &mut RenderContext);
}
