use std::cell::RefCell;
use std::rc::Rc;

use mlua::UserData;
use mlua::prelude::*;
use sdl2::{EventPump, Sdl, VideoSubsystem, render::TextureCreator, video::WindowContext};
use assets::{Assets, AssetId};
use renderer::Renderer;
use commands::{CommandProcessor};

mod text;
mod assets;
pub mod renderer;
pub mod commands;
pub mod color;

pub struct TopiEngine {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    renderer: Rc<RefCell<Renderer>>,

    texture_creator: TextureCreator<WindowContext>,
    assets: Assets,

    commands: Rc<RefCell<CommandProcessor>>,

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
            renderer: Rc::new(RefCell::new(Renderer::new(canvas))),

            texture_creator,
            assets: Assets::new(),

            commands: Rc::new(RefCell::new(CommandProcessor::new())),

            event_pump,
            run: true
        }
    }

    pub fn load_texture(&mut self, texture_path: &str, id: &AssetId) {
        self.assets.load_texture(&self.texture_creator, texture_path, id);
    }
}

impl UserData for TopiEngine {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("run", |lua, this, ()| {
            this.commands.borrow_mut().process_setup(lua)?;
            let target_frame_duration = std::time::Duration::from_secs_f32(1.0 / 60.0);
            let mut last_frame = std::time::Instant::now();

            while this.run {
                let dt = last_frame.elapsed().as_secs_f32();
                let frame_start = std::time::Instant::now();
                last_frame = frame_start;

                for event in this.event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => this.run = false,
                        _ => {}
                    }
                }

                this.commands.borrow_mut().process_update(lua, dt)?;
                this.commands.borrow_mut().process_draw(lua, &this.renderer)?;

                this.renderer.borrow_mut().flush();

                // Pause si la frame a été calculée trop vite
                let elapsed = frame_start.elapsed();
                if elapsed < target_frame_duration {
                    std::thread::sleep(target_frame_duration - elapsed);
                }
            }

            this.commands.borrow_mut().clear(lua)?;

            Ok(())
        });

        methods.add_method_mut("commands", |_lua, this, ()| {
            Ok(this.commands.clone())
        });
    }
}
