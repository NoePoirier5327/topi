//! Fichier contenant les bindings vers les entrées utilisateur.

use sdl2::sys::SDL_KeyCode::*;

/// Type générique des touches manipulées par topi.
pub type Keycode = u32;

pub const KEY_RETURN: Keycode = SDLK_RETURN as Keycode;
pub const KEY_ESCAPE: Keycode = SDLK_ESCAPE as Keycode;
pub const KEY_BACKSPACE: Keycode = SDLK_BACKSPACE as Keycode;
pub const KEY_TAB: Keycode = SDLK_TAB as Keycode;
pub const KEY_RSHIFT: Keycode = SDLK_RSHIFT as Keycode;
pub const KEY_LSHIFT: Keycode = SDLK_LSHIFT as Keycode;
pub const KEY_CAPSLOCK: Keycode = SDLK_CAPSLOCK as Keycode;
pub const KEY_SPACE: Keycode = SDLK_SPACE as Keycode;
pub const KEY_RCTRL: Keycode = SDLK_RCTRL as Keycode;
pub const KEY_LCTRL: Keycode = SDLK_LCTRL as Keycode;

pub const KEY_EXCLAM: Keycode = SDLK_EXCLAIM as Keycode;
pub const KEY_AT: Keycode = SDLK_AT as Keycode;
pub const KEY_HASH: Keycode = SDLK_HASH as Keycode;
pub const KEY_DOLLAR: Keycode = SDLK_DOLLAR as Keycode;
pub const KEY_PERCENT: Keycode = SDLK_PERCENT as Keycode;
pub const KEY_CARET: Keycode = SDLK_CARET as Keycode;
pub const KEY_AMPERSAND: Keycode = SDLK_AMPERSAND as Keycode;
pub const KEY_ASTERISK: Keycode = SDLK_ASTERISK as Keycode;
pub const KEY_RPAREN: Keycode = SDLK_RIGHT as Keycode;
pub const KEY_LPAREN: Keycode = SDLK_LEFTPAREN as Keycode;
pub const KEY_MINUS: Keycode = SDLK_MINUS as Keycode;
pub const KEY_UNDERSCORE: Keycode = SDLK_UNDERSCORE as Keycode;
pub const KEY_PLUS: Keycode = SDLK_PLUS as Keycode;
pub const KEY_EQUAL: Keycode = SDLK_EQUALS as Keycode;
pub const KEY_RBRACKET: Keycode = SDLK_RIGHTBRACKET as Keycode;
pub const KEY_LBRACKET: Keycode = SDLK_LEFTBRACKET as Keycode;
pub const KEY_SEMICOLON: Keycode = SDLK_SEMICOLON as Keycode;
pub const KEY_COLON: Keycode = SDLK_COLON as Keycode;
pub const KEY_SINGLEQUOTE: Keycode = SDLK_QUOTE as Keycode;
pub const KEY_DOUBLEQUOTE: Keycode = SDLK_QUOTEDBL as Keycode;
pub const KEY_BACKQUOTE: Keycode = SDLK_BACKQUOTE as Keycode;
pub const KEY_COMMA: Keycode = SDLK_COMMA as Keycode;
pub const KEY_DOT: Keycode = SDLK_PERIOD as Keycode;
pub const KEY_SLASH: Keycode = SDLK_SLASH as Keycode;
pub const KEY_BACKSLASH: Keycode = SDLK_BACKSLASH as Keycode;
pub const KEY_LESS: Keycode = SDLK_LESS as Keycode;
pub const KEY_GREATER: Keycode = SDLK_GREATER as Keycode;
pub const KEY_QUESTION: Keycode = SDLK_QUESTION as Keycode;

pub const KEY_0: Keycode = SDLK_0 as Keycode;
pub const KEY_1: Keycode = SDLK_1 as Keycode;
pub const KEY_2: Keycode = SDLK_2 as Keycode;
pub const KEY_3: Keycode = SDLK_3 as Keycode;
pub const KEY_4: Keycode = SDLK_4 as Keycode;
pub const KEY_5: Keycode = SDLK_5 as Keycode;
pub const KEY_6: Keycode = SDLK_6 as Keycode;
pub const KEY_7: Keycode = SDLK_7 as Keycode;
pub const KEY_8: Keycode = SDLK_8 as Keycode;
pub const KEY_9: Keycode = SDLK_9 as Keycode;

pub const KEY_A: Keycode = SDLK_a as Keycode;
pub const KEY_B: Keycode = SDLK_b as Keycode;
pub const KEY_C: Keycode = SDLK_c as Keycode;
pub const KEY_D: Keycode = SDLK_d as Keycode;
pub const KEY_E: Keycode = SDLK_e as Keycode;
pub const KEY_F: Keycode = SDLK_f as Keycode;
pub const KEY_G: Keycode = SDLK_g as Keycode;
pub const KEY_H: Keycode = SDLK_h as Keycode;
pub const KEY_I: Keycode = SDLK_i as Keycode;
pub const KEY_J: Keycode = SDLK_j as Keycode;
pub const KEY_K: Keycode = SDLK_k as Keycode;
pub const KEY_L: Keycode = SDLK_l as Keycode;
pub const KEY_M: Keycode = SDLK_m as Keycode;
pub const KEY_N: Keycode = SDLK_n as Keycode;
pub const KEY_O: Keycode = SDLK_o as Keycode;
pub const KEY_P: Keycode = SDLK_p as Keycode;
pub const KEY_Q: Keycode = SDLK_q as Keycode;
pub const KEY_R: Keycode = SDLK_r as Keycode;
pub const KEY_S: Keycode = SDLK_s as Keycode;
pub const KEY_T: Keycode = SDLK_t as Keycode;
pub const KEY_U: Keycode = SDLK_u as Keycode;
pub const KEY_V: Keycode = SDLK_v as Keycode;
pub const KEY_W: Keycode = SDLK_w as Keycode;
pub const KEY_X: Keycode = SDLK_x as Keycode;
pub const KEY_Y: Keycode = SDLK_y as Keycode;
pub const KEY_Z: Keycode = SDLK_z as Keycode;

pub const KEY_UP: Keycode = SDLK_UP as Keycode;
pub const KEY_DOWN: Keycode = SDLK_DOWN as Keycode;
pub const KEY_LEFT: Keycode = SDLK_RIGHT as Keycode;
pub const KEY_RIGHT: Keycode = SDLK_LEFT as Keycode;
