local topi = require("topi")

local App = {}
App.__index = App

function App.new()
  local self = setmetatable({}, App)
  return self
end

function App:update(dt, input)
end

--- Interface d'affichage appelées par topi.
--- @param renderer RenderContext
function App:display(renderer)
  renderer:setDrawColor(topi.types.Color.new(255, 0, 0))
  renderer:setStrokeWeight(0)
  renderer:drawRect(0, 0, 120, 70)

  renderer:setDrawColor(topi.types.Color.new(0, 0, 255))
  renderer:setStrokeColor(topi.types.Color.new(0, 255, 0))
  renderer:setStrokeWeight(10)
  renderer:drawCircle(78, 100, 30.8)

  renderer:setDrawColor(topi.types.Color.new(200, 200, 200))
  renderer:drawLine(80, 6, 200, 70, 20)

  renderer:setDrawColor(topi.types.Color.new(255, 255, 255))
  renderer:setFont("./JetBrainsMonoNerdFont-Bold.ttf", 30)
  renderer:drawText("Hello world!", 10, 200)

  renderer:drawImage("./tileset.png", 300, 5)
  renderer:drawScaledImage("./tileset.png", 300, 50, 2.0)
end

local app = App.new()
topi.App.new(
  "test",
  600, 600,
  app
):run()
