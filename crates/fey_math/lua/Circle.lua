---@meta

---A circle, represented by a center point and a radius.
---@class (exact) Circle: CircleMethods
---@field center Vec2
---@field radius number

---@class CircleClass : CircleMethods
---@overload fun(center: Vec2, radius: number): Circle
local module = {}

---@class CircleMethods: Shape
local methods = {}

---Create a new circle.
---@param center Vec2
---@param radius number
---@return Circle
---@nodiscard
function module.new(center, radius) end

---Create a new circle.
---@param radius number
---@return Circle
---@nodiscard
function module.new(radius) end

---Returns a temporary copy of this value.
---@param self Circle
---@return Circle
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Circle
---@return Circle
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Circle
---@return Circle
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two circles are approximately equal.
---@param self Circle
---@param other Circle
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Area of the circle.
---@param self Circle
---@return number
---@nodiscard
function methods.area(self) end

---Circumference of the circle.
---@param self Circle
---@return number
---@nodiscard
function methods.circumference(self) end

---Returns `true` if this circle contains the other.
---@param self Circle
---@param circ Circle
---@return boolean
---@nodiscard
function methods.contains_circ(self, circ) end

---Diameter of the circle.
---@param self Circle
---@return number
---@nodiscard
function methods.diameter(self) end

return module