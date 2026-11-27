use mlua::prelude::*;
use topi_core::RenderContext;

use crate::types::color::LuaColor;

/// Wrapper lua pour le RenderContext de topi.
pub struct LuaRenderContext<'a>(&'a mut RenderContext);

impl<'a> LuaRenderContext<'a> {
    /// Construit et renvoie un nouveau wrapper sur le renderer en paramètre.
    pub fn new(renderer: &'a mut RenderContext) -> Self {
        Self(renderer)
    }
}

/// Interface vers lua.
impl<'a> LuaUserData for LuaRenderContext<'a> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("drawRect", |_, this, (x, y, width, height): (i32, i32, u32, u32)| {
            this.0.draw_rect(x, y, width, height);
            Ok(())
        });

        methods.add_method_mut("drawCircle", |_, this, (x, y, radius): (i32, i32, f64)| {
            this.0.draw_circle(x, y, radius);
            Ok(())
        });

        methods.add_method_mut("drawLine", |_, this, (x1, y1, x2, y2, thickness): (i32, i32, i32, i32, u32)| {
            this.0.draw_line(x1, y1, x2, y2, thickness);
            Ok(())
        });

        methods.add_method_mut("drawText", |_, this, (text, x, y): (String, i32, i32)| {
            this.0.draw_text(&text, x, y);
            Ok(())
        });

        methods.add_method_mut("drawImage", |_, this, (path, x, y): (String, i32, i32)| {
            this.0.draw_image(&path, x, y);
            Ok(())
        });

        methods.add_method_mut("drawScaledImage", |_, this, (path, x, y, scale): (String, i32, i32, f64)| {
            this.0.draw_scaled_image(&path, x, y, scale);
            Ok(())
        });

        // TODO Ajouter support pour draw_image_part

        methods.add_method_mut("setDrawColor", |_, this, color: LuaAnyUserData| {
            let color = color.borrow::<LuaColor>()?;
            this.0.set_draw_color(&color.as_topi_format());
            Ok(())
        });

        methods.add_method_mut("setStrokeColor", |_, this, color: LuaAnyUserData| {
            let color = color.borrow::<LuaColor>()?;
            this.0.set_stroke_color(&color.as_topi_format());
            Ok(())
        });

        methods.add_method_mut("setStrokeWeight", |_, this, weight: u32| {
            this.0.set_stroke_weight(weight);
            Ok(())
        });

        methods.add_method_mut("enableFilling", |_, this, _: ()| {
            this.0.enable_filling();
            Ok(())
        });

        methods.add_method_mut("disableFilling", |_, this, _: ()| {
            this.0.disable_filling();
            Ok(())
        });

        methods.add_method_mut("setFont", |_, this, (font, font_size): (String, u16)| {
            this.0.set_font(&font, font_size);
            Ok(())
        });
    }
}
