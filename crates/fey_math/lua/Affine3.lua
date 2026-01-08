---@meta

---A 3D affine matrix (translation, rotation, scaling and shear).
---@class (exact) Affine3: Affine3Methods
---@field matrix Mat3
---@field translation Vec3
---@operator mul(Affine3): Affine3

---@class Affine3Class : Affine3Methods
---@overload fun(matrix: Mat3, translation: Vec3): Affine3
local module = {}

---@class Affine3Methods
local methods = {}

---An identity matrix.
---@return Affine3
---@nodiscard
function module.identity() end

---A zero matrix.
---@return Affine3
---@nodiscard
function module.zero() end

---Create a new matrix.
---@param matrix Mat3
---@param translation Vec3
---@return Affine3
---@nodiscard
function module.new(matrix, translation) end

---Create a matrix rotating around the x-axis.
---@param angle number Angle (in radians).
---@return Affine3
---@nodiscard
function module.rotation_x(angle) end

---Create a matrix rotating around the y-axis.
---@param angle number Angle (in radians).
---@return Affine3
---@nodiscard
function module.rotation_y(angle) end

---Create a matrix rotating around the z-axis.
---@param angle number Angle (in radians).
---@return Affine3
---@nodiscard
function module.rotation_z(angle) end

---Create a scaling matrix.
---@param scale Vec3|number
---@return Affine3
---@nodiscard
function module.scale(scale) end

---Create a translation matrix.
---@param translation Vec3
---@return Affine3
---@nodiscard
function module.translation(translation) end

---Returns a temporary copy of this value.
---@param self Affine3
---@return Affine3
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Affine3
---@return Affine3
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Affine3
---@return Affine3
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the matrices are approximately equal.
---@param self Affine3
---@param other Affine3
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Inverts the matrix.
---@param self Affine3
---@return Affine3?
---@nodiscard
function methods.inverse(self) end

---Multiplies the two matrices.
---@param self Affine3
---@param other Affine3
---@return Affine3
---@nodiscard
function methods.mul_affine3(self, other) end

---Transform the 2D vector by this matrix.
---@param self Affine3
---@param vec Vec2
---@return Vec2
---@nodiscard
function methods.transform_vec2(self, vec) end

---Transform the 2D point by this matrix.
---@param self Affine3
---@param pos Vec2
---@return Vec2
---@nodiscard
function methods.transform_pos2(self, pos) end

---Transform the 3D vector by this matrix.
---@param self Affine3
---@param vec Vec3
---@return Vec3
---@nodiscard
function methods.transform_vec3(self, vec) end

---Transform the 3D point by this matrix.
---@param self Affine3
---@param pos Vec3
---@return Vec3
---@nodiscard
function methods.transform_pos3(self, pos) end

return module