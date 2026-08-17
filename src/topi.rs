use mlua::UserData;
use mlua::prelude::*;
use sdl2::{EventPump, Sdl, VideoSubsystem, render::TextureCreator, video::WindowContext};
use assets::{Assets, AssetId};
use renderer::Renderer;

mod text;
mod assets;
mod renderer;

pub struct TopiEngine {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    renderer: Renderer,

    texture_creator: TextureCreator<WindowContext>,
    assets: Assets,

    event_pump: EventPump,
    run: bool
}

impl TopiEngine {
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
            renderer: Renderer::new(canvas),

            texture_creator,
            assets: Assets::new(),

            event_pump,
            run: true
        }
    }

    pub fn load_texture(&mut self, texture_path: &str, id: &AssetId) {
        self.assets.load_texture(&self.texture_creator, texture_path, id);
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

impl UserData for TopiEngine {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("run", |_lua, this, ()| {
            this.run();
            Ok(())
        });
    }
}
