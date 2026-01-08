---@meta

---Represents the projection of a 2D shape onto an axis.
---@class (exact) Projection: ProjectionMethods
---@field min number
---@field max number

---@class ProjectionClass : ProjectionMethods
---@overload fun(min: number, max: number): Projection
local module = {}

---@class ProjectionMethods
local methods = {}

---Create a new Projection.
---@param min number
---@param max number
---@return Projection
---@nodiscard
function module.new(min, max) end

---Returns a temporary copy of this value.
---@param self Projection
---@return Projection
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Projection
---@return Projection
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Projection
---@return Projection
---@nodiscard
function methods.box_clone(self) end

---Returns `true` if the two projections are approximately equal.
---@param self Projection
---@param other Projection
---@return boolean
---@nodiscard
function methods.approx(self, other) end

---Length of the projection `|max - min|`.
---@param self Projection
---@return number
---@nodiscard
function methods.len(self) end

---If this projection overlaps the other, returns the amount which it overlaps.
---@param self Projection
---@param other Projection
---@return number?
---@nodiscard
function methods.overlap(self, other) end

---Returns `true` if the projections overlap.
---@param self Projection
---@param other Projection
---@return boolean
---@nodiscard
function methods.overlaps(self, other) end

return module