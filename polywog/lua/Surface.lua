---@meta

---@class (exact) Surface: SurfaceMethods

---@class SurfaceClass: SurfaceMethods
local module = {}

---@class SurfaceMethods
local methods = {}

---Create a new surface. If no format is provided, will default to `"rgba8"`.
---@param width integer
---@param height integer
---@param format TextureFormat?
---@return Surface
---@nodiscard
function module.new(width, height, format) end

-- ---The surface ID.
-- ---@param self Surface
-- ---@return Guid
-- ---@nodiscard
-- function methods.id(self) end

---Size of the surface.
---@param self Surface
---@return Vec2
---@nodiscard
function methods.size(self) end

---Width of the surface.
---@param self Surface
---@return integer
---@nodiscard
function methods.width(self) end

---Height of the surface.
---@param self Surface
---@return integer
---@nodiscard
function methods.height(self) end

---The surface format.
---@param self Surface
---@return TextureFormat
---@nodiscard
function methods.format(self) end

---The surface's texture.
---@param self Surface
---@return Texture
---@nodiscard
function methods.texture(self) end

return module