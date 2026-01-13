---@meta

---@class AppModule
local App = {}

---Quit the app.
function App.quit() end

---If `quit()` was called and the app is scheduled to shutdown.
function App.quit_requested() end

---Restart the app, which will reload all Lua modules and reset from `Main.lua`.
function App.restart() end

---If `restart()` was called and the app is scheduled to restart.
function App.restart_requested() end

return App