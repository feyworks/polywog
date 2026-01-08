---@meta

---A convex polygon.
---@class (exact) Polygon: PolygonMethods

---@class PolygonClass : PolygonMethods
---@overload fun(init_capacity: integer?): Polygon
local module = {}

---@class PolygonMethods: Polygonal
local methods = {}

---Create a new polygon.
---@param init_capacity integer?
---@return Polygon
---@nodiscard
function module.new(init_capacity) end

---Create a polygon from an array of points.
---@param points Vec2[]
---@return Polygon
---@nodiscard
function module.from_arr(points) end

---Create a polygon from a [`Quad`](Quad.lua).
---@param quad Quad
---@return Polygon
---@nodiscard
function module.from_quad(quad) end

---Create a polygon from a [`Rect`](Rect.lua).
---@param rect Rect
---@return Polygon
---@nodiscard
function module.from_rect(rect) end

---Create a polygon from a [`Triangle`](Triangle.lua).
---@param tri Triangle
---@return Polygon
---@nodiscard
function module.from_tri(tri) end

---Returns a clone of this shape.
---@param self Polygon
---@return Polygon
---@nodiscard
function methods.clone(self) end

---Returns `true` if the two Polygons are approximately equal.
---@param self Polygon
---@param other Polygon
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Currently reserved capacity for points.
---@param self Polygon
---@return integer
---@nodiscard
function methods.capacity(self) end

---Remove all points from the polygon.
---@param self Polygon
function methods.clear(self) end

---Get the *nth* edge of the polygon. A polygon has the same amount of edges as vertices,
---so if this index exceeds `len()`, then `nil` will be returned.
---@param self Polygon
---@param index integer
---@return Line
---@nodiscard
function methods.edge(self, index) end

---Return a list of all the polygon's edges. If `fill` is set, will fill that table and return it
---instead of creating a new one.
---@param self Polygon
---@param fill Line[]?
---@return Line[]
---@nodiscard
function methods.edges(self, fill) end

---Insert a point at the (zero-based) index.
---@param self PolygonMethods
---@param index integer
---@param p Vec2
function methods.insert(self, index, p) end

---Returns `true` if the polygon has no points.
---@param self Polygon
---@return boolean
---@nodiscard
function methods.is_empty(self) end

---How many points are in the polygon.
---@param self Polygon
---@return integer
---@nodiscard
function methods.len(self) end

---Return a list of all points. If `fill` is set, it will fill that table and return it.
---@param self Polygon
---@param fill Vec2[]
---@return Vec2[]
---@nodiscard
function methods.points(self, fill) end

---Removes the last point from the polygon and returns it.
---@param self Polygon
---@return Vec2?
function methods.pop(self) end

---Adds a point to the polygon.
---@param self Polygon
---@param p Vec2
---@overload fun(self: Polygon, x: number, y: number)
function methods.push(self, p) end

---Removes the point at `index`.
---@param self PolygonMethods
---@param index integer
---@return Vec2?
---@nodiscard
function methods.remove(self, index) end

---Preallocate space for `capacity` points. This doesn't add any points to the polygon.
---@param self Polygon
---@param capacity integer
function methods.reserve(self, capacity) end

---Resize the polygon, using `fill_fn` to produce any points needed.
---@param self Polygon
---@param new_len integer
---@param fill_fn fun(): Vec2
---@overload fun(self: Polygon, new_len: integer, p: Vec2)
function methods.resize_with(self, new_len, fill_fn) end

---Transform this polygon by the provided matrix.
---@param self Polygon
---@param mat Affine2
function methods.transform_in_place(self, mat) end

---Translate the polygon by the amount.
---@param self Polygon
---@param amount Vec2
function methods.translate(self, amount) end

return module