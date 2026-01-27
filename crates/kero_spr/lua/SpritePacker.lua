---@meta

---@alias PackedSprites { [string]: Sprite }
---@alias PackedSheets { [string]: SpriteSheet }
---@alias PackedFonts { [string]: SpriteFont }
---@alias PackedPatches { [string]: SpritePatch }
---@alias PackedAnims { [string]: SpriteAnim }
---@alias PackedAtlas {
---     texture: Texture,
---     sprites: PackedSprites,
---     sheets: PackedSheets,
---     fonts: PackedFonts,
---     patches: PackedPatches,
---     anims: PackedAnims,
--- }

---Packs sprites, sheets, fonts, etc. into an atlas.
---@class (exact) SpritePacker: SpritePackerMethods

---@class SpritePackerClass: SpritePackerMethods
local module = {}

---@class SpritePackerMethods
local methods = {}

---Create a new packer.
---@return SpritePacker
---@nodiscard
function module.new() end

---Add a sprite (a single image) to be packed.
---@param self SpritePacker
---@param id string
---@param file string
---@param premultiply boolean
---@param trim_threshold integer? `0-255`
function methods.add_sprite(self, id, file, premultiply, trim_threshold) end

---Adds all images found in the directory as sprites. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
---@param premultiply boolean
---@param trim_threshold integer? `0-255`
function methods.add_sprites_in(self, directory, premultiply, trim_threshold) end

---Add a sheet to be packed. The sheet will be split up and tiles will be
---individually packed in order to fit them in better.
---@param self SpritePacker
---@param id string
---@param file string
---@param premultiply boolean
---@param tile_w integer
---@param tile_h integer
---@param trim_threshold integer? `0-255`
function methods.add_sheet(self, id, file, premultiply, tile_w, tile_h, trim_threshold) end

---Adds all images found in the directory as sheets. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
---@param premultiply boolean
---@param tile_w integer
---@param tile_h integer
---@param trim_threshold integer? `0-255`
function methods.add_sheets_in(self, directory, premultiply, tile_w, tile_h, trim_threshold) end

---Add a font file to be packed. Each glyph will be packed individually.
---@param self SpritePacker
---@param id string
---@param file string
---@param size number
---@param chars string[]?
function methods.add_font(self, id, file, size, chars) end

---Adds all fonts found in the directory to be packed. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
---@param size number
---@param chars string[]?
function methods.add_fonts_in(self, directory, size, chars) end

---Add a 9-patch to be packed.
---@param self SpritePacker
---@param id string
---@param file string
---@param premultiply boolean
---@param inner Rect
function methods.add_patch(self, id, file, premultiply, inner) end

---Add a 9-patch to be packed.
---@param self SpritePacker
---@param id string
---@param file string
---@param premultiply boolean
---@param inner_x number
---@param inner_y number
---@param inner_w number
---@param inner_h number
function methods.add_patch(self, id, file, premultiply, inner_x, inner_y, inner_w, inner_h) end

---Adds all images found in the directory as patches. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
---@param premultiply boolean
---@param inner Rect
function methods.add_patches_in(self, directory, premultiply, inner) end

---Adds all images found in the directory as patches. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
---@param premultiply boolean
---@param inner_x number
---@param inner_y number
---@param inner_w number
---@param inner_h number
function methods.add_patches_in(self, directory, premultiply, inner_x, inner_y, inner_w, inner_h) end

---Add an aseprite animation to be packed. The cels of the animation will be
---packed individually to better fit them into the atlas.
---@param self SpritePacker
---@param id string
---@param ase_file string
function methods.add_ase(self, id, ase_file) end

---Adds all aseprite files found in the directory to be packed. Their IDs will be set
---to their filenames without extensions.
---@param self SpritePacker
---@param directory string
function methods.add_ases_in(self, directory) end

---Pack all the items into a sprite atlas.
---@param self SpritePacker
---@param max_size integer
---@return PackedAtlas?
---@nodiscard
function methods.pack(self, max_size) end

return module