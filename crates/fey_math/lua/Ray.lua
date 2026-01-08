---@meta

---A ray with an origin and direction.
---@class (exact) Ray: RayMethods
---@field origin Vec2
---@field direction Vec2

---@class RayClass : RayMethods
---@overload fun(origin: Vec2, direction: Vec2): Ray
local module = {}

---@class RayMethods
local methods = {}

---Create a new ray.
---@param origin Vec2
---@param direction Vec2
---@return Ray
---@nodiscard
function module.new(origin, direction) end

---Returns a temporary copy of this value.
---@param self Ray
---@return Ray
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Ray
---@return Ray
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Ray
---@return Ray
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two rays are approximately equal.
---@param self Ray
---@param other Ray
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Get a point at the `distance` along the ray.
---@param self Ray
---@param distance number
---@return Vec2
---@nodiscard
function methods.point(self, distance) end

return module