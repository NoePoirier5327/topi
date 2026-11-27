mod input;
mod render;

use input::LuaInputManager;
use render::LuaRenderContext;

use mlua::prelude::*;
use topi_core::{App, AppLogic, InputManager, RenderContext};

/// Point d'entré vers le module lua.
#[mlua::lua_module]
fn topi(lua: &Lua) -> LuaResult<LuaTable> {
    // Table d'accès global à l'export global.
    let exports = lua.create_table()?;

    // Tables contenants les points d'accès vers les composantes de topi.
    let app_export = lua.create_table()?;

    app_export.set(
        "new",
        lua.create_function(|lua, (title, width, height, table): (String, u32, u32, LuaTable)| {
            let logic = LuaAppLogic::new(lua, table)?;
            let app = App::new(&title, width, height, logic);
            Ok(LuaApp(app))
        })?
    )?;

    // Accès dans l'export global.
    exports.set("App", app_export)?;

    // Renvoie de l'export lua.
    Ok(exports)
}

/// Version lua du App de topi_core.
struct LuaApp(App);

/// Interface d'accès générique à lua pour l'app.
impl LuaUserData for LuaApp {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Accès à la méthode de lancement de l'application.
        methods.add_method_mut("run", |_, this, _: ()| {
            this.0.run();
            Ok(())
        });
    }
}

/// Interface vers l'instance de la logique du jeu sous-jacente à l'App.
struct LuaAppLogic {
    lua: Lua,
    table: LuaTable,
    update_func: LuaFunction,
    display_func: LuaFunction
}

impl LuaAppLogic {
    /// Construit un nouvel à une logique de jeu pour App.
    /// L'accès se fait sous forme de ciblage de fonctions de table lua.
    ///
    /// # Arguments
    /// * `lua` référence vers l'instance lua courante.
    /// * `table` table lua contenant la logique.
    ///
    /// # Return
    /// Instance vers la logique de jeu compatible avec lua.
    fn new(lua: &Lua, table: LuaTable) -> LuaResult<Self> {
        let update_func = table.get("update")?;
        let display_func = table.get("display")?;

        Ok(Self {
            lua: lua.clone(),
            table,
            update_func,
            display_func,
        })
    }
}

/// Interface générique vers la logique topi.
impl AppLogic for LuaAppLogic {
    fn update(&mut self, dt: f64, input: &InputManager) {
        // Exécution de la fonction lua.
        let result = self.lua.scope(|scope| {
            let input_wrapper = scope.create_userdata(LuaInputManager::new(input))?;
            self.update_func.call::<()>((self.table.clone(), dt, input_wrapper))
        });

        // Traitement de l'erreur.
        if let Err(e) = result {
            eprintln!("Lua update wrapper got an error : {e}");
        }
    }

    fn display(&self, renderer: &mut RenderContext) {
        // Execution de la fonction lua.
        let result = self.lua.scope(|scope| {
            let render_wrapper = scope.create_userdata(LuaRenderContext::new(renderer))?;
            self.display_func.call::<()>((self.table.clone(), render_wrapper))
        });

        // Traitement de l'erreur.
        if let Err(e) = result {
            eprintln!("Lua render wrapper got an error : {e}");
        }
    }
}
