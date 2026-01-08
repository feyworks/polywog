---@meta

---A 2x2 matrix.
---@class (exact) Mat2: Mat2Methods
---@field x_axis Vec2
---@field y_axis Vec2
---@operator mul(Mat2): Mat2

---@class Mat2Class : Mat2Methods
---@overload fun(x_axis: Vec2, y_axis: Vec2): Mat2
local module = {}

---@class Mat2Methods
local methods = {}

---An identity matrix.
---@return Mat2
---@nodiscard
function module.identity() end

---A zero matrix.
---@return Mat2
---@nodiscard
function module.zero() end

---Create a new matrix.
---@param x_axis Vec2
---@param y_axis Vec2
---@return Mat2
---@nodiscard
function module.new(x_axis, y_axis) end

---Create a rotation matrix.
---@param angle number Angle (in radians).
---@return Mat2
---@nodiscard
function module.rotation(angle) end

---Create a scaling matrix.
---@param scale Vec2|number
---@return Mat2
---@nodiscard
function module.scale(scale) end

---Create a rotation scaling matrix.
---@param scale Vec2|number
---@param angle number
---@return Mat2
---@nodiscard
function module.scale_rotation(scale, angle) end

---Returns a temporary copy of this value.
---@param self Mat2
---@return Mat2
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Mat2
---@return Mat2
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Mat2
---@return Mat2
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the matrices are approximately equal.
---@param self Mat2
---@param other Mat2
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Calculates the determinant.
---@param self Mat2
---@return number
---@nodiscard
function methods.determinant(self) end

---Inverts the matrix.
---@param self Mat2
---@return Mat2
---@nodiscard
function methods.inverse(self) end

---Multiplies the two matrices.
---@param self Mat2
---@param other Mat2
---@return Mat2
---@nodiscard
function methods.mul_mat2(self, other) end

---Transform the 2D vector by this matrix.
---@param self Mat2
---@param vec Vec2
---@return Vec2
---@nodiscard
function methods.transform_vec2(self, vec) end

---Transpose the matrix.
---@param self Mat2
---@return Mat2
---@nodiscard
function methods.transpose(self) end

return module