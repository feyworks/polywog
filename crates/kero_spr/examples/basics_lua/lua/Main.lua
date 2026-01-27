local SpritePacker = require "SpritePacker"
local Vec2         = require "Vec2"

local Main     = {}

function Main:init()
    local packer = SpritePacker.new()
    packer:add_ase("player", "../basics/assets/player.aseprite")
    packer:add_sprite("portrait", "../basics/assets/portrait.png", true, 0)
    packer:add_sheet("tiles", "../basics/assets/tiles.png", true, 16, 16, 0)
    packer:add_patch("textbox", "../basics/assets/textbox.png", true, 8, 8, 16, 16)
    packer:add_font("virtue", "../basics/assets/virtue.ttf", 16)

    self.atlas = packer:pack(4096)
end

function Main:update()
    
end

function Main:render()
    self.atlas.sprites.portrait:draw(Vec2.zero())
end

return Main
