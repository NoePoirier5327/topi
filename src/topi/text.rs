//! Module d'affichage de texte sur une fenêtre sdl2. <br>
//! le 26/05/2026 <br>
//! par Noé Poirier

use std::collections::HashMap;
use std::sync::LazyLock;

use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::Canvas;
use sdl2::video::Window;


static ALPHABET_TEXTURE_SHAPE: LazyLock<HashMap<char, Rect>> = LazyLock::new(|| {
    let mut temp = HashMap::new();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for (i, chr) in alphabet.char_indices() {
        let pos = Rect::new((i as i32) * 5, 0, 5, 6);
        temp.insert(chr, pos);
    }

    let alphabet = "abcdefghiklmnopqrstuvwxyz".to_string();
    for (i, chr) in alphabet.char_indices() {
        let pos = Rect::new((i as i32) * 5, 6, 5, 6);
        temp.insert(chr, pos);
    }

    let alphabet = "0123456789 .,;:?!=+-_*|/\\".to_string();
    for (i, chr) in alphabet.char_indices() {
        let pos = Rect::new((i as i32) * 5, 12, 5, 6);
        temp.insert(chr, pos);
    }

    temp
});

/// Fonction d'affichage de texte sur un affichage sdl2.
///
/// # Arguments
/// * `text`: texte à afficher.
/// * `pos_x`: abcisse sur lequel afficher le texte.
/// * `pos_y`; ordonnée sur laquelle afficher le texte.
/// * `text_size`: facteur d'agrandissement du texte à afficher.
/// * `canvas`: fenêtre sur laquelle afficher le texte.
/// * `alphabet_texture`: texture de l'alphabet servant à l'affichage
pub fn display(text: String, pos_x: i32, pos_y: i32, text_size: u32, canvas: &mut Canvas<Window>, alphabet_texture: &Texture) {
    for (i, chr) in text.char_indices() {
        assert!(ALPHABET_TEXTURE_SHAPE.contains_key(&chr), "Aucune texture associée au caractère {chr}.");

        let width = ALPHABET_TEXTURE_SHAPE[&chr].width() * text_size;
        let height = ALPHABET_TEXTURE_SHAPE[&chr].height() * text_size;
        let text_x = ((i as u32) * width) as i32 + pos_x;
        
        let dest_rect = Rect::new(text_x, pos_y, width, height);

        let _ = canvas.copy(alphabet_texture, ALPHABET_TEXTURE_SHAPE[&chr], dest_rect);
    }
}
