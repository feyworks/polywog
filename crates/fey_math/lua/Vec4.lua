---@meta

---A 4-dimensional vector.
---@class (exact) Vec4: Vec4Methods
---@field x number
---@field y number
---@field z number
---@field w number
---@operator add(Vec4): Vec4
---@operator sub(Vec4): Vec4
---@operator mul(Vec4|number): Vec4
---@operator div(Vec4|number): Vec4
---@operator unm: Vec4

---@class Vec4Class : Vec4Methods
---@overload fun(x: number, y: number, z: number, w: number): Vec4
local module = {}

---@class Vec4Methods
local methods = {}

---`(0, 0, 0, 0)`
---@return Vec4
---@nodiscard
function module.zero() end

---`(1, 1, 1, 1)`
---@return Vec4
---@nodiscard
function module.one() end

---`(1, 0, 0, 0)`
---@return Vec4
---@nodiscard
function module.x_axis() end

---`(0, 1, 0, 0)`
---@return Vec4
---@nodiscard
function module.y_axis() end

---`(0, 0, 1, 0)`
---@return Vec4
---@nodiscard
function module.z_axis() end

---`(0, 0, 0, 1)`
---@return Vec4
---@nodiscard
function module.w_axis() end

---Create a new vector.
---@param x number
---@param y number
---@param z number
---@param w number
---@return Vec4
---@nodiscard
function module.new(x, y, z, w) end

---Create a new vector `(val, val, val, val)`.
---@param val number
---@return Vec4
---@nodiscard
function module.splat(val) end

---Returns a temporary copy of this value.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.box_clone(self) end

---Returns a copy of this vector with absolute value of all components.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.abs(self) end

---Returns `true` if the two vectors are approximately equal.
---@param self Vec4
---@param other Vec4
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Returns `true` if the vector is approximately equal to `(0, 0, 0)`.
---@param self Vec4
---@return boolean
---@nodiscard
function methods.approx_zero(self) end

---Rounds the vector's components up to the nearest integer.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.ceil(self) end

---Clamp's the vector's components between the components of `min` and `max`.
---@param self Vec4
---@param min Vec4
---@param max Vec4
---@return Vec4
---@nodiscard
function methods.clamp(self, min, max) end

---Returns the normalized direction towards the point.
---@param self Vec4
---@param point Vec4
---@return Vec4
---@nodiscard
function methods.dir_to(self, point) end

---Returns the distance between two points.
---@param self Vec4
---@param point Vec4
---@return Vec4
---@nodiscard
function methods.dist(self, point) end

---Returns the dot product of two vectors.
---@param self Vec4
---@param other Vec4
---@return number
---@nodiscard
function methods.dot(self, other) end

---Rounds the vector's components down to the nearest integer.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.floor(self) end

---Returns `true` if the vector is equal to `(0, 0, 0)`.
---@return boolean
---@nodiscard
function methods.is_zero(self) end

---Length of the vector.
---@param self Vec4
---@return number
---@nodiscard
function methods.len(self) end

---Returns a vector with components set to the largest values in the arguments.
---@param self Vec4
---@param ... Vec4
---@return Vec4
---@nodiscard
function methods.max(self, ...) end

---Returns a vector with components set to the smallest values in the arguments.
---@param self Vec4
---@param ... Vec4
---@return Vec4
---@nodiscard
function methods.min(self, ...) end

---Normalizes the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.norm(self) end

---Reflects the vector off the provided normal.
---@param self Vec4
---@param normal Vec4
---@return Vec4
---@nodiscard
function methods.reflect(self, normal) end

---Rounds the vector's components to the nearest integer.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.round(self) end

---Returns a copy of this vector with all components signed (set to `1`, `-1`, or `0`).
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.sign(self) end

---Returns the square between two points.
---@param self Vec4
---@param point Vec4
---@return Vec4
---@nodiscard
function methods.sqr_dist(self, point) end

---Squared length of the vector.
---@param self Vec4
---@return number
---@nodiscard
function methods.sqr_len(self) end

---Changes the length of the vector without changing its direction.
---@param self Vec4
---@param new_len number
---@return number
---@nodiscard
function methods.with_len(self, new_len) end

---Replaces the `x` component of the vector.
---@param self Vec4
---@param x number
---@return Vec4
---@nodiscard
function methods.with_x(self, x) end

---Replaces the `y` component of the vector.
---@param self Vec4
---@param y number
---@return Vec4
---@nodiscard
function methods.with_y(self, y) end

---Replaces the `z` component of the vector.
---@param self Vec4
---@param z number
---@return Vec4
---@nodiscard
function methods.with_z(self, z) end

---Replaces the `w` component of the vector.
---@param self Vec4
---@param w number
---@return Vec4
---@nodiscard
function methods.with_w(self, w) end

---Returns the 2D portion of this vector.
---@param self Vec4
---@return Vec2
---@nodiscard
function methods.xy(self) end

---Returns the 3D portion of this vector.
---@param self Vec4
---@return Vec3
---@nodiscard
function methods.xyz(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.wxzy(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.wzxy(self) end



---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.wzyx(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.ywzx(self) end



---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.yxwz(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.yxzw(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.yzwx(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.yzxw(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.zxyw(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.zywx(self) end

---Swizzles the vector.
---@param self Vec4
---@return Vec4
---@nodiscard
function methods.zyxw(self) end

return module
