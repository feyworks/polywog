local Keyboard = require "Keyboard"
local Key      = require "Key"

local Main     = {}

function Main:init()
    
end

function Main:update()
    if Keyboard.pressed(Key.SPACE) then
        print("SPACE!")
    end
end

function Main:render()
    
end

return Main
