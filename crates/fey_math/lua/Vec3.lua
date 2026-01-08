---@meta

---A 3-dimensional vector
---@class (exact) Vec3: Vec3Methods
---@field x number
---@field y number
---@field z number
---@operator add(Vec3): Vec3
---@operator sub(Vec3): Vec3
---@operator mul(Vec3|number): Vec3
---@operator div(Vec3|number): Vec3
---@operator unm: Vec3

---@class Vec3Class : Vec3Methods
---@overload fun(x: number, y: number, z: number): Vec3
local module = {}

---@class Vec3Methods
local methods = {}

---`(0, 0, 0)`
---@return Vec3
---@nodiscard
function module.zero() end

---`(1, 1, 1)`
---@return Vec3
---@nodiscard
function module.one() end

---`(1, 0, 0)`
---@return Vec3
---@nodiscard
function module.x_axis() end

---`(0, 1, 0)`
---@return Vec3
---@nodiscard
function module.y_axis() end

---`(0, 0, 1)`
---@return Vec3
---@nodiscard
function module.z_axis() end

---`(1, 0, 0)`
---@return Vec3
---@nodiscard
function module.right() end

---`(-1, 0, 0)`
---@return Vec3
---@nodiscard
function module.left() end

---`(0, -1, 0)`
---@return Vec3
---@nodiscard
function module.down() end

---`(0, 1, 0)`
---@return Vec3
---@nodiscard
function module.up() end

---`(0, 0, 1)`
---@return Vec3
---@nodiscard
function module.forward() end

---`(0, 0, -1)`
---@return Vec3
---@nodiscard
function module.backward() end

---Create a new vector.
---@param x number
---@param y number
---@param z number
---@return Vec3
---@nodiscard
function module.new(x, y, z) end

---Create a new vector `(val, val, val)`.
---@param val number
---@return Vec3
---@nodiscard
function module.splat(val) end

---Given the triangle `(a, b, c)`, and the interpolation values `ab` and `bc`, returns a barycentric coordinate.
---@param a Vec3
---@param b Vec3
---@param c Vec3
---@param ab number
---@param bc number
---@return Vec3
---@nodiscard
function module.barycentric(a, b, c, ab, bc) end

---Returns a temporary copy of this value.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.box_clone(self) end

---Returns a copy of this vector with absolute value of all components.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.abs(self) end

---Returns `true` if the two vectors are approximately equal.
---@param self Vec3
---@param other Vec3
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Returns `true` if the vector is approximately equal to `(0, 0, 0)`.
---@param self Vec3
---@return boolean
---@nodiscard
function methods.approx_zero(self) end

---Rounds the vector's components up to the nearest integer.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.ceil(self) end

---Clamp's the vector's components between the components of `min` and `max`.
---@param self Vec3
---@param min Vec3
---@param max Vec3
---@return Vec3
---@nodiscard
function methods.clamp(self, min, max) end

---Returns the cross product of two vectors.
---@param self Vec3
---@param other Vec3
---@return Vec3
---@nodiscard
function methods.cross(self, other) end

---Returns the normalized direction towards the point.
---@param self Vec3
---@param point Vec3
---@return Vec3
---@nodiscard
function methods.dir_to(self, point) end

---Returns the distance between two points.
---@param self Vec3
---@param point Vec3
---@return Vec3
---@nodiscard
function methods.dist(self, point) end

---Returns the dot product of two vectors.
---@param self Vec3
---@param other Vec3
---@return number
---@nodiscard
function methods.dot(self, other) end

---Rounds the vector's components down to the nearest integer.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.floor(self) end

---Returns `true` if the vector is equal to `(0, 0, 0)`.
---@return boolean
---@nodiscard
function methods.is_zero(self) end

---Length of the vector.
---@param self Vec3
---@return number
---@nodiscard
function methods.len(self) end

---Returns a vector with components set to the largest values in the arguments.
---@param self Vec3
---@param ... Vec3
---@return Vec3
---@nodiscard
function methods.max(self, ...) end

---Returns a vector with components set to the smallest values in the arguments.
---@param self Vec3
---@param ... Vec3
---@return Vec3
---@nodiscard
function methods.min(self, ...) end

---Normalizes the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.norm(self) end

---Reflects the vector off the provided normal.
---@param self Vec3
---@param normal Vec3
---@return Vec3
---@nodiscard
function methods.reflect(self, normal) end

---Rounds the vector's components to the nearest integer.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.round(self) end

---Returns a copy of this vector with all components signed (set to `1`, `-1`, or `0`).
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.sign(self) end

---Returns the square between two points.
---@param self Vec3
---@param point Vec3
---@return Vec3
---@nodiscard
function methods.sqr_dist(self, point) end

---Squared length of the vector.
---@param self Vec3
---@return number
---@nodiscard
function methods.sqr_len(self) end

---Returns a copy of this vector with all components rounded towards zero.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.trunc(self) end

---Changes the length of the vector without changing its direction.
---@param self Vec3
---@param new_len number
---@return number
---@nodiscard
function methods.with_len(self, new_len) end

---Replaces the `x` component of the vector.
---@param self Vec3
---@param x number
---@return Vec3
---@nodiscard
function methods.with_x(self, x) end

---Replaces the `y` component of the vector.
---@param self Vec3
---@param y number
---@return Vec3
---@nodiscard
function methods.with_y(self, y) end

---Replaces the `z` component of the vector.
---@param self Vec3
---@param z number
---@return Vec3
---@nodiscard
function methods.with_z(self, z) end

---Add a fourth dimension to this vector.
---@param self Vec3
---@param w number
---@return Vec3
---@nodiscard
function methods.with_w(self, w) end

---Returns the 2D portion of this vector.
---@param self Vec3
---@return Vec2
---@nodiscard
function methods.xy(self) end

---Swizzles the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.xzy(self) end

---Swizzles the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.yxz(self) end

---Swizzles the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.yzx(self) end

---Swizzles the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.zxy(self) end

---Swizzles the vector.
---@param self Vec3
---@return Vec3
---@nodiscard
function methods.zyx(self) end

return module
