---@meta

---A seeded random number generator.
---@class (exact) Rand: RandMethods
---@field seed integer

---@class RandClass : RandMethods
---@overload fun(seed: integer?): Rand
local module = {}

---@class RandMethods
local methods = {}

---Create a new random number generator.
---@param seed integer?
---@return Rand
---@nodiscard
function module.new(seed) end

---Create a clone of this generator.
---@param self Rand
---@return Rand
---@nodiscard
function methods.clone(self) end

---Returns either `true` or `false`.
---@param self Rand
---@return boolean
---@nodiscard
function methods.bool(self) end

---Has a `chance` (value from `0-1`) to return `true`.
---@param self Rand
---@param chance number
---@return boolean
---@nodiscard
function methods.chance(self, chance) end

---Randomly select and return one of the provided arguments.
---@generic T
---@param self Rand
---@param item1 T
---@param item2 T
---@param ... T
---@return T
---@nodiscard
function methods.choose(self, item1, item2, ...) end

---Randomly select an item from the list and return it.
---@generic T
---@param self Rand
---@param choices T[]
---@return T?
---@nodiscard
function methods.choose_from(self, choices) end

---Randomly select an item from the list using their respective weights.
---@generic T
---@param self Rand
---@param values T[]
---@param weights number[]
---@return T?
---@nodiscard
function methods.choose_weighted(self, values, weights) end

---Clone the generator.
---@param self Rand
---@return Rand
---@nodiscard
function methods.clone(self) end

---Use the generator to produce a new [`Guid`](Guid.lua).
---@param self Rand
---@return Guid
---@nodiscard
function methods.guid(self) end

---Return a random integer in the range `[0, max)`.
---@param self Rand
---@param max integer
---@return integer
---@nodiscard
function methods.int(self, max) end

---Return a random integer in the range `[min, max)`.
---@param self Rand
---@param min integer
---@param max integer
---@return integer
---@nodiscard
function methods.int(self, min, max) end

---Return a random float in the range `[0, max)`.
---@param self Rand
---@param max number
---@return number
---@nodiscard
function methods.float(self, max) end

---Return a random float in the range `[min, max)`.
---@param self Rand
---@param min number
---@param max number
---@return number
---@nodiscard
function methods.float(self, min, max) end

---Randomly shuffle the provided list.
---@generic T
---@param self Rand
---@param list T[]
function methods.shuffle(self, list) end

return module
