---@meta

---@alias GamepadStatus "wired"|"draining"|"charging"|"charged"

---@class (exact) Gamepad: GamepadMethods

---@class GamepadClass: GamepadMethods
local module = {}

---@class GamepadMethods
local methods = {}

---How many gamepads are connected.
---@return integer
---@nodiscard
function module.count() end

---All connected gamepads.
---@param fill Gamepad[]?
---@return Gamepad[]?
---@nodiscard
function module.all(fill) end

---All gamepads that connected this frame.
---@param fill Gamepad[]?
---@return Gamepad[]?
---@nodiscard
function module.newly_connected(fill) end

---The most recently active gamepad.
---@return Gamepad?
---@nodiscard
function module.last_active() end

---The gamepad name.
---@param self Gamepad
---@return string
---@nodiscard
function methods.name(self) end

---If the gamepad was connected this frame.
---@param self Gamepad
---@return boolean
---@nodiscard
function methods.was_connected(self) end

---Charging status of the gamepad.
---@param self Gamepad
---@return GamepadStatus
---@return integer percent A value from `0-100` for `"charging"` and `"draining"`.
---@nodiscard
function methods.charging_status(self) end

---If the button is down.
---@param self Gamepad
---@param btn GamepadButton
---@return boolean
---@nodiscard
function methods.down(self, btn) end

---If the button was pressed this frame.
---@param self Gamepad
---@param btn GamepadButton
---@return boolean
---@nodiscard
function methods.pressed(self, btn) end

---If the button was released this frame.
---@param self Gamepad
---@param btn GamepadButton
---@return boolean
---@nodiscard
function methods.released(self, btn) end

---If the button was repeated this frame.
---@param self Gamepad
---@param btn GamepadButton
---@return boolean
---@nodiscard
function methods.repeated(self, btn) end

---If the button state changed this frame.
---@param self Gamepad
---@param btn GamepadButton
---@return boolean
---@nodiscard
function methods.btn_changed(self, btn) end

---The button value from `0.0` (fully up) to `1.0` (fully down).
---@param self Gamepad
---@param btn GamepadButton
---@return number
---@nodiscard
function methods.value(self, btn) end

---The axis value from `-1.0` to `1.0`.
---@param self Gamepad
---@param axis GamepadAxis
---@return number
---@nodiscard
function methods.axis(self, axis) end

---If the axis changed this frame.
---@param self Gamepad
---@param axis GamepadAxis
---@return boolean
---@nodiscard
function methods.axis_changed(self, axis) end

return module