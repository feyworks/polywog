---@meta

---A 2D axis-aligned rectangle. Most of the methods for this struct assume that the rectangle has a positive
---width and height, so rectangles where T is signed may yield incorrect values for negative-sized instances.
---@class (exact) Rect: RectMethods
---@field x number
---@field y number
---@field w number
---@field h number

---@class RectClass : RectMethods
---@overload fun(x: number, y: number, w: number, h: number): Rect
---@overload fun(w: number, h: number): Rect
local module = {}

---@class RectMethods: Shape
local methods = {}

---Create a new rect.
---@param x number
---@param y number
---@param w number
---@param h number
---@return Rect
---@nodiscard
function module.new(x, y, w, h) end

---Create a new rect.
---@param w number
---@param h number
---@return Rect
---@nodiscard
function module.new(w, h) end

---Returns a temporary copy of this value.
---@param self Rect
---@return Rect
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Rect
---@return Rect
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Rect
---@return Rect
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two rects are approximately equal.
---@param self Rect
---@param other Rect
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Returns the width and height of the rectangle.
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.size(self) end

---The left edge of the rectangle (`x`).
---@param self Rect
---@return number
---@nodiscard
function methods.left(self) end

---The right edge of the rectangle (`x + w`).
---@param self Rect
---@return number
---@nodiscard
function methods.right(self) end

---The top edge of the rectangle (`y`).
---@param self Rect
---@return number
---@nodiscard
function methods.top(self) end

---The bottom edge of the rectangle (`y + h`).
---@param self Rect
---@return number
---@nodiscard
function methods.bottom(self) end

---The top-left point of the rectangle (`x, y`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.top_left(self) end

---The top-right point of the rectangle (`x + w, y`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.top_right(self) end

---The bottom-right point of the rectangle (`x + w, y + h`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.bottom_right(self) end

---The bottom-left point of the rectangle (`x, y + h`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.bottom_left(self) end

---All 4 corners of the rectangle.
---@param self Rect
---@return Vec2 top_left
---@return Vec2 top_right
---@return Vec2 bottom_right
---@return Vec2 bottom_left
---@nodiscard
function methods.corners(self) end

---The top-center point of the rectangle (`x + w / 2, y`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.top_center(self) end

---The bottom-center point of the rectangle (`x + w / 2, y + h`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.bottom_center(self) end

---The left-center point of the rectangle (`x, y + h / 2`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.left_center(self) end

---The right-center point of the rectangle (`x + w, y + h / 2`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.right_center(self) end

---The horizontal center of the rectangle (`x + w / 2`).
---@param self Rect
---@return number
---@nodiscard
function methods.center_x(self) end

---The vertical center of the rectangle (`y + h / 2`).
---@param self Rect
---@return number
---@nodiscard
function methods.center_y(self) end

---The center of the rectangle (`x + w / 2, y + h / 2`).
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.center(self) end

---The minimum x-position of the rectangle (`min(x, x + w)`).
---@param self Rect
---@return number
---@nodiscard
function methods.min_x(self) end

---The minimum y-position of the rectangle (`min(y, y + h)`).
---@param self Rect
---@return number
---@nodiscard
function methods.min_y(self) end

---The minimum point of the rectangle (`min(x, x + w), min(y, y + h)`)
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.min_pos(self) end

---The maximum x-position of the rectangle (`max(x, x + w)`).
---@param self Rect
---@return number
---@nodiscard
function methods.max_x(self) end

---The maximum y-position of the rectangle (`max(y, y + h)`).
---@param self Rect
---@return number
---@nodiscard
function methods.max_y(self) end

---The maximum point of the rectangle (`max(x, x + w), max(y, y + h)`)
---@param self Rect
---@return number
---@return number
---@nodiscard
function methods.max_pos(self) end

---The right edge of the rectangle.
---@param self Rect
---@return Line
---@nodiscard
function methods.right_edge(self) end

---The left edge of the rectangle.
---@param self Rect
---@return Line
---@nodiscard
function methods.left_edge(self) end

---The top edge of the rectangle.
---@param self Rect
---@return Line
---@nodiscard
function methods.top_edge(self) end

---The bottom edge of the rectangle.
---@param self Rect
---@return Line
---@nodiscard
function methods.bottom_edge(self) end

---All 4 edges of the rectangle.
---@param self Rect
---@return Line right
---@return Line bottom
---@return Line left
---@return Line top
---@nodiscard
function methods.edges(self) end

---Area of the rectangle.
---@param self Rect
---@return number
---@nodiscard
function methods.area(self) end

---Perimeter of the rectangle.
---@param self Rect
---@return number
---@nodiscard
function methods.perimeter(self) end

---Returns `true` if the rectangle is inside this one.
---@param self Rect
---@param inner Rect
---@return boolean
---@nodiscard
function methods.contains_rect(self, inner) end

---Inflate the rectangle by the amount
---@param self Rect
---@param w number
---@param h number?
---@return Rect
---@nodiscard
function methods.inflate(self, w, h) end

---If the two rectangles overlap, returns the overlapping region.
---@param self Rect
---@param other Rect
---@return Rect?
---@nodiscard
function methods.overlap(self, other) end

---Returns a rectangle that exactly encoses the two rectangles.
---@param self Rect
---@param other Rect
---@return Rect
---@nodiscard
function methods.conflate(self, other) end

---Clamps the point within the ractangle.
---@param self Rect
---@param p Vec2
---@return Vec2
---@nodiscard
---@overload fun(self: Rect, x: number, y: number): number, number
function methods.clamp_inside(self, p) end

---Returns `true` if both `w >= 0` and `h >= 0`.
---@param self Rect
---@return boolean
---@nodiscard
function methods.is_positive(self) end

---Returns a non-negative sized equivalent of this rectangle. For example, `(10, 10, -4, -4)` will
---be converted to `(6, 6, 4, 4)`.
---@param self Rect
---@return Rect
---@nodiscard
function methods.non_neg(self) end

---Returns a rectangle with ratio (`w:h`) scaled up to fit inside this rectangle. If `fractional`
---is `false`, then the inner rectangle will only scale up by rounded numbers. In addition to the
---inner rectangle, this also returns the calculated scaling value.
---@param self Rect
---@param w number
---@param h number
---@param fractional boolean
---@return Rect inner
---@return number scale
---@nodiscard
function methods.fitted(self, w, h, fractional) end

---Maps a point from inside this rectangle to the equivalent point inside `target`.
---@param self Rect
---@param point Vec2
---@param target Rect
---@---@return Vec2
---@nodiscard
function methods.map_pos(self, point, target) end

---Translates the rectangle by the amount.
---@param self Rect
---@param amount Vec2
---@return Rect
---@nodiscard
function methods.translate(self, amount) end

return module