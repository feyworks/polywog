---@meta

---A quad, represented by 4 points.
---@class (exact) Quad: QuadMethods
---@field a Vec2
---@field b Vec2
---@field c Vec2
---@field d Vec2

---@class QuadClass : QuadMethods
---@overload fun(a: Vec2, b: Vec2, c: Vec2, d: Vec2): Quad
local module = {}

---@class QuadMethods: Polygonal
local methods = {}

---Create a new quad.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param d Vec2
---@return Quad
---@nodiscard
function module.new(a, b, c, d) end

---Create a quad from a rectangle.
---@param rect Rect
---@return Quad
---@nodiscard
---@overload fun(x: number, y: number, w: number, h: number): Quad
function module.from_rect(rect) end

---Create a quad representing a thick line segment.
---@param a Vec2
---@param b Vec2
---@param a_width number
---@param b_width number?
---@return Quad
---@nodiscard
function module.line(a, b, a_width, b_width) end

---Returns a temporary copy of this value.
---@param self Quad
---@return Quad
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Quad
---@return Quad
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Quad
---@return Quad
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two quads are approximately equal.
---@param self Quad
---@param other Quad
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---The quad's four points.
---@param self Quad
---@return Vec2
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.points(self) end

---The four edge vectors of the quad.
---@param self Quad
---@return Vec2
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.vectors(self) end

---The vector `a -> b` of the quad.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.ab(self) end

---The vector `b -> c` of the quad.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.bc(self) end

---The vector `c -> d` of the quad.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.cd(self) end

---The vector `d -> a` of the quad.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.da(self) end

---The four edges of the quad.
---@param self Quad
---@return Line
---@return Line
---@return Line
---@return Line
---@nodiscard
function methods.edges(self) end

---The quad's `a -> b` edge.
---@param self Quad
---@return Line
---@nodiscard
function methods.edge_ab(self) end

---The quad's `b -> c` edge.
---@param self Quad
---@return Line
---@nodiscard
function methods.edge_bc(self) end

---The quad's `c -> d` edge.
---@param self Quad
---@return Line
---@nodiscard
function methods.edge_cd(self) end

---The quad's `d -> a` edge.
---@param self Quad
---@return Line
---@nodiscard
function methods.edge_da(self) end

---The quad's four edge normals.
---@param self Quad
---@return Vec2
---@return Vec2
---@return Vec2
---@return Vec2
---@nodiscard
function methods.norms(self) end

---The quad's `a -> b` edge normal.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.norm_ab(self) end

---The quad's `b -> c` edge normal.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.norm_bc(self) end

---The quad's `c -> d` edge normal.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.norm_cd(self) end

---The quad's `d -> a` edge normal.
---@param self Quad
---@return Vec2
---@nodiscard
function methods.norm_da(self) end

return module