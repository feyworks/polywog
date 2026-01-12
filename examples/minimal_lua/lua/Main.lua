local Keyboard = require "Keyboard"
local Key      = require "Key"
local Mouse    = require "Mouse"
local Vec2     = require "Vec2"
local Color    = require "Color"
local Draw     = require "Draw"
local Line     = require "Line"
local Font     = require "Font"

local Main     = {}

function Main:init()
    self.font = Font.from_ttf_file("../text/assets/virtue.ttf", 16, true)
end

function Main:update()
    if Keyboard.pressed(Key.SPACE) then
        print("SPACE!")
    end
end

function Main:render()
    local m = Mouse.pos()
    Draw.text("Hello, world!", m.x, m.y, self.font)
end

return Main
