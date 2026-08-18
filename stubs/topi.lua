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
---@return nil
function CommandProcessor:new_anonym_task(cmd_type, func) end

---@param func fun(renderer: Renderer)
---@return nil
function CommandProcessor:new_drawing_task(func) end

---@return nil
function CommandProcessor:clear() end

---@class RGBAColor
---@field r number
---@field g number
---@field b number
---@field a number
local RGBAColor = {}

---@class Renderer
local Renderer = {}

---@param x number
---@param y number
---@param w number
---@param h number
---@param color RGBAColor
---@return nil
function Renderer:draw_colored_rect(x, y, w, h, color) end

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