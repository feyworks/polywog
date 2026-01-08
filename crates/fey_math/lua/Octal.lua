---@meta

---@class (exact) Octal

---@class OctalClass
---@field EAST Octal
---@field SOUTH_EAST Octal
---@field SOUTH Octal
---@field SOUTH_WEST Octal
---@field WEST Octal
---@field NORTH_WEST Octal
---@field NORTH Octal
---@field NORTH_EAST Octal
local Octal = {}

---The direction's name.
---@param dir Octal
---@return string
---@nodiscard
function Octal.name(dir) end

---The direction in radians.
---@param dir Octal
---@return number
---@nodiscard
function Octal.to_rads(dir) end

---The direction in degrees.
---@param dir Octal
---@return number
---@nodiscard
function Octal.to_degs(dir) end

---The direction in rotations.
---@param dir Octal
---@return number
---@nodiscard
function Octal.to_rots(dir) end

---Return the direction nearest to the vector's direction.
---@param v Vec2
---@return Octal
---@nodiscard
function Octal.from_vec2(v) end

---Convert the cardinal direction into its octal equivalent.
---@param dir Cardinal
---@return Octal
---@nodiscard
function Octal.from_cardinal(dir) end

---The direction's sin/cos pair.
---@param dir Octal
---@return number
---@return number
---@nodiscard
function Octal.sin_cos(dir) end

---The direction's normal vector.
---@param dir Octal
---@param len number?
---@return Vec2
---@nodiscard
function Octal.norm(dir, len) end

---Reverse the direction.
---@param dir Octal
---@return Octal
---@nodiscard
function Octal.rev(dir) end

---The next direction clockwise.
---@param dir Octal
---@return Octal
---@nodiscard
function Octal.cw(dir) end

---The next direction counter-clockwise.
---@param dir Octal
---@return Octal
---@nodiscard
function Octal.ccw(dir) end

---Returns the x/y step to move this direction on a grid.
---@param dir Octal
---@return integer
---@return integer
---@nodiscard
function Octal.grid_step(dir) end

return Octal