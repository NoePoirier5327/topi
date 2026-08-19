local topi = require("topi")
local app = topi.init("Bouncing rectangle", 1000, 1000)

local directions = {-1, 1}
local SPEED = { x = math.random(400, 600) * directions[math.random(1,2)], y = math.random(400, 600) *  directions[math.random(1,2)] }
local RECT_SIZE = { w = math.random(50, 150), h = math.random(50, 150) }
local pos = { x = math.random(RECT_SIZE.w, 1000 - RECT_SIZE.w), y = math.random(RECT_SIZE.h, 1000 - RECT_SIZE.h) }

local function update_rect(dt)
  pos.x = pos.x + SPEED.x * dt
  pos.y = pos.y + SPEED.y * dt

  if pos.x <= 0 or pos.x + RECT_SIZE.w >= 1000 then
    SPEED.x = -SPEED.x
  end

  if pos.y <= 0 or pos.y + RECT_SIZE.h >= 1000 then
    SPEED.y = -SPEED.y
  end
end

local function draw_rect(renderer)
  renderer:draw_filled_colored_rect(pos.x, pos.y, RECT_SIZE.w, RECT_SIZE.h, { r = 255, g = 0, b = 0, a = 255 })
end

app:commands():new_anonym_task("Update", update_rect)
app:commands():new_drawing_task(draw_rect)
app:run()
