---@meta

---@class (exact) Cardinal

---@class CardinalClass
---@field EAST Cardinal
---@field SOUTH Cardinal
---@field WEST Cardinal
---@field NORTH Cardinal
local Cardinal = {}

---The direction's name.
---@param dir Cardinal
---@return string
---@nodiscard
function Cardinal.name(dir) end

---The direction in radians.
---@param dir Cardinal
---@return number
---@nodiscard
function Cardinal.to_rads(dir) end

---The direction in degrees.
---@param dir Cardinal
---@return number
---@nodiscard
function Cardinal.to_degs(dir) end

---The direction in rotations.
---@param dir Cardinal
---@return number
---@nodiscard
function Cardinal.to_rots(dir) end

---Return the direction nearest to the vector's direction.
---@param v Vec2
---@return Cardinal
---@nodiscard
function Cardinal.from_vec2(v) end

---The direction's sin/cos pair.
---@param dir Cardinal
---@return number
---@return number
---@nodiscard
function Cardinal.sin_cos(dir) end

---The direction's normal vector.
---@param dir Cardinal
---@param len number?
---@return Vec2
---@nodiscard
function Cardinal.norm(dir, len) end

---Reverse the direction.
---@param dir Cardinal
---@return Cardinal
---@nodiscard
function Cardinal.rev(dir) end

---Given an octal direction, return the cardinal direction that most
---closely represents it. This is effectively snapping the angle to 90°.
---
---The bias provides a direction to be used as a tiebreaker; when an octal
---direction is equidistant between two cardinal direction (eg. `NORTH_EAST`
---is equally close to `NORTH` and `EAST`), the direction nearest to the bias
---will be chosen.
---@param octal Octal
---@param bias Cardinal
---@return Octal
---@nodiscard
function Cardinal.from_octal(octal, bias) end

---The next direction clockwise.
---@param dir Cardinal
---@return Cardinal
---@nodiscard
function Cardinal.cw(dir) end

---The next direction counter-clockwise.
---@param dir Cardinal
---@return Cardinal
---@nodiscard
function Cardinal.ccw(dir) end

---Returns the x/y step to move this direction on a grid.
---@param dir Cardinal
---@return integer
---@return integer
---@nodiscard
function Cardinal.grid_step(dir) end

return Cardinal