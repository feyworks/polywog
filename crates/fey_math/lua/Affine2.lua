---@meta

---A 2D affine matrix (translation, rotation, scaling and shear).
---@class (exact) Affine2: Affine2Methods
---@field matrix Mat2
---@field translation Vec2
---@operator mul(Affine2): Affine2

---@class Affine2Class : Affine2Methods
---@overload fun(matrix: Mat2, translation: Vec2): Affine2
local module = {}

---@class Affine2Methods
local methods = {}

---An identity matrix.
---@return Affine2
---@nodiscard
function module.identity() end

---A zero matrix.
---@return Affine2
---@nodiscard
function module.zero() end

---Create a new matrix.
---@param matrix Mat2
---@param translation Vec2
---@return Affine2
---@nodiscard
function module.new(matrix, translation) end

---Create a rotation matrix.
---@param angle number Angle (in radians).
---@return Affine2
---@nodiscard
function module.rotation(angle) end

---Create a scaling matrix.
---@param scale Vec2|number
---@return Affine2
---@nodiscard
function module.scale(scale) end

---Create a scaling matrix.
---@param x number
---@param y number
---@return Affine2
---@nodiscard
function module.scale(x, y) end

---Create a translation matrix.
---@param translation Vec2
---@return Affine2
---@nodiscard
function module.translation(translation) end

---Create a translation matrix.
---@param x number
---@param y number
---@return Affine2
---@nodiscard
function module.translation(x, y) end

---Create a translation-rotation-scaling matrix.
---@param translation Vec2
---@param rotation number
---@param scale Vec2
---@return Affine2
---@nodiscard
function module.trs(translation, rotation, scale) end

---Returns a temporary copy of this value.
---@param self Affine2
---@return Affine2
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Affine2
---@return Affine2
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Affine2
---@return Affine2
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the matrices are approximately equal.
---@param self Affine2
---@param other Affine2
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Inverts the matrix.
---@param self Affine2
---@return Affine2?
---@nodiscard
function methods.inverse(self) end

---Multiplies the two matrices.
---@param self Affine2
---@param other Affine2
---@return Affine2
---@nodiscard
function methods.mul_affine2(self, other) end

---Transform the 2D vector by this matrix.
---@param self Affine2
---@param vec Vec2
---@return Vec2
---@nodiscard
function methods.transform_vec2(self, vec) end

---Transform the 2D point by this matrix.
---@param self Affine2
---@param pos Vec2
---@return Vec2
---@nodiscard
function methods.transform_pos2(self, pos) end

-- transform_circ
-- transform_circ_retain
-- transform_dyn
-- transform_dyn_retain
-- transform_line
-- transform_poly
-- transform_quad
-- transform_rect
-- transform_rect_retain
-- transform_tri

return module