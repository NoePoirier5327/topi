use mlua::prelude::*;
use topi_core::InputManager;

/// Wrapper lua pour le InputManager de topi.
pub struct LuaInputManager<'a>(&'a InputManager);

impl<'a> LuaInputManager<'a> {
    /// Construit un wrapper sur le input_manager en paramètre et le renvoie.
    #[inline]
    pub fn new(input_manager: &'a InputManager) -> Self {
        Self(input_manager)
    }
}

/// Interface vers lua.
impl<'a> LuaUserData for LuaInputManager<'a> {

}
