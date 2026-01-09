---@meta

---@alias CursorIcon 
---     |"default"
---     |"context_menu"
---     |"help"
---     |"pointer"
---     |"progress"
---     |"wait"
---     |"cell"
---     |"crosshair"
---     |"text"
---     |"vertical_text"
---     |"alias"
---     |"copy"
---     |"move"
---     |"no_drop"
---     |"not_allowed"
---     |"grab"
---     |"grabbing"
---     |"resize_e"
---     |"resize_se"
---     |"resize_s"
---     |"resize_sw"
---     |"resize_w"
---     |"resize_nw"
---     |"resize_n"
---     |"resize_ne"
---     |"resize_x"
---     |"resize_y"
---     |"resize_all"
---     |"resize_col"
---     |"resize_row"
---     |"all_scroll"
---     |"zoom_in"
---     |"zoom_out"

---@class Window
local Window = {}

---Text displayed in the window title bar.
---@return string
---@nodiscard
function Window.title() end

---Set the text displayed in the window title bar.
---@param title string
function Window.set_title(title) end

---The window's scale factor.
---@return number
---@nodiscard
function Window.scale_factor() end

---Monitor the window is on.
---@return Monitor?
---@nodiscard
function Window.monitor() end

---Centers the window on the chosen monitor.
---@param monitor Monitor
function Window.center_on(monitor) end

---The current fullscreen mode.
---@return ("borderless"|"exclusive")?
---@nodiscard
function Window.fullscreen_mode() end

---Set to windowed mode.
---@param monitor Monitor?
function Window.set_windowed(monitor) end

---Set to borderless fullscreen mode.
---@param monitor Monitor?
function Window.set_fullscreen_borderless(monitor) end

---Set to exclusive fullscreen mode.
---@param mode VideoMode
function Window.set_fullscreen_exclusive(mode) end

---Returns `true` if the window currently has focus.
---@return boolean
---@nodiscard
function Window.has_focus() end

---Position of the window.
---@return Vec2?
---@nodiscard
function Window.pos() end

---X position of the window.
---@return integer?
---@nodiscard
function Window.x() end

---Y position of the window.
---@return integer?
---@nodiscard
function Window.y() end

---Outer position of the window.
---@return Vec2?
---@nodiscard
function Window.outer_pos() end

---Outer x position of the window.
---@return integer?
---@nodiscard
function Window.outer_x() end

---Outer y position of the window.
---@return integer?
---@nodiscard
function Window.outer_y() end

---Set the window's outer position.
---@param x integer
---@param y integer
function Window.set_outer_pos(x, y) end

---Size of the window.
---@return Vec2
---@nodiscard
function Window.size() end

---Width of the window.
---@return integer
---@nodiscard
function Window.width() end

---Height of the window.
---@return integer
---@nodiscard
function Window.height() end

---Outer size of the window.
---@return Vec2
---@nodiscard
function Window.outer_size() end

---Outer width of the window.
---@return integer
---@nodiscard
function Window.outer_width() end

---Outer height of the window.
---@return integer
---@nodiscard
function Window.outer_height() end

---Request a new dpi-independent size for the window.
---@param w integer
---@param h integer
function Window.request_size(w, h) end

---Returns `true` if the window is resizable.
---@return boolean
---@nodiscard
function Window.resizable() end

---Set whether the window is resizable or not.
---@param resizable boolean
function Window.set_resizable(resizable) end

---Returns `true` if the window is maximized.
---@return boolean
---@nodiscard
function Window.maximized() end

---Set whether the window is maximized or not.
---@param maximized boolean
function Window.set_maximized(maximized) end

---Returns `true` if the window is minimized.
---@return boolean
---@nodiscard
function Window.minimized() end

---Set whether the window is minimized or not.
---@param minimized boolean
function Window.set_minimized(minimized) end

---Set the window's cursor icon.
---@param cursor CursorIcon
function Window.set_cursor(cursor) end

return Window