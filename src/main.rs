mod stubs;
mod topi;

use topi::TopiEngine;
use topi::commands::{CommandProcessor, CommandType};
use topi::renderer::Renderer;
use topi::color::RGBAColor;

crate::impl_lua_class_stub! {
    class TopiEngine {
        methods {
            fn run() -> "nil";
            fn commands() -> "CommandProcessor";
        }
    }
}

crate::impl_lua_class_stub! {
    class CommandProcessor {
        methods {
            fn new_anonym_task(cmd_type: "CommandType", func: "fun(dt?: number)") -> "nil";
            fn new_drawing_task(func: "fun(renderer: Renderer)") -> "nil";
            fn clear() -> "nil";
        }
    }
}

crate::impl_lua_class_stub! {
    class Renderer {
        methods {
            fn draw_colored_rect(x: "number", y: "number", w: "number", h: "number", color: "RGBAColor") -> "nil";
        }
    }
}

crate::impl_lua_class_stub! {
    class RGBAColor {
        fields {
            r: "number",
            g: "number",
            b: "number",
            a: "number"
        }
    }
}

crate::impl_lua_enum_stub! {
    enum CommandType {
        Setup,
        Update,
    }
}

fn main() {
    let mut stub = String::new();
    stub.push_str("---@meta topi\n\n");

    stub.push_str(&CommandType::generate_stub());
    stub.push_str(&CommandProcessor::generate_stub());
    stub.push_str(&RGBAColor::generate_stub());
    stub.push_str(&Renderer::generate_stub());
    stub.push_str(&TopiEngine::generate_stub());

    stub.push_str("---@class Topi\n");
    stub.push_str("local topi = {}\n\n");
    stub.push_str("--- Initialise une fenêtre de jeu.\n");
    stub.push_str("---@param title string\n");
    stub.push_str("---@param width number\n");
    stub.push_str("---@param height number\n");
    stub.push_str("---@return TopiEngine\n");
    stub.push_str("function topi.init(title, width, height) end\n\n");
    stub.push_str("return topi");

    println!("{}", stub);
    std::fs::write("stubs/topi.lua", stub).unwrap();
}
