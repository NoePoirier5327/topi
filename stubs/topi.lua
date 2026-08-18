---@meta topi

---@enum CommandType
local CommandType = {
    Setup = "Setup",
    Update = "Update",
}

---@class CommandProcessor
local CommandProcessor = {}

---@param cmd_type CommandType
---@param func fun(dt?: number)
function CommandProcessor:new_anonyme_task(cmd_type, func) end

---@return nil
function CommandProcessor:clear() end

---@class TopiEngine
local TopiEngine = {}

---@return nil
function TopiEngine:run() end

---@return CommandProcessor
function TopiEngine:commands() end

---@class Topi
local topi = {}

--- Initialise une fenêtre de jeu.
---@param title string
---@param width number
---@param height number
---@return TopiEngine
function topi.init(title, width, height) end

return topi