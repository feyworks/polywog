---@meta

---@alias TextureFormat
---     |"r8"
---     |"r16"
---     |"r32f"
---     |"rg8"
---     |"rg16"
---     |"rg32f"
---     |"rgba8"
---     |"rgba16"
---     |"rgba32f"

---@class (exact) Texture: TextureMethods

---@class TextureClass: TextureMethods
local module = {}

---@class TextureMethods
local methods = {}

---The default texture (a 1x1 white pixel).
---@return Texture
---@nodiscard
function module.default() end

---Create a new texture from the provided image. If the image is an RGB format,
---then an RGBA-equivalent texture will be created.
---@param img Image
---@return Texture
---@nodiscard
function module.from_img(img) end

-- ---Uploads pixels from the image to the texture. The image format must be the texture
-- ---format's equivalent (meaning RGB images will always throw an error if used here).
-- ---@param self Texture
-- ---@param img Image
-- function methods.set_pixels(self, img) end

-- ---The texture ID.
-- ---@param self Texture
-- ---@return Guid
-- ---@nodiscard
-- function methods.id(self) end

---Size of the texture.
---@param self Texture
---@return Vec2
---@nodiscard
function methods.size(self) end

---Width of the texture.
---@param self Texture
---@return integer
---@nodiscard
function methods.width(self) end

---Height of the texture.
---@param self Texture
---@return integer
---@nodiscard
function methods.height(self) end

---The texture format.
---@param self Texture
---@return TextureFormat
---@nodiscard
function methods.format(self) end

return module
