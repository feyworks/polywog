---@meta

---A raycast hit on the surface of a shape. Contains the distance along the ray the hit occurred,
---and the normal of the edge the ray intersected.
---@class (exact) RayHit: RayHitMethods
---@field normal Vec2
---@field distance number

---@class RayHitClass : RayHitMethods
---@overload fun(normal: Vec2, distance: number): RayHit
local module = {}

---@class RayHitMethods
local methods = {}

---Create a new ray hit.
---@param normal Vec2
---@param distance number
---@return RayHit
---@nodiscard
function module.new(normal, distance) end

---Returns a temporary copy of this value.
---@param self RayHit
---@return RayHit
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self RayHit
---@return RayHit
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self RayHit
---@return RayHit
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two ray hits are approximately equal.
---@param self RayHit
---@param other RayHit
---@return boolean
---@nodiscard
function methods.approx(self, other) end

return module