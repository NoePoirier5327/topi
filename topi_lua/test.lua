local topi = require("topi")

local App = {}
App.__index = App

function App.new()
  local self = setmetatable({}, App)
  return self
end

function App:update(dt, input)
end

function App:display(renderer)
end

local app = App.new()
topi.App.new(
  "test",
  600, 600,
  app
):run()
