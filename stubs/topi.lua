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
function CommandProcessor:new_anonym_task(cmd_type, func) end

---@param func fun(renderer: Renderer)
function CommandProcessor:new_drawing_task(func) end

function CommandProcessor:clear() end

---@class RGBAColor
---@field r number
---@field g number
---@field b number
---@field a number
local RGBAColor = {}

---@class Renderer
local Renderer = {}

---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param x3 number
---@param y3 number
---@param color RGBAColor
function Renderer:draw_filled_colored_triangle(x1, y1, x2, y2, x3, y3, color) end

---@param cx number
---@param cy number
---@param radius number
---@param color RGBAColor
function Renderer:draw_filled_colored_circle(cx, cy, radius, color) end

---@param x number
---@param y number
---@param w number
---@param h number
---@param color RGBAColor
function Renderer:draw_filled_colored_rect(x, y, w, h, color) end

---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param thickness number
---@param color RGBAColor
function Renderer:draw_line(x1, y1, x2, y2, thickness, color) end

---@class TopiEngine
local TopiEngine = {}

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