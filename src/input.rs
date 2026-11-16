//! Fichier contenant l'interface vers le gestionnaire d'entrées du moteur topi (entrées clavier,
//! mannettes, souris ...).

pub mod keycode;

use sdl2::sys::{SDL_GetKeyboardState, SDL_GetScancodeFromKey, SDL_PumpEvents, SDL_Scancode::SDL_NUM_SCANCODES};
use crate::types::point::Point2i;

/// Wrapper de construction d'un nouveau gestionnaire d'entrées pour une instance de topi::App.
pub fn new_input_manager() -> InputManager {
    InputManager::new()
}

/// Wrapper de mise à jour des InputManager.
pub fn update_input_manager(
    input: &mut InputManager,
    last_kb_input: char,
    mouse_pos: Point2i,
    mouse_left_click_state: bool,
    mouse_right_click_state: bool
) {
    input.update(
        last_kb_input,
        mouse_pos,
        mouse_left_click_state,
        mouse_right_click_state
    );
}

/// Gestionnaire d'entrées du moteur topi.
pub struct InputManager {
    current_state: [u8; SDL_NUM_SCANCODES as usize],
    previous_state: [u8; SDL_NUM_SCANCODES as usize],
    last_kb_input: char,
    mouse_pos: Point2i,
    current_mouse_left_click_state: bool,
    current_mouse_right_click_state: bool,
    previous_mouse_left_click_state: bool,
    previous_mouse_right_click_state: bool
}

impl InputManager {
    /// Construit un nouveau gestionnaire d'entrées associée à une instance de topi::App.
    fn new() -> Self {
        let mut current_state = [0u8; SDL_NUM_SCANCODES as usize];

        unsafe {
            SDL_PumpEvents();
            let ptr = SDL_GetKeyboardState(std::ptr::null_mut());
            let states = std::slice::from_raw_parts(ptr, SDL_NUM_SCANCODES as usize);
            current_state.copy_from_slice(states);
        };

        Self {
            current_state,
            previous_state: [0u8; SDL_NUM_SCANCODES as usize],
            last_kb_input: '\0',
            mouse_pos: Point2i::new(0, 0),
            current_mouse_left_click_state: false,
            current_mouse_right_click_state: false,
            previous_mouse_left_click_state: false,
            previous_mouse_right_click_state: false
        }
    }

    /// Met à jour l'état du gestionnaire d'entrées courant.
    /// Est appelé via un wrapper dans son instance de topi::App sous-jacente.
    ///
    /// # Arguments
    /// * `last_kb_input` dernière entrée clavier sous forme de caractère imprimable.
    /// * `mouse_pos` position de la souris sur la fenêtre courante.
    /// * `mouse_left_click_state` informe sur le fait que le bouton gauche de la souris est cliqué ou non.
    /// * `mouse_right_click_state` informe sur le fait que le bouton droit de la souris est cliqué ou non.
    fn update(
        &mut self,
        last_kb_input: char,
        mouse_pos: Point2i,
        mouse_left_click_state: bool,
        mouse_right_click_state: bool
    ) {
        self.previous_state.copy_from_slice(&self.current_state);
        self.previous_mouse_left_click_state = self.current_mouse_left_click_state;
        self.previous_mouse_right_click_state = self.current_mouse_right_click_state;

        unsafe {
            SDL_PumpEvents();
            let ptr = SDL_GetKeyboardState(std::ptr::null_mut());
            let states = std::slice::from_raw_parts(ptr, SDL_NUM_SCANCODES as usize);
            self.current_state.copy_from_slice(states);
        }

        self.last_kb_input = last_kb_input;
        self.mouse_pos = mouse_pos;
        self.current_mouse_left_click_state = mouse_left_click_state;
        self.current_mouse_right_click_state = mouse_right_click_state;
    }

    /// Vérifie si la touche en paramètre est maintenue par l'utilisateur.
    #[inline]
    pub fn is_key_held(&self, input: keycode::Keycode) -> bool {
        let scancode = unsafe { SDL_GetScancodeFromKey(input as i32) };
        self.current_state[scancode as usize] != 0
    }

    /// Vérifie si la touche en paramètre est pressée par l'utilisateur.
    #[inline]
    pub fn is_key_pressed(&self, input: keycode::Keycode) -> bool {
        let scancode = unsafe { SDL_GetScancodeFromKey(input as i32) };
        self.current_state[scancode as usize] != 0 && self.previous_state[scancode as usize] == 0
    }

    /// Renvoie la dernière entrée clavier du gestionnaire d'entrées sous forme de caractère
    /// imprimable.
    #[inline]
    pub fn get_last_kb_input(&self) -> char {
        self.last_kb_input
    }

    /// Renvoie la position courante de la souris sur la fenêtre sous-jacente.
    #[inline]
    pub fn get_mouse_pos(&self) -> Point2i {
        self.mouse_pos
    }

    /// Vérifie si un bouton de la souris est cliqué ou non.
    #[inline]
    pub fn is_any_mouse_button_clicked(&self) -> bool {
        self.is_left_mouse_button_clicked() || self.is_right_mouse_button_clicked()
    }

    /// Vérifie si le bouton gauche de la souris est cliqué.
    #[inline]
    pub fn is_left_mouse_button_clicked(&self) -> bool {
        !self.previous_mouse_left_click_state && self.current_mouse_left_click_state
    }

    /// Véfifie si le bouton droit de la souris est cliqué.
    #[inline]
    pub fn is_right_mouse_button_clicked(&self) -> bool {
        !self.previous_mouse_right_click_state && self.current_mouse_right_click_state
    }

    /// Vérifie si n'importe quel bouton de la souris est maintenu.
    #[inline]
    pub fn is_any_mouse_button_held(&self) -> bool {
        self.is_left_mouse_button_held() || self.is_right_mouse_button_held()
    }

    /// Vérifie si le bouton gauche de la souris est maintenu.
    #[inline]
    pub fn is_left_mouse_button_held(&self) -> bool {
        self.current_mouse_left_click_state
    }

    /// Vérifie si le bouton droit de la souris est maintenu.
    #[inline]
    pub fn is_right_mouse_button_held(&self) -> bool {
        self.current_mouse_right_click_state
    }

    /// Vérifie si le bouton gauche de la souris est cliqué.
    #[inline]
    pub fn is_left_mouse_button_released(&self) -> bool {
        !self.current_mouse_left_click_state
    }

    /// Vérifie si le bouton droit de la souris est relaché.
    #[inline]
    pub fn is_right_mouse_button_released(&self) -> bool {
        !self.current_mouse_right_click_state
    }
}
