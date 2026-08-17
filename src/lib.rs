mod topi;

use mlua::prelude::*;

use crate::topi::TopiEngine;

#[mlua::lua_module]
fn topi(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("init", lua.create_function(|_, (title, width, height): (String, u32, u32)| {
        println!("Welcome to topi engine, a game engine written in rust.");
        Ok(TopiEngine::new(title.as_str(), width, height))
    })?)?;

    Ok(exports)
}
