local topi = require("topi")
local app = topi.init("Test", 600, 400)

function Setup()
  print("Called on setup")
end

function Hello()
  print("Hello")
end

app:commands():new_anonyme_task("Setup", Setup)
app:commands():new_anonyme_task("Update", Hello)
app:run()
