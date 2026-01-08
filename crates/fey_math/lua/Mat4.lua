---@meta

---A 4x4 matrix.
---@class (exact) Mat4: Mat4Methods
---@field x_axis Vec4
---@field y_axis Vec4
---@field z_axis Vec4
---@field w_axis Vec4
---@operator mul(Mat4): Mat4

---@class Mat4Class : Mat4Methods
---@overload fun(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4): Mat4
local module = {}

---@class Mat4Methods
local methods = {}

---An identity matrix.
---@return Mat4
---@nodiscard
function module.identity() end

---A zero matrix.
---@return Mat4
---@nodiscard
function module.zero() end

---Create a new matrix.
---@param x_axis Vec4
---@param y_axis Vec4
---@param z_axis Vec4
---@param w_axis Vec4
---@return Mat4
---@nodiscard
function module.new(x_axis, y_axis, z_axis, w_axis) end

---Create a matrix representing a rotation around an axis.
---@param axis Vec3
---@param angle number
---@return Mat4
---@nodiscard
function module.axis_angle(axis, angle) end

---Create a matrix rotating around the x-axis.
---@param angle number Angle (in radians).
---@return Mat4
---@nodiscard
function module.rotation_x(angle) end

---Create a matrix rotating around the y-axis.
---@param angle number Angle (in radians).
---@return Mat4
---@nodiscard
function module.rotation_y(angle) end

---Create a matrix rotating around the z-axis.
---@param angle number Angle (in radians).
---@return Mat4
---@nodiscard
function module.rotation_z(angle) end

---Create a scaling matrix.
---@param scale Vec3|number
---@return Mat4
---@nodiscard
function module.scale(scale) end

---Create a translation matrix.
---@param translation Vec3|Vec2
---@return Mat4
---@nodiscard
function module.translation(translation) end

---Creates an orthographic perspective matrix.
---@param left number
---@param right number
---@param bottom number
---@param top number
---@param z_near number
---@param z_far number
---@return Mat4
---@nodiscard
function module.ortho(left, right, bottom, top, z_near, z_far) end

---Creates an orthographic perspective matrix.
---@param w number
---@param h number
---@return Mat4
---@nodiscard
function module.ortho_size(w, h) end

---Returns a temporary copy of this value.
---@param self Mat4
---@return Mat4
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Mat4
---@return Mat4
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Mat4
---@return Mat4
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the matrices are approximately equal.
---@param self Mat4
---@param other Mat4
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Calculates the determinant.
---@param self Mat4
---@return number
---@nodiscard
function methods.determinant(self) end

---Inverts the matrix.
---@param self Mat4
---@return Mat4?
---@nodiscard
function methods.inverse(self) end

---Multiplies the two matrices.
---@param self Mat4
---@param other Mat4
---@return Mat4
---@nodiscard
function methods.mul_mat4(self, other) end

---Transform the 2D vector by this matrix.
---@param self Mat4
---@param vec Vec2
---@return Vec2
---@nodiscard
function methods.transform_vec2(self, vec) end

---Transform the 2D point by this matrix.
---@param self Mat4
---@param pos Vec2
---@return Vec2
---@nodiscard
function methods.transform_pos2(self, pos) end

---Transform the 3D vector by this matrix.
---@param self Mat4
---@param vec Vec3
---@return Vec3
---@nodiscard
function methods.transform_vec3(self, vec) end

---Transform the 3D point by this matrix.
---@param self Mat4
---@param pos Vec3
---@return Vec3
---@nodiscard
function methods.transform_pos3(self, pos) end

---Transform the 4D vector by this matrix.
---@param self Mat4
---@param vec Vec4
---@return Vec3
---@nodiscard
function methods.transform_vec4(self, vec) end

---Transpose the matrix.
---@param self Mat4
---@return Mat4
---@nodiscard
function methods.transpose(self) end

return module