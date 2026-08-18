---@class TopiEngine
local TopiEngine = {}

---@return nil
function TopiEngine:run() end

---@param path string
---@param id number
---@return nil
function TopiEngine:load_texture(path, id) end

---@meta topi

---@class Topi
local topi = {}

--- Initialise une fenêtre de jeu.
---@param title string
---@param width number
---@param height number
---@return TopiEngine
function topi.init(title, width, height) end

return topi
