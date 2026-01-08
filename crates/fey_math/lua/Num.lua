---@meta

---Numeric operations.
---@class NumClass
---@field PI number `π`
---@field PI_OVER_2 number `π/2`
---@field PI_OVER_4 number `π/4`
---@field TAU number `2π`
---@field SQRT_2 number `√2`
---@field MAX_INT integer `9223372036854775807`
---@field MIN_INT integer `-9223372036854775808`
---@field MAX_NUM integer `1.7976931348623157E+308f64`
---@field MIN_NUM integer `-1.7976931348623157E+308f64`
local Num = {}

---Returns the absolute value of `x`.
---@param x number
---@return number
---@nodiscard
function Num.abs(x) end

---Computes the positive difference between `x` and `y`.
---@param x number
---@param y number
---@return number
---@nodiscard
function Num.diff(x, y) end

---Computes the arccosine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.acos(x) end

---Computes the inverse hyperbolic cosine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.acosh(x) end

---Approach the target by the provided amount without overshooting.
---@param from number
---@param to number
---@param amount number
---@return number
---@nodiscard
function Num.approach(from, to, amount) end

---Computes the arcsine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.asin(x) end

---@param x number
---@return number
---@nodiscard
---Computes the inverse hyperbolic sine of `x`. 
function Num.asinh(x) end

---Computes the arctangent of `x`.
---@param x number
---@return number
---@nodiscard
function Num.atan(x) end

---Computes the arctangent of `y` and `x`.
---@param y number
---@param x number
---@return number
---@nodiscard
function Num.atan2(y, x) end

---Computes the inverse hyperbolic arctangent of `x`.
---@param x number
---@return number
---@nodiscard
function Num.atanh(x) end

---Catmull-Rom[¹](https://en.wikipedia.org/wiki/Cubic_Hermite_spline#Catmull%E2%80%93Rom_spline) interpolation.
---@param from number
---@param control1 number
---@param control2 number
---@param to number
---@param t number
---@return number
---@nodiscard
function Num.catmull_rom(from, control1, control2, to, t) end

---Computes the cube root of `x`.
---@param x number
---@return number
---@nodiscard
function Num.cbrt(x) end

---Rounds `x` up to the nearest integer.
---@param x number
---@return integer
---@nodiscard
function Num.ceil(x) end

---Clamps `x` in the range `[min, max]`.
---@param x number
---@param min number
---@param max number
---@return number
---@nodiscard
function Num.clamp(x, min, max) end

---Computes the cosine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.cos(x) end

---Computes the hyperbolic cosine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.cosh(x) end

---Cubic Bézier[¹](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) inerpolation.
---@param from number
---@param control1 number
---@param control2 number
---@param to number
---@param t number
---@return number
---@nodiscard
function Num.cubic_bezier(from, control1, control2, to, t) end

---Computes `Eˣ`.
---@param x number
---@return number
---@nodiscard
function Num.exp(x) end

---Computes `2ˣ`.
---@param x number
---@return number
---@nodiscard
function Num.exp2(x) end

---Rounds `x` down to the nearest integer.
---@param x number
---@return integer
---@nodiscard
function Num.floor(x) end

---Computes the fractional part of `x`.
---@param x number
---@return number
---@nodiscard
function Num.fract(x) end

---Cubic Hermite[¹](https://en.wikipedia.org/wiki/Cubic_Hermite_spline) inerpolation.
---@param from any
---@param from_tangent any
---@param to any
---@param to_tangent any
---@param t any
function Num.hermite(from, from_tangent, to, to_tangent, t) end

---Computes the inverse lerp `(x - from) / (to - from)`.
---@param x number
---@param from number
---@param to number
---@return number
---@nodiscard
function Num.inv_lerp(x, from, to) end

---Returns `true` if `x` is finite.
---@param x number
---@return boolean
---@nodiscard
function Num.is_finite(x) end

---Returns `true` if `x` is infinite.
---@param x number
---@return boolean
---@nodiscard
function Num.is_infinite(x) end

---Returns `true` if `x` is not a number (NaN).
---@param x number
---@return boolean
---@nodiscard
function Num.is_nan(x) end

---Linear interpolation.
---@param from number
---@param to number
---@param t number
---@return number
---@nodiscard
function Num.lerp(from, to, t) end

---Computes the natural logarithm of `x`.
---@param x number
---@return number
---@nodiscard
function Num.ln(x) end

---Computes the logarithm of `x` with respect to `base`.
---@param x number
---@param base number
---@return number
---@nodiscard
function Num.log(x, base) end

---Maps `x` from one range to another.
---@param x number
---@param from_min number
---@param from_max number
---@param to_min number
---@param to_max number
---@return number
---@nodiscard
function Num.remap(x, from_min, from_max, to_min, to_max) end

---Returns the highest number of all arguments.
---@generic T: number
---@param x T
---@param ... T
---@return T
---@nodiscard
function Num.max(x, ...) end

---Returns the lowest number of all arguments.
---@generic T: number
---@param x T
---@param ... T
---@return T
---@nodiscard
function Num.min(x, ...) end

---Computes `xⁿ`.
---@param x number
---@param n number
---@return number
---@nodiscard
function Num.pow(x, n) end

---Quadratic Bézier[¹](https://en.wikipedia.org/wiki/B%C3%A9zier_curve) inerpolation.
---@param from number
---@param control number
---@param to number
---@param t number
---@return number
---@nodiscard
function Num.quad_bezier(from, control, to, t) end

---Rounds `x` to the nearest integer.
---@param x number
---@return integer
---@nodiscard
function Num.round(x) end

---Returns the sign of `x` (either `1`, `-1`, or `0`).
---@param x number
---@return integer
---@nodiscard
function Num.sign(x) end

---Computes the sine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.sin(x) end

---Returns `sin(x), cos(x)`.
---@param x number
---@return number
---@nodiscard
function Num.sin_cos(x) end

---Computes the hyperbolic sine of `x`.
---@param x number
---@return number
---@nodiscard
function Num.sinh(x) end

---Smoothstep[¹](https://en.wikipedia.org/wiki/Smoothstep) interpolation.
---@param from number
---@param to number
---@param t number
---@return number
---@nodiscard
function Num.smoothstep(from, to, t) end

---Computes `√x`.
---@param x number
---@return number
---@nodiscard
function Num.sqrt(x) end

---Computes the tangent of `x`.
---@param x number
---@return number
---@nodiscard
function Num.tan(x) end

---Computes the hyperbolic tangent of `x`.
---@param x number
---@return number
---@nodiscard
function Num.tanh(x) end

---Converts radians to degrees.
---@param radians number
---@return number
---@nodiscard
function Num.to_degrees(radians) end

---Converts degrees to radians.
---@param degrees number
---@return number
---@nodiscard
function Num.to_radians(degrees) end

---Rounds `x` to the nearest integer towards zero.
---@param x number
---@return number
---@nodiscard
function Num.trunc(x) end

return Num
