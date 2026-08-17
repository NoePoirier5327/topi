pub enum DrawableItem {
    Sprite {x: i32, y: i32, id: usize},
    Rect {x: i32, y: i32, w: u32, h: u32, color: RGBColor},
    Text {x: i32, y: i32, content: String},
}

pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Renderer {
    queue: Vec<DrawableItem>,
    canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl Renderer {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self {
            queue: Vec::new(),
            canvas
        }
    }

    pub fn submit(&mut self, item: DrawableItem) {
        self.queue.push(item);
    }

    pub fn flush(&mut self) {
        for item in self.queue.drain(..) {
            match item {
                DrawableItem::Rect {x, y, w, h, color} => {
                    self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(color.r, color.g, color.b, color.a));
                    let _ = self.canvas.fill_rect(sdl2::rect::Rect::new(x, y, w, h));
                },

                DrawableItem::Text { x, y, content } => { /* TODO */ },
                DrawableItem::Sprite { x, y, id } => { /* TODO */ }
            }

            self.canvas.present();
        }
    }
}
