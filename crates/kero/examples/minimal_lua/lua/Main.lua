local Keyboard = require "Keyboard"
local Key      = require "Key"
local App      = require "App"
local Draw     = require "Draw"
local Mouse    = require "Mouse"
local Vec2     = require "Vec2"

local Main     = {}

function Main:init()
    print("init")
end

function Main:update()
    -- close the game
    if Keyboard.pressed(Key.ESCAPE) then
        App.quit()
    end

    -- restart the game, reloading all the Lua code
    if Keyboard.pressed(Key.R) then
        App.restart()
    end
end

function Main:render()
    local mouse = Mouse.pos()
    Draw.line(Vec2.zero(), mouse, 0xff3377ff)
end

return Main
