--- Filesystem submodule for Ultralight.
-- Provides callbacks to handle file operations for local resources.
-- This submodule is used to implement support for the `file://` protocol,
-- allowing Ultralight to access local files via registered Lua callbacks.
-- @module ultralight.filesystem

local filesystem = {}

--- Registers a callback to check if a file exists.
-- This function allows you to register a callback that will be called to check if a file exists at a given path.
-- @function onFileExists
-- @tparam function callback A function receiving the file path and returning `true` if the file exists, `false` or `nil` otherwise.
-- @tparam string callback.path The file path to check.
-- @treturn boolean|nil Returns `true` if the file exists, `false` if it does not, or `nil` if no callback is set.
-- @usage
-- ultralight.filesystem.onFileExists(function(path)
--   return love.filesystem.getInfo(path) ~= nil
-- end)
function filesystem.onFileExists(callback) end

--- Registers a callback to determine the MIME type of a file.
-- The callback receives the file path and should return a string representing the MIME type.
-- If the callback returns `nil`, the `"application/unknown"` MIME type will be used.
-- @function onGetFileMimeType
-- @tparam function callback A function receiving the file path and returning a MIME type string or `nil`.
-- @tparam string callback.path The file path to check.
-- @treturn string The MIME type of the file (defaults to `"application/unknown"` if callback returns `nil`).
-- @usage
-- ultralight.filesystem.onGetFileMimeType(function(path)
--   if path:match("%.html$") then
--     return "text/html"
--   end
-- end)
function filesystem.onGetFileMimeType(callback) end

--- Registers a callback to determine the character set of a file.
-- The callback receives the file path and should return a charset string (e.g., `"utf-8"`, `"iso-8859-1"`).
-- If the callback returns `nil`, the default charset `"utf-8"` will be used.
-- @function onGetFileCharset
-- @tparam function callback A function receiving the file path and returning a charset string or `nil`.
-- @tparam string callback.path The file path to check.
-- @treturn string|nil The charset name (defaults to `"utf-8"` if callback returns `nil`).
-- @usage
-- ultralight.filesystem.onGetFileCharset(function(path)
--   if path:match("%.txt$") then
--     return "iso-8859-1"
--   end
-- end)
function filesystem.onGetFileCharset(callback) end

--- Registers a callback to open a file.
-- The callback receives the file path and should return the file content as a string, or `nil` if the file cannot be opened.
-- Returning `nil` or nothing indicates failure to open the file.
-- @function onOpenFile
-- @tparam function callback A function receiving the file path and returning the file content as a string or `nil`.
-- @tparam string callback.path The file path to open.
-- @treturn string|nil The file content as a string, or `nil` if opening failed.
-- @usage
-- ultralight.filesystem.onOpenFile(function(path)
--   return love.filesystem.read(path)
-- end)
function filesystem.onOpenFile(callback) end

return filesystem
