---@meta

---A 2-dimensional vector.
---@class (exact) Vec2: Vec2Methods
---@field x number
---@field y number
---@operator add(Vec2): Vec2
---@operator sub(Vec2): Vec2
---@operator mul(Vec2|number): Vec2
---@operator div(Vec2|number): Vec2
---@operator unm: Vec2

---@class Vec2Class : Vec2Methods
---@overload fun(x: number, y: number): Vec2
local module = {}

---@class Vec2Methods
local methods = {}

---`(0, 0)`
---@return Vec2
---@nodiscard
function module.zero() end

---`(1, 1)`
---@return Vec2
---@nodiscard
function module.one() end

---`(1, 0)`
---@return Vec2
---@nodiscard
function module.x_axis() end

---`(0, 1)`
---@return Vec2
---@nodiscard
function module.y_axis() end

---`(1, 0)`
---@return Vec2
---@nodiscard
function module.right() end

---`(-1, 0)`
---@return Vec2
---@nodiscard
function module.left() end

---`(0, 1)`
---@return Vec2
---@nodiscard
function module.down() end

---`(0, -1)`
---@return Vec2
---@nodiscard
function module.up() end

---`(1, 0)`
---@return Vec2
---@nodiscard
function module.east() end

---`(√2, √2)`
---@return Vec2
---@nodiscard 
function module.south_east() end

---`(0, 1)`
---@return Vec2
---@nodiscard 
function module.south() end

---`(-√2, √2)`
---@return Vec2
---@nodiscard 
function module.south_west() end

---`(-1, 0)`
---@return Vec2
---@nodiscard 
function module.west() end

---`(-√2, -√2)`
---@return Vec2
---@nodiscard 
function module.north_west() end

---`(0, -1)`
---@return Vec2
---@nodiscard 
function module.north() end

---`(√2, -√2)`
---@return Vec2
---@nodiscard 
function module.north_east() end

---Create a new vector.
---@param x number
---@param y number
---@return Vec2
---@nodiscard
function module.new(x, y) end

---Create a new vector `(val, val)`.
---@param val number
---@return Vec2
---@nodiscard
function module.splat(val) end

---Given the triangle `(a, b, c)`, and the interpolation values `ab` and `bc`, returns a barycentric coordinate.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param ab number
---@param bc number
---@return Vec2
---@nodiscard
function module.barycentric(a, b, c, ab, bc) end

---Returns a temporary copy of this value.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.box_clone(self) end

---Set the vector's coordinates.
---@param self Vec2
---@param x number
---@param y number
function methods.set(self, x, y) end

---Returns a copy of this vector with absolute value of all components.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.abs(self) end

---Returns `true` if the two vectors are approximately equal.
---@param self Vec2
---@param other Vec2
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Returns `true` if the vector is approximately equal to `(0, 0)`.
---@param self Vec3
---@return boolean
---@nodiscard
function methods.approx_zero(self) end

---Rounds the vector's components up to the nearest integer.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.ceil(self) end

---Clamp's the vector's components between the components of `min` and `max`.
---@param self Vec2
---@param min Vec2
---@param max Vec2
---@return Vec2
---@nodiscard
function methods.clamp(self, min, max) end

---Returns the cross product of two vectors.
---@param self Vec2
---@param other Vec2
---@return number
---@nodiscard
function methods.cross(self, other) end

---Returns the normalized direction towards the point.
---@param self Vec2
---@param point Vec2
---@return Vec2
---@nodiscard
function methods.dir_to(self, point) end

---Returns the distance between two points.
---@param self Vec2
---@param point Vec2
---@return Vec2
---@nodiscard
function methods.dist(self, point) end

---Returns the dot product of two vectors.
---@param self Vec2
---@param other Vec2
---@return number
---@nodiscard
function methods.dot(self, other) end

---Rounds the vector's components down to the nearest integer.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.floor(self) end

---Returns `true` if the vector is equal to `(0, 0)`.
---@return boolean
---@nodiscard
function methods.is_zero(self) end

---Length of the vector.
---@param self Vec2
---@return number
---@nodiscard
function methods.len(self) end

---Returns a vector with components set to the largest values in the arguments.
---@param self Vec2
---@param ... Vec2
---@return Vec2
---@nodiscard
function methods.max(self, ...) end

---Returns a vector with components set to the smallest values in the arguments.
---@param self Vec2
---@param ... Vec2
---@return Vec2
---@nodiscard
function methods.min(self, ...) end

---Normalizes the vector.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.norm(self) end

---Reflects the vector off the provided normal.
---@param self Vec2
---@param normal Vec2
---@return Vec2
---@nodiscard
function methods.reflect(self, normal) end

---Rounds the vector's components to the nearest integer.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.round(self) end

---Returns a copy of this vector with all components signed (set to `1`, `-1`, or `0`).
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.sign(self) end

---Returns the square between two points.
---@param self Vec2
---@param point Vec2
---@return Vec2
---@nodiscard
function methods.sqr_dist(self, point) end

---Squared length of the vector.
---@param self Vec2
---@return number
---@nodiscard
function methods.sqr_len(self) end

---Returns a copy of this vector with all components rounded towards zero.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.trunc(self) end

---Rotates the vector 90° counter-clockwise.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.turn_left(self) end

---Rotates the vector 90° clockwise.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.turn_right(self) end

---Changes the length of the vector without changing its direction.
---@param self Vec2
---@param new_len number
---@return number
---@nodiscard
function methods.with_len(self, new_len) end

---Replaces the `x` component of the vector.
---@param self Vec2
---@param x number
---@return Vec2
---@nodiscard
function methods.with_x(self, x) end

---Replaces the `y` component of the vector.
---@param self Vec2
---@param y number
---@return Vec2
---@nodiscard
function methods.with_y(self, y) end

---Add a third dimension to this vector.
---@param self Vec2
---@param z number
---@return Vec3
---@nodiscard
function methods.with_z(self, z) end

---Swizzles the vector.
---@param self Vec2
---@return Vec2
---@nodiscard
function methods.yx(self) end 

return module