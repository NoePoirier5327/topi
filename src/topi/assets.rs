use std::collections::HashMap;
use sdl2::{image::LoadTexture, rect::Rect, render::{Canvas, Texture, TextureCreator}, video::{Window, WindowContext}};

pub type AssetId = u32;

pub struct Assets {
    textures: HashMap<AssetId, Texture<'static>>
}

impl Assets {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new()
        }
    }

    pub fn render(
        &self,
        canvas: &mut Canvas<Window>,
        src: Option<Rect>,
        dst: Rect,
        id: &AssetId
    ) {
        assert!(self.textures.contains_key(id), "Failed to render texture.");
        let _ = canvas.copy(&self.textures[id], src, dst);
    }

    pub fn load_texture(
        &mut self,
        texture_creator: &TextureCreator<WindowContext>,
        texture_path: &str,
        id: &AssetId
    ) {
        assert!(!self.textures.contains_key(id), "Failed to load new texture, id already exists.");
        let texture = texture_creator.load_texture(texture_path).expect("Failed to load new texture.");
        let static_texture: Texture<'static> = unsafe { std::mem::transmute(texture) };
        self.textures.insert(*id, static_texture).unwrap();
    }
}
