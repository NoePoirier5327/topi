local topi = require("topi")
local app = topi.init("Shapes", 1000, 1000)

app:commands():new_drawing_task(
  function (renderer)
    renderer:draw_filled_colored_triangle(0, 0, 500, 0, 500, 500, {r = 0, g = 255, b = 0, a = 255})
    renderer:draw_filled_colored_circle(750, 250, 250, {r = 255, g = 0, b = 0, a = 255})
    renderer:draw_filled_colored_rect(0, 500, 500, 500, {r = 0, g = 0, b = 255, a = 255})
  end
)

app:run()
