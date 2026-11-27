use mlua::prelude::*;
use topi_core::types::color::Color;

// TODO Ajouter le support des couleurs par défaut.

/// Wrapper pour les couleurs de topi vers lua.
pub struct LuaColor(Color);

impl LuaColor {
    /// Construit une nouvelle couleur et la renvoie.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(Color::RGBA(r, g, b, a))
    }

    /// Renvoie la valeur au format topi wrappée dans l'instance courante.
    pub fn as_topi_format(&self) -> Color {
        self.0
    }
}

impl LuaUserData for LuaColor {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, this| Ok(this.0.r));
        fields.add_field_method_get("g", |_, this| Ok(this.0.g));
        fields.add_field_method_get("b", |_, this| Ok(this.0.b));
        fields.add_field_method_get("a", |_, this| Ok(this.0.a));

        fields.add_field_method_set("r", |_, this, r: u8| { this.0.r = r; Ok(()) });
        fields.add_field_method_set("g", |_, this, g: u8| { this.0.g = g; Ok(()) });
        fields.add_field_method_set("b", |_, this, b: u8| { this.0.b = b; Ok(()) });
        fields.add_field_method_set("a", |_, this, a: u8| { this.0.a = a; Ok(()) });
    }
}
