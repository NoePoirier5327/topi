use mlua::prelude::*;
use topi_core::RenderContext;

/// Wrapper lua pour le RenderContext de topi.
pub struct LuaRenderContext<'a>(&'a RenderContext);

impl<'a> LuaRenderContext<'a> {
    /// Construit et renvoie un nouveau wrapper sur le renderer en paramètre.
    pub fn new(renderer: &'a RenderContext) -> Self {
        Self(renderer)
    }
}

/// Interface vers lua.
impl<'a> LuaUserData for LuaRenderContext<'a> {

}
