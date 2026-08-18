use mlua::{FromLua, Value as LuaValue, Result as LuaResult, Error as LuaError};

#[derive(Clone, Copy, Debug)]
pub struct RGBAColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl<'lua> FromLua<'lua> for RGBAColor {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua mlua::Lua) -> LuaResult<Self> {
        match value {
            // Cas 1 : Table associative { r = 255, g = 0, b = 0, a = 255 }
            LuaValue::Table(table) => Ok(RGBAColor {
                r: table.get("r").unwrap_or(0),
                g: table.get("g").unwrap_or(0),
                b: table.get("b").unwrap_or(0),
                a: table.get("a").unwrap_or(255),
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: value.type_name(),
                to: "RGBAColor",
                message: Some("Attendu : table {r, g, b, a}".into()),
            }),
        }
    }
}
