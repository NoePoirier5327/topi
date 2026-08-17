pub enum DrawableItem {
    Sprite {x: i32, y: i32, id: usize},
    Rect {x: i32, y: i32, w: usize, h: usize},
    Text {x: i32, y: i32, content: String},
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
                Dr
            }
        }
    }
}
