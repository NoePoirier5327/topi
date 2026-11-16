//! Fichiers contenant l'interfacage vers les éléments affichables par RenderManager.

pub mod rectangle;
pub mod line;
pub mod circle;
pub mod text;

use sdl2::render::WindowCanvas;

/// Indique au moteur que l'instance qu'il manipule est affichable par le RenderManager.
pub trait Renderable {
    fn display(&self, canvas: &mut WindowCanvas);
}
