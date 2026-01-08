---@meta

---A triangle, represented by 3 points.
---@class (exact) Triangle: TriangleMethods
---@field a Vec2
---@field b Vec2
---@field c Vec2

---@class TriangleClass : TriangleMethods
---@overload fun(a: Vec2, b: Vec2, c: Vec2): Triangle
local module = {}

---@class TriangleMethods: Polygonal
local methods = {}

---Create a new triangle.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@return Triangle
---@nodiscard
function module.new(a, b, c) end

---Returns a temporary copy of this value.
---@param self Triangle
---@return Triangle
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Triangle
---@return Triangle
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Triangle
---@return Triangle
---@nodiscard
function methods.box_clone(self) end

---The triangle's points.
---@param self Triangle
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function module.points(self) end

---The edge vectors of the triangle.
---@param self Triangle
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function module.vectors(self) end

---The vector `a -> b` of the triangle.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.ab(self) end

---The vector `b -> c` of the triangle.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.bc(self) end

---The vector `c -> a` of the triangle.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.ca(self) end

---The three edges of the triangle.
---@param self Triangle
---@return Line
---@return Line
---@return Line
---@nodiscard
function methods.edges(self) end

---The triangle's `a -> b` edge.
---@param self Triangle
---@return Line
---@nodiscard
function methods.edge_ab(self) end

---The triangle's `b -> c` edge.
---@param self Triangle
---@return Line
---@nodiscard
function methods.edge_bc(self) end

---The triangle's `c -> a` edge.
---@param self Triangle
---@return Line
---@nodiscard
function methods.edge_ca(self) end

---The triangle's three edge normals.
---@param self Triangle
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.norms(self) end

---The triangle's `a -> b` edge normal.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.norm_ab(self) end

---The triangle's `b -> c` edge normal.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.norm_bc(self) end

---The triangle's `c -> a` edge normal.
---@param self Triangle
---@return Vec2
---@nodiscard
function methods.norm_ca(self) end

---Returns `true` if the two triangles are approximately equal.
---@param self Triangle
---@param other Triangle
---@return boolean
---@nodiscard
function methods.approx(self, other) end

return module