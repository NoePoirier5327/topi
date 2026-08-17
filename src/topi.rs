mod text;
mod assets;
mod renderer;

struct Topi {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    event_pump: sdl2::EventPump,
    render: renderer::Renderer,
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
            render: renderer::Renderer::new(canvas),
            run: true
        }
    }

    pub fn run(&mut self) {
        while self.run {
            self.event_handler();
            self.update();
            self.display();
        }
    }

    fn event_handler(&mut self) {
        
    }

    fn update(&mut self) {

    }

    fn display(&self) {

    }
}
