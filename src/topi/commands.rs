use std::{cell::RefCell, rc::Rc};

use mlua::{
    Function, 
    Lua,
    RegistryKey,
    Result as LuaResult,
    UserData,
    FromLua,
    Value as LuaValue,
    Error as LuaError
};

use super::renderer::Renderer;

pub enum Command {
    AnonymeTask(RegistryKey),
    DrawingTask(RegistryKey)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CommandType {
    Setup,
    Update,
}

impl<'lua> FromLua<'lua> for CommandType {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => match s.to_str()? {
                "setup" | "Setup" => Ok(CommandType::Setup),
                "update" | "Update" => Ok(CommandType::Update),
                unknown => Err(LuaError::RuntimeError(format!("Unknown command type: {}", unknown))),
            },

            LuaValue::Integer(0) => Ok(CommandType::Setup),
            LuaValue::Integer(1) => Ok(CommandType::Update),

            _ => Err(LuaError::RuntimeError("String or integer expected for CommandType".to_string())),
        }
    }
}

/// Structure chargé de la gestion des commandes vers le moteur de jeu.
pub struct CommandProcessor {
    setup_command: Vec<Command>, // File de commande à executer une seule fois. 
    update_command: Vec<Command>, // File de commande à executer périodiquement.
    draw_command: Vec<Command>
}

impl CommandProcessor {
    pub fn new() -> Self {
        Self {
            setup_command: Vec::new(),
            update_command: Vec::new(),
            draw_command: Vec::new(),
        }
    }

    /// Créer une nouvelle tâche anonyme à executer.
    ///
    /// # Arguments
    /// * `cmd_type` : Setup -> n'est executer qu'une fois, Update -> executer à cahque tour de boucle.
    /// * `key` : fonction lua anonyme à executer.
    pub fn new_anonyme_task(&mut self, cmd_type: CommandType, key: RegistryKey) {
        match cmd_type {
            CommandType::Setup => self.setup_command.push(Command::AnonymeTask(key)),
            CommandType::Update => self.update_command.push(Command::AnonymeTask(key)),
        }
    }

    /// Créer une nouvelle tâche d'affichage.
    ///
    /// # Argument
    /// * `key`: fonction d'affichage en lua.
    pub fn new_drawing_task(&mut self, key: RegistryKey) {
        self.draw_command.push(Command::DrawingTask(key));
    }

    /// Execute une seule fois les commandes dans la file setup_command (au démarrage).
    /// Libère les fonctions 
    pub fn process_setup(&mut self, lua: &Lua) -> LuaResult<()> {
        for cmd in self.setup_command.drain(..) {
            match cmd {
                Command::AnonymeTask(key) => {
                    let func: Function = lua.registry_value(&key)?;
                    func.call::<(), ()>(())?;
                    lua.remove_registry_value(key)?;
                }

                _ => {
                    return Err(LuaError::RuntimeError("Unexpected command for setup processing.".to_string()));
                }
            }
        }
        Ok(())
    }

    /// Execute périodiquement les commandes dans la file update_command.
    /// Ne libère pas les fonctions anonymes.
    pub fn process_update(&self, lua: &Lua, dt: f32) -> LuaResult<()> {
        for cmd in &self.update_command {
            match cmd {
                Command::AnonymeTask(key) => {
                    let func: Function = lua.registry_value(key)?;
                    func.call::<f32, ()>(dt)?;
                }

                _ => {
                    return Err(LuaError::RuntimeError("Unexpected command for update processing.".to_string()));
                }
            }
        }
        Ok(())
    }

    /// Est chargée de gérer les commandes d'affichage du moteur.
    ///
    /// # Arguments
    /// * `lua`: interface l'interpréteur lua.
    /// * `renderer`: instance de l'afficheur du moteur car la commande Draw l'a en argument.
    pub fn process_draw(&self, lua: &Lua, renderer: &Rc<RefCell<Renderer>>) -> LuaResult<()> {
        for cmd in &self.draw_command {
            match cmd {
                Command::DrawingTask(key) => {
                    let func: Function = lua.registry_value(key)?;
                    func.call::<_, ()>(renderer.clone())?;
                }

                _ => {
                    return Err(LuaError::RuntimeError("Unexpected command for drawing process.".to_string()));
                }
            }
        }

        Ok(())
    }

    /// Libère explicitement les fonctions anonymes.
    pub fn clear(&mut self, lua: &Lua) -> LuaResult<()> {
        for cmd in self.update_command.drain(..) {
            match cmd {
                Command::AnonymeTask(key) => {
                    lua.remove_registry_value(key)?;
                },

                _ => {
                    return Err(LuaError::RuntimeError("Unexpected command for update.".to_string()));
                }
            }
        }

        for cmd in self.draw_command.drain(..) {
            match cmd {
                 Command::DrawingTask(key) => {
                    lua.remove_registry_value(key)?;
                },

                _ => {
                    return Err(LuaError::RuntimeError("Unexpected command for drawing".to_string()));
                }
            }
        }

        Ok(())
    }
}

impl UserData for CommandProcessor {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("new_anonym_task", |lua, this, (cmd_type, func): (CommandType, Function)| {
            let key = lua.create_registry_value(func)?;
            this.new_anonyme_task(cmd_type, key);
            Ok(())
        });

        methods.add_method_mut("new_drawing_task", |lua, this, func: Function| {
            let key = lua.create_registry_value(func)?;
            this.new_drawing_task(key);
            Ok(())
        });

        methods.add_method_mut("clear", |lua, this, ()| {
            this.clear(lua)?;
            Ok(())
        });
    }
}
