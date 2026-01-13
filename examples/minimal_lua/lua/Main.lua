local Keyboard = require "Keyboard"
local Key      = require "Key"
local App      = require "App"

local Main     = {}

function Main:init()
    print("init")
end

function Main:update()
    if Keyboard.pressed(Key.ESCAPE) then
        App.quit()
    end
    if Keyboard.pressed(Key.R) then
        App.restart()
    end
end

function Main:render()
    
end

return Main
