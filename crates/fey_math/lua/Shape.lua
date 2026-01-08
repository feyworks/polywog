---@meta

---@alias DynShape Circle|Triangle|Rect|Quad|Polygon

---@class Shape
local Shape = {}

---Centroid of the shape.
---@param self Shape
---@return Vec2
---@nodiscard
function Shape.centroid(self) end

---If the point is contained within the shape.
---@param self Shape
---@param p Vec2
---@return boolean
---@nodiscard
function Shape.contains(self, p) end

---Rectangular bounds of the shape.
---@return Rect
---@nodiscard
function Shape.bounds(self) end

---Project the shape onto the axis.
---@param self Shape
---@param axis Vec2
---@return Projection
---@nodiscard
function Shape.project_onto_axis(self, axis) end

---Project a point onto the outside surface of the shape.
---@param self Shape
---@param p Vec2
---@return Vec2
---@nodiscard
function Shape.project_point(self, p) end

---Check if a ray intersects this shape.
---@param self Shape
---@param ray Ray
---@return boolean
---@nodiscard
function Shape.rayhit(self, ray) end

---Raycast against the shape.
---@param self Shape
---@param ray Ray
---@return RayHit
---@nodiscard
function Shape.raycast(self, ray) end

---If this shape overlaps the circle.
---@param self Shape
---@param circ Circle
---@return boolean
---@nodiscard
function Shape.overlaps_circ(self, circ) end

---If this shape overlaps the rectangle.
---@param self Shape
---@param rect Rect
---@return boolean
---@nodiscard
function Shape.overlaps_rect(self, rect) end

---If the two shapes overlap.
---@param self Shape
---@param other DynShape
---@return boolean
---@nodiscard
function Shape.overlaps_shape(self, other) end

---If the two shapes overlap, return a push-out vector that can be used to extract them from each other.
---@param self Shape
---@param circ Circle
---@return Vec2?
---@nodiscard
function Shape.extract_from_circ(self, circ) end

---If the two shapes overlap, return a push-out vector that can be used to extract them from each other.
---@param self Shape
---@param rect Rect
---@return Vec2?
---@nodiscard
function Shape.extract_from_rect(self, rect) end

---If the two shapes overlap, return a push-out vector that can be used to extract them from each other.
---@param self Shape
---@param other DynShape
---@return Vec2?
---@nodiscard
function Shape.extract_from_shape(self, other) end

---If this shape is convex.
---@param self Shape
---@return boolean
---@nodiscard
function Shape.is_convex(self) end

return Shape