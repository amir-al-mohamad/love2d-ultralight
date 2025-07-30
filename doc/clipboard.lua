--- Clipboard submodule for Ultralight.
-- Provides callback hooks for clipboard operations.
-- Allows intercepting and overriding clipboard read, write, and clear actions,
-- enabling custom behavior or filtering of clipboard text.
-- Useful for implementing custom clipboard logic.
-- @module ultralight.clipboard

local clipboard = {}

--- Registers a callback for reading clipboard text.
-- The callback determines whether to override the default clipboard read behavior.
-- If the callback returns a string, it will be used as clipboard content.
-- If it returns `nil`, the system clipboard will be read.
-- If it returns `false`, the operation will be suppressed entirely.
-- @function onGetText
-- @tparam function callback A function returning text (`string`), `nil`, or `false`.
-- @treturn string|nil|false The text to use, `nil` to use system clipboard, or `false` to cancel.
function ultralight.clipboard.onGetText(callback) end

--- Registers a callback for writing clipboard text.
-- If the callback returns `true` or `nil`, the system clipboard will be updated with the provided text.
-- If it returns `false`, the operation will be suppressed.
-- @function onSetText
-- @tparam function callback A function receiving the text to be written.
-- @tparam string callback.text The text to write to the clipboard.
-- @treturn boolean|nil|false `true`/`nil` to proceed, `false` to cancel.
function clipboard.onSetText(callback) end

--- Registers a callback for clearing the clipboard.
-- If the callback returns `true` or `nil`, the system clipboard will be cleared.
-- If it returns `false`, the operation will be suppressed.
-- @function onClearClipboard
-- @tparam function callback A function called when the clipboard is about to be cleared.
-- @treturn boolean|nil|false `true`/`nil` to proceed, `false` to cancel.
function clipboard.onClear(callback) end

return clipboard
