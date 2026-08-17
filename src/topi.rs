mod text;
mod assets;
mod renderer;

pub struct Topi {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    event_pump: sdl2::EventPump,
    renderer: renderer::Renderer,
    run: bool
}

impl Topi {
    pub fn new(window_name: &str, window_width: u32, window_height: u32) -> Self {
        let sdl_context = sdl2::init().expect("Failed to load sdl2.");
        let video_subsystem = sdl_context.video().expect("Failed to load video subsystem.");
        let window = video_subsystem.window(window_name, window_width, window_height).position_centered().build()
            .expect("Failed to load sdl2 window.");
        let canvas = window.clone().into_canvas().build().expect("Failed to load render surface.");
        let texture_creator = canvas.texture_creator();
        let event_pump = sdl_context.event_pump().unwrap();

        Self {
            sdl_context,
            video_subsystem,
            texture_creator,
            event_pump,
            renderer: renderer::Renderer::new(canvas),
            run: true
        }
    }

    pub fn get_renderer(&mut self) -> &renderer::Renderer {
        &self.renderer
    }

    pub fn run(&mut self) {
        while self.run {
            self.event_handler();
            self.update();
            self.renderer.flush();
        }
    }

    fn event_handler(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => { self.run = false; },
                _ => {  }
            }
        }
    }

    fn update(&mut self) {
        
    }
}
