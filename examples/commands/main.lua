-- On importe le moteur de jeu
local topi = require("topi")

-- On créer une nouvelle fenêtre.
local app = topi.init("Test", 600, 400)

function Setup()
  print("Called on setup")
end

function Hello()
  print("Hello")
end

app:commands():new_anonym_task("Setup", Setup)  -- Execute une seule fois la fonction Setup
app:commands():new_anonym_task("Update", Hello) -- Execute périodiquement la fonction Hello
app:run() -- Appelle le moteur de jeu pour lancer l'application.
