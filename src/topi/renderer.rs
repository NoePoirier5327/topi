use mlua::UserData;
use sdl2::{render::Canvas, video::Window};
use super::color::RGBAColor;

pub enum DrawableItem {
    Sprite {x: i32, y: i32, id: usize},
    ColoredRect {x: i32, y: i32, w: u32, h: u32, color: RGBAColor},
    Text {x: i32, y: i32, content: String},
}

pub struct Renderer {
    queue: Vec<DrawableItem>,
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self {
            queue: Vec::new(),
            canvas,
        }
    }

    pub fn submit(&mut self, item: DrawableItem) {
        self.queue.push(item);
    }

    pub fn flush(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();

        for item in self.queue.drain(..) {
            match item {
                DrawableItem::ColoredRect {x, y, w, h, color} => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
                    let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(x, y, w, h));
                },

                DrawableItem::Text { x, y, content } => { /* TODO */ },
                DrawableItem::Sprite { x, y, id } => { /* TODO */ }
            }
        }

        self.canvas.present();
    }
}

impl UserData for Renderer {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("draw_colored_rect", |_lua, this, (x, y, w, h, color): (i32, i32, u32, u32, RGBAColor)| {
            this.submit(DrawableItem::ColoredRect { x, y, w, h, color });
            Ok(())
        });
    }
}
