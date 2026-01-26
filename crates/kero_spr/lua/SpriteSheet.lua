---@meta

---A sheet of sprite tiles.
---@class (exact) SpriteSheet: SpriteSheetMethods

---@class SpriteSheetClass: SpriteSheetMethods
local module = {}

---@class SpriteSheetMethods
local methods = {}

---How many columns the sheet has.
---@param self SpriteSheet
---@return integer
---@nodiscard
function methods.cols(self) end

---How many rows the sheet has.
---@param self SpriteSheet
---@return integer
---@nodiscard
function methods.rows(self) end

---Width of a single tile.
---@param self SpriteSheet
---@return number
---@nodiscard
function methods.tile_w(self) end

---Height of a single tile.
---@param self SpriteSheet
---@return number
---@nodiscard
function methods.tile_h(self) end

---Size of a single tile.
---@param self SpriteSheet
---@return Vec2
---@nodiscard
function methods.tile_size(self) end

---Draw a tile from (`col, row`) of the sheet.
---@param self SpriteSheet
---@param col integer
---@param row integer
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function methods.draw_tile(self, col, row, pos, color, mode, flip_x, flip_y) end

return module