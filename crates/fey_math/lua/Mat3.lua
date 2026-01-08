---@meta

---A 3x3 matrix.
---@class (exact) Mat3: Mat3Methods
---@field x_axis Vec3
---@field y_axis Vec3
---@field z_axis Vec3
---@operator mul(Mat3): Mat3

---@class Mat3Class : Mat3Methods
---@overload fun(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3): Mat3
local module = {}

---@class Mat3Methods
local methods = {}

---An identity matrix.
---@return Mat3
---@nodiscard
function module.identity() end

---A zero matrix.
---@return Mat3
---@nodiscard
function module.zero() end

---Create a new matrix.
---@param x_axis Vec3
---@param y_axis Vec3
---@param z_axis Vec3
---@return Mat3
---@nodiscard
function module.new(x_axis, y_axis, z_axis) end

---Create a matrix representing a rotation around an axis.
---@param axis Vec2
---@param angle number
---@return Mat3
---@nodiscard
function module.axis_angle(axis, angle) end

---Create a matrix rotating around the x-axis.
---@param angle number Angle (in radians).
---@return Mat3
---@nodiscard
function module.rotation_x(angle) end

---Create a matrix rotating around the y-axis.
---@param angle number Angle (in radians).
---@return Mat3
---@nodiscard
function module.rotation_y(angle) end

---Create a matrix rotating around the z-axis.
---@param angle number Angle (in radians).
---@return Mat3
---@nodiscard
function module.rotation_z(angle) end

---Create a scaling matrix.
---@param scale Vec3|number
---@return Mat3
---@nodiscard
function module.scale(scale) end

---Create a translation matrix.
---@param translation Vec2
---@return Mat3
---@nodiscard
function module.translation(translation) end

---Returns a temporary copy of this value.
---@param self Mat3
---@return Mat3
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Mat3
---@return Mat3
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Mat3
---@return Mat3
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the matrices are approximately equal.
---@param self Mat3
---@param other Mat3
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Calculates the determinant.
---@param self Mat3
---@return number
---@nodiscard
function methods.determinant(self) end

---Inverts the matrix.
---@param self Mat3
---@return Mat3?
---@nodiscard
function methods.inverse(self) end

---Multiplies the two matrices.
---@param self Mat3
---@param other Mat3
---@return Mat3
---@nodiscard
function methods.mul_mat3(self, other) end

---Transform the 2D vector by this matrix.
---@param self Mat3
---@param vec Vec2
---@return Vec2
---@nodiscard
function methods.transform_vec2(self, vec) end

---Transform the 2D point by this matrix.
---@param self Mat3
---@param pos Vec2
---@return Vec2
---@nodiscard
function methods.transform_pos2(self, pos) end

---Transform the 3D vector by this matrix.
---@param self Mat3
---@param vec Vec3
---@return Vec3
---@nodiscard
function methods.transform_vec3(self, vec) end

---Transpose the matrix.
---@param self Mat3
---@return Mat3
---@nodiscard
function methods.transpose(self) end

return module