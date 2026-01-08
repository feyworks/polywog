---@meta

---A line segment.
---@class (exact) Line: LineMethods
---@field a Vec2
---@field b Vec2

---@class LineClass : LineMethods
---@overload fun(a: Vec2, b: Vec2): Line
local module = {}

---@class LineMethods: Shape
local methods = {}

---A zero-length line from `(0, 0)` to `(0, 0)`.
---@return Line
---@nodiscard
function module.zero() end

---Create a new line.
---@param a Vec2
---@param b Vec2
---@return Line
---@nodiscard
function module.new(a, b) end

---Returns a temporary copy of this value.
---@param self Line
---@return Line
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Line
---@return Line
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Line
---@return Line
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two lines are approximately equal.
---@param self Line
---@param other Line
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---The points of the line.
---@param self Line
---@return Vec2
---@return Vec2
---@nodiscard
function methods.points(self) end

---Reverse the line (swap the start and end points).
---@param self Line
---@return Line
---@nodiscard
function methods.rev(self) end

---The vector from the line’s start to end point.
---@param self Line
---@return Vec2
---@nodiscard
function methods.vector(self) end

---Squared length of the line.
---@param self Line
---@return number
---@nodiscard
function methods.sqr_len(self) end

---Rectangular bounds of the line.
---@param self Line
---@return Rect
---@nodiscard
function methods.bounds(self) end

---Center of the line.
---@param self Line
---@return Vec2
---@nodiscard
function methods.center(self) end

---Length of the line.
---@param self Line
---@return number
---@nodiscard
function methods.len(self) end

---Axis of the line from `a` to `b`.
---@param self Line
---@return Vec2
---@nodiscard
function methods.norm(self) end

---Left-perpendicular axis of the line.
---@param self Line
---@return Vec2
---@nodiscard
function methods.left_norm(self) end

---Right-perpendicular axis of the line.
---@param self Line
---@return Vec2
---@nodiscard
function methods.right_norm(self) end

---Project this line onto the provided axis.
---@param self Line
---@param axis Vec2
---@return Projection
---@nodiscard
function methods.project_onto_axis(self, axis) end

---Project the point onto this line.
---@param self Line
---@param p Vec2
---@return Vec2
---@nodiscard
function methods.project_point(self, p) end

---Check if the ray hits this line.
---@param self Line
---@param ray Ray
---@return boolean
---@nodiscard
function methods.rayhit(self, ray) end

---Cast a ray against this line. If it intersects the line, return the distance along the ray that the intersection occurred.
---@param self LineMethods
---@param ray Ray
---@return number?
---@nodiscard
function methods.raycast(self, ray) end

return module