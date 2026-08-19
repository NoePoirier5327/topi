use mlua::UserData;
use sdl2::{pixels::Color, rect::FPoint, render::{Canvas, Vertex, VertexIndices}, video::Window};
use super::color::RGBAColor;

pub enum DrawableItem {
    Sprite {x: i32, y: i32, id: usize},
    FilledColoredTriangle {x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: RGBAColor},
    FilledColoredRect {x: i32, y: i32, w: u32, h: u32, color: RGBAColor},
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
                DrawableItem::FilledColoredTriangle { x1, y1, x2, y2, x3, y3, color } => {
                    let sdl_color = Color::RGBA(color.r, color.g, color.b, color.a);
                    let vertices = [
                        Vertex {
                            position: FPoint::new(x1 as f32, y1 as f32),
                            color: sdl_color,
                            tex_coord: FPoint::new(0.0, 0.0)
                        },

                        Vertex {
                            position: FPoint::new(x2 as f32, y2 as f32),
                            color: sdl_color,
                            tex_coord: FPoint::new(0.0, 0.0)
                        },

                        Vertex {
                            position: FPoint::new(x3 as f32, y3 as f32),
                            color: sdl_color,
                            tex_coord: FPoint::new(0.0, 0.0)
                        }
                    ];

                    let _ = self.canvas.render_geometry(&vertices, None, VertexIndices::Sequential);
                }

                DrawableItem::FilledColoredRect {x, y, w, h, color} => {
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
        methods.add_method_mut("draw_filled_colored_triangle", |_lua, this, (x1, y1, x2, y2, x3, y3, color) : (i32, i32, i32, i32, i32, i32, RGBAColor)| {
            this.submit(DrawableItem::FilledColoredTriangle { x1, y1, x2, y2, x3, y3, color });
            Ok(())
        });

        methods.add_method_mut("draw_filled_colored_rect", |_lua, this, (x, y, w, h, color): (i32, i32, u32, u32, RGBAColor)| {
            this.submit(DrawableItem::FilledColoredRect { x, y, w, h, color });
            Ok(())
        });
    }
}
