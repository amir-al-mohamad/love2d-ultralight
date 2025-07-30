--- Ultralight API module for creating and managing web views.
-- @module ultralight

local clipboard = require("ultralight.clipboard")
local filesystem = require("ultralight.filesystem")

local ultralight = {}

--- The current version of the module.
-- @string version Semantic version, e.g. `"1.0.0"`.
ultralight.version = "1.0.0"

--- Creates a new Ultralight view.
-- @function createView
-- @treturn UltralightView A new view instance.
function ultralight.createView() end

--- Updates logic for all views.
-- Should be called from `love.update`.
-- @function update
function ultralight.update() end

--- Renders all views.
-- Should be called from `love.draw`.
-- @function draw
function ultralight.draw() end

--- Cleans up Ultralight resources before quitting.
---
--- IMPORTANT: This function **must** be called from `love.quit()` when the application is closing.
--- If not called, the application may hang.
---
--- NOTE: `love.quit` is not called when an unhandled Lua error occurs.
--- In such cases, you must manually call `ultralight.quit()` to avoid hanging.
--- See the workaround example below.
---
--- @usage
--- function love.quit()
---   ultralight.quit()
--- end
--
--- local oldErrorHandler = love.errhand
--- function love.errorhandler(err)
---   ultralight.quit()
---
---   local errorDrawFunction = oldErrorHandler(err)
---   while true do
---     local ret = errorDrawFunction()
---     if ret then break end
---   end
--- end
---
--- @function quit
function ultralight.quit() end

--- Clipboard submodule for Ultralight.
-- Provides functions to interact with the system clipboard.
-- @section clipboard
ultralight.clipboard = clipboard

--- Filesystem submodule for Ultralight.
-- Provides functions to interact with the filesystem.
-- @section filesystem
ultralight.filesystem = filesystem

return ultralight
