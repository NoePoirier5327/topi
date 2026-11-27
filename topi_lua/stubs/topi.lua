--- @meta

--- @class Color manipule une couleur au format RGBA dans le moteur topi.
--- @field r integer quantitée de rouge dans la couleur total, 0 <= r <= 255.
--- @field g integer quantitée de vert dans la couleur total, 0 <= g <= 255.
--- @field b integer quantitée de bleu dans la couleur total, 0 <= b <= 255.
--- @field a integer taux de transparence de la couleur total, 0 <= a <= 255.
local Color = {}
Color.__index = Color

--- Construit une nouvelle couleur avec les spécifications en paramètre.
--- Son format est le RGBA.
---
--- @param r integer quantitée de rouge dans la nouvelle couleur.
--- @param g integer quantitée de vert dans la nouvelle couleur.
--- @param b integer quantitée de bleu dans la nouvelle couleur.
--- @param a? integer taux de la nouvelle couleur, si omis, vaut 255.
---
--- @return Color Instance vers notre nouvelle couleur.
function Color.new(r, g, b, a) end


--- @class types Interface d'accès aux types préfabriqués dans topi.
--- @field Color Color interface vers la gestion des couleurs affichables dans topi.
local types = {}
types.__index = types
types.Color = Color


--- @class App interface vers l'application graphique de topi.
local App = {}
App.__index = App

--- Construit une nouvelle application graphique (fenêtre) appliquant la logique `logic` à son exécution.
--- Logic doit contenir une fonction `update` et une fonction `display` prenant respectivement en paramètre
--- un number et un ImputManager pour update et un RenderContext pour display.
---
--- @param title string Titre de la nouvelle fenêtre de jeu.
--- @param width integer Largeur de la nouvelle fenêtre de jeu, width >= 0.
--- @param height integer hauteur de la nouvelle fenêtre de jeu, height >= 0.
--- @param logic table instance vers la logique rattachée à la fenêtre de jeu.
---
--- @return App Instance de la nouvelle fenêtre de jeu.
function App.new(title, width, height, logic) end

--- Lance l'exécution du jeu, ne s'arrête pas tant que la fenêtre n'est pas fermée.
function App:run() end


--- @class RenderContext Interface sur la manipulation graphique de topi.
local RenderContext = {}
RenderContext.__index = RenderContext

--- Affiche un rectangle sur la fenêtre de jeu.
---
--- @param x integer coordonnée du bord gauche du rectangle à afficher.
--- @param y integer coordonnée du bord haut du rectangle à afficher.
--- @param width integer largeur du nouveau rectangle à afficher.
--- @param height integer hauteur du nouveau rectangle à afficher.
function RenderContext:drawRect(x, y, width, height) end

--- Affiche un cercle sur la fenêtre de jeu.
---
--- @param x integer coordonnée en x du centre du nouveau cercle.
--- @param y integer coordonnée en y du centre du nouveau cercle.
--- @param radius number rayon du nouveau cercle, radius >= 0.
function RenderContext:drawCircle(x, y, radius) end

--- Affiche une ligne sur la fenêtre de jeu.
---
--- @param x1 integer coordonnée en x de la première extrémitée de la nouvelle ligne.
--- @param y1 integer coordonnée en y de la première extrémitée de la nouvelle ligne.
--- @param x2 integer coordonnée en x de la seconde extrémitée de la nouvelle ligne.
--- @param y2 integer coordonnée en y de la seconde extrémitée de la nouvelle ligne.
--- @param thickness integer épaisseur de la nouvelle ligne.
function RenderContext:drawLine(x1, y1, x2, y2, thickness) end

--- Affiche un message texte sur la fenêtre de jeu.
--- Si aucune police n'est chargée, la fonction lève une erreur.
--- Utilisez `setFont` pour charger une police avant d'afficher du texte.
---
--- @param msg string message à afficher à la position en paramètre.
--- @param x integer coordonnée du bord gauche du texte à afficher.
--- @param y integer coordonnée du bord haut du texte à afficher.
function RenderContext:drawText(msg, x, y) end

--- Affiche une image sur la fenêtre de jeu.
--- Si l'image n'est pas accessible via `path`, la fonction lève une erreur.
---
--- @param path string chemin vers l'image à afficher.
--- @param x integer position du bord gauche de l'image à afficher.
--- @param y integer position du bord haut de l'image à afficher.
function RenderContext:drawImage(path, x, y) end

--- Affiche une image sur la fenêtre de jeu avec possibilitée de changer son échelle de rendue.
--- Si l'image n'est pas accessible via `path`, la fonction lève une erreur.
---
--- @param path string chemin vers l'image à afficher.
--- @param x integer position du bord gauche de l'image à afficher.
--- @param y integer position du bord haut de l'image à afficher.
--- @param scale number échelle de rendue à appliquer à l'image en paramètre, scale >= 0.
function RenderContext:drawScaledImage(path, x, y, scale) end

--- Modifie la couleur d'affichage principale.
--- @param color Color nouvelle couleur attribuée au renderer courant.
function RenderContext:setDrawColor(color) end

--- Modifie la couleur des bordures des objets affichés.
--- @param color Color nouvelle couleur attribuée au renderer courant.
function RenderContext:setStrokeColor(color) end

--- Modifie l'épaisseur de la bordure extérieur des objets affichés par topi.
--- @param strokeWeight integer nouvelle épaisseur de bordure, strokeWeight >= 0.
function RenderContext:setStrokeWeight(strokeWeight) end

--- Active le remplissage des formes.
function RenderContext:enableFilling() end

--- Désactive le remplissage des formes affichables par topi (seul la bordure des formes est affichée).
function RenderContext:disableFilling() end

--- Charge la police en paramètre dans le contexte de rendu pour pouvoir afficher du texte avec `drawText`.
---
--- @param fontPath string chemin vers la police à charger pour l'affichage.
--- @param fontSize integer taille de la police à l'affichage.
function RenderContext:setFont(fontPath, fontSize) end


--- @class Topi Interface d'accès des modules de topi.
--- @field App App Interface vers la manipulation d'applications graphique sur topi.
--- @field types types Interface vers la manipulation des types préfabriqués pour l'utilisateur dans topi.
local Topi = {}
Topi.__index = Topi
Topi.types = types

return Topi
