---@meta

---@class Polygonal: Shape
local Polygonal = {}

---Get the nearest point on this polygon to the source point.
---@param self Polygonal
---@param source Vec2
---@return Vec2
---@nodiscard
function Polygonal.nearest_vertex(self, source) end

---Iterate all polygonal edges of the shape, returning true if any of them satisfy the conditional function provided.
---@param self Polygonal
---@param cond fun(edge: Line): boolean
---@return boolean
---@nodiscard
function Polygonal.all_edges(self, cond) end

---Iterate all edge normals of the shape, returning true if any of them satisfy the conditional function provided.
---@param self Polygonal
---@param cond fun(norm: Vec2): boolean
---@return boolean
---@nodiscard
function Polygonal.all_normals(self, cond) end

---Walk through every normal of the polygon.
---@param self Polygonal
---@param plot fun(norm: Vec2)
function Polygonal.visit_normals(self, plot) end

---Walk through every edge of the polygon.
---@param self Polygonal
---@param plot fun(edge: Line)
function Polygonal.visit_edges(self, plot) end

return Polygonal