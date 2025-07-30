--- View class for Ultralight web views.
-- @classmod UltralightView

local View = {}

--- Gets the current size of the view.
-- @function getDimensions
-- @treturn number width The width of the view, in pixels.
-- @treturn number height The height of the view, in pixels.
function View.getDimensions() end

--- Gets the current width of the view.
-- @function getWidth
-- @treturn number The width of the view, in pixels.
function View.getWidth() end

--- Gets the current height of the view.
-- @function getHeight
-- @treturn number The height of the view, in pixels.
function View.getHeight() end

--- Sets the size of the view.
-- This resizes the rendering area of the web view.
-- @function setDimensions
-- @tparam number width The new width of the view, in pixels.
-- @tparam number height The new height of the view, in pixels.
function View.setDimensions(width, height) end

--- Sets the width of the view.
-- @function setWidth
-- @tparam number width The new width of the view, in pixels.
function View.setWidth(width) end

--- Sets the height of the view.
-- @function setHeight
-- @tparam number height The new height of the view, in pixels.
function View.setHeight(height) end

--- Returns the current scale factor (zoom level).
-- @function getScale
-- @treturn number The device scale factor.
function View:getScale() end

--- Sets the device scale (zoom level) for the View.
-- Adjusts the rendering scale factor of the web content.
-- @function setScale
-- @tparam number scale The scale factor to apply (e.g., 1.0 for 100%, 2.0 for 200% zoom).
function View:setScale(scale) end

--- Returns the raw framebuffer pixel data of the current view, along with its width and height.
-- This function retrieves the pixel data as a byte string, which can be used for image processing or rendering.
-- See examples for how to use this data.
--
-- @function getFrameBuffer
-- @treturn string Raw pixel data as a byte string.
-- @treturn number width The width of the framebuffer in pixels.
-- @treturn number height The height of the framebuffer in pixels.
function View:getFrameBuffer() end

--- Gives keyboard focus to the View.
-- This sets visual focus (e.g., highlights selection) and allows the View to receive keyboard input.
-- @function focus
-- @see unfocus
-- @see hasFocus
-- @see hasInputFocus
function View:focus() end

--- Removes focus from the View.
-- This unfocuses any focused input elements and disables keyboard input.
-- @function unfocus
-- @see focus
-- @see hasFocus
-- @see hasInputFocus
function View:unfocus() end

--- Checks whether the View currently has keyboard focus.
-- This determines whether the View will handle keyboard input events.
-- @function hasFocus
-- @treturn boolean `true` if the View is focused, otherwise `false`.
-- @see focus
-- @see unfocus
function View:hasFocus() end

--- Checks whether an input field (e.g., a textbox) inside the View is focused.
-- Useful to determine whether keyboard input should be routed to the View or game.
-- @function hasInputFocus
-- @treturn boolean `true` if an input element is focused, otherwise `false`.
-- @see hasFocus
function View:hasInputFocus() end

--- Loads a URL into the view.
-- This will start loading and rendering the given web page.
-- @function loadURL
-- @tparam string url The URL to load (e.g. "https://example.com").
function View:loadURL(url) end

--- Loads raw HTML content into the view.
-- This replaces the current document with the given HTML string.
-- @function loadHTML
-- @tparam string html The HTML content to render.
function View:loadHTML(html) end

--- Returns the current URL loaded in the View.
-- @function getURL
-- @treturn string The current page URL.
function View:getURL() end

--- Returns the title of the current page.
-- @function getTitle
-- @treturn string The page title.
function View:getTitle() end

--- Checks whether the View is currently loading a page.
-- Useful for showing loading indicators or disabling UI until the load finishes.
-- @function isLoading
-- @treturn boolean `true` if a page is still loading, otherwise `false`.
function View:isLoading() end

--- Refresh the current page in the View.
-- Acts like pressing the "Refresh" button in a browser.
-- @function refresh
function View:refresh() end

--- Cancel the current page load.
-- If the View is in the middle of loading a page, this will cancel it.
-- @function cancelLoad
function View:cancelLoad() end

--- Checks if the View can navigate backward in its browsing history.
-- Useful before calling `goBack()` to avoid doing nothing.
-- @function canGoBack
-- @treturn boolean `true` if there is a previous page in history, otherwise `false`.
-- @see goBack
function View:canGoBack() end

--- Navigates backward in the browsing history.
-- Does nothing if there's no previous page. Use `canGoBack()` to check first.
-- @function goBack
-- @see canGoBack
function View:goBack() end

--- Checks if the View can navigate forward in its browsing history.
-- Useful before calling `goForward()` to avoid doing nothing.
-- @function canGoForward
-- @treturn boolean `true` if there is a next page in history, otherwise `false`.
-- @see goForward
function View:canGoForward() end

--- Navigates forward in the browsing history.
-- Does nothing if there's no next page. Use `canGoForward()` to check first.
-- @function goForward
-- @see canGoForward
function View:goForward() end

--- Handles mouse button press event.
-- @function mousePress
-- @tparam number x Mouse x position, in pixels.
-- @tparam number y Mouse y position, in pixels.
-- @tparam number button The button index that was pressed. 1 is the primary mouse button, 2 is the secondary mouse button and 3 is the middle button.
function View:mousePress(x, y, button) end

--- Handles mouse button release event.
-- @function mouseRelease
-- @tparam number x Mouse x position, in pixels.
-- @tparam number y Mouse y position, in pixels.
-- @tparam number button The button index that was released. 1 is the primary mouse button, 2 is the secondary mouse button and 3 is the middle button.
function View:mouseRelease(x, y, button) end

--- Handles mouse movement event.
-- @function mouseMove
-- @tparam number x The mouse position on the x-axis.
-- @tparam number y The mouse position on the y-axis.
function View:mouseMove(x, y) end

--- Handles mouse wheel scroll event.
-- @function mouseWheelMove
-- @tparam number x Amount of horizontal mouse wheel movement. Positive values indicate movement to the right.
-- @tparam number y Amount of vertical mouse wheel movement. Positive values indicate upward movement.
function View:mouseWheelMove(x, y) end

--- Sends a key press event to the WebView.
-- If the `modifiers` table is not provided, modifier keys (Ctrl, Alt, Shift, Gui) are detected automatically using `love.keyboard.isDown`.
-- @function keyPress
-- @tparam Scancode scancode The scancode representing the pressed key.
-- @tparam boolean isrepeat Whether this keypress event is a repeat.
-- @tparam[opt] table modifiers Optional table of modifier keys:
-- @tparam[opt] boolean modifiers.ctrl `true` if Ctrl is pressed
-- @tparam[opt] boolean modifiers.alt `true` if Alt is pressed
-- @tparam[opt] boolean modifiers.shift `true` if Shift is pressed
-- @tparam[opt] boolean modifiers.gui `true` if Meta (Windows/Cmd) is pressed
-- @usage
-- function love.keypressed(_, scancode, isrepeat)
--   -- Automatic modifier detection
--   view:keyPress(scancode, isrepeat)
--
--   -- Or pass modifiers explicitly
--   local modifiers = {
--     alt = love.keyboard.isDown("lalt") or love.keyboard.isDown("ralt"),
--     ctrl = love.keyboard.isDown("lctrl") or love.keyboard.isDown("rctrl"),
--     gui  = love.keyboard.isDown("lgui") or love.keyboard.isDown("rgui"),
--     shift = love.keyboard.isDown("lshift") or love.keyboard.isDown("rshift"),
--   }
--   view:keyPress(scancode, isrepeat, modifiers)
-- end
function View:keyPress(scancode, modifiers) end

--- Sends a key release event to the WebView.
-- If the `modifiers` table is not provided, modifier keys (Ctrl, Alt, Shift, Gui) are detected automatically using `love.keyboard.isDown`.
-- @function keyRelease
-- @tparam Scancode scancode The scancode representing the released key.
-- @tparam[opt] table modifiers Optional table of modifier keys:
-- @tparam[opt] boolean modifiers.ctrl `true` if Ctrl is pressed
-- @tparam[opt] boolean modifiers.alt `true` if Alt is pressed
-- @tparam[opt] boolean modifiers.shift `true` if Shift is pressed
-- @tparam[opt] boolean modifiers.gui `true` if Meta (Windows/Cmd) is pressed
-- @usage
-- function love.keyreleased(_, scancode)
--   -- Automatic modifier detection
--   view:keyRelease(scancode)
--
--   -- Or pass modifiers explicitly
--   local modifiers = {
--     alt = love.keyboard.isDown("lalt") or love.keyboard.isDown("ralt"),
--     ctrl = love.keyboard.isDown("lctrl") or love.keyboard.isDown("rctrl"),
--     gui  = love.keyboard.isDown("lgui") or love.keyboard.isDown("rgui"),
--     shift = love.keyboard.isDown("lshift") or love.keyboard.isDown("rshift"),
--   }
--   view:keyRelease(scancode, modifiers)
-- end
function View:keyRelease(scancode, modifiers) end

--- Sends a text input event to the WebView.
-- This should be used for character input, including Unicode.
-- If `modifiers` is not provided, modifier keys are detected automatically.
-- @function textInput
-- @tparam string text The text input string.
-- @tparam[opt] table modifiers Optional table of modifier keys:
-- @tparam[opt] boolean modifiers.ctrl `true` if Ctrl is pressed
-- @tparam[opt] boolean modifiers.alt `true` if Alt is pressed
-- @tparam[opt] boolean modifiers.shift `true` if Shift is pressed
-- @tparam[opt] boolean modifiers.gui `true` if Meta (Windows/Cmd) is pressed
-- @usage
-- function love.textinput(text)
--   -- Automatic modifier detection
--   view:textInput(text)
--
--   -- Or pass modifiers explicitly
--   local modifiers = {
--     alt = love.keyboard.isDown("lalt") or love.keyboard.isDown("ralt"),
--     ctrl = love.keyboard.isDown("lctrl") or love.keyboard.isDown("rctrl"),
--     gui  = love.keyboard.isDown("lgui") or love.keyboard.isDown("rgui"),
--     shift = love.keyboard.isDown("lshift") or love.keyboard.isDown("rshift"),
--   }
--   view:textInput(text, modifiers)
-- end
function View:textInput(text, modifiers) end

--- Evaluates a raw JavaScript string in the View context and returns the result.
--
-- Executes the provided JavaScript code string `script` inside the WebView.
-- If the script runs successfully, returns the result as a string.
-- If an error occurs during execution, returns an error message.
--
-- @function evaluateScript
-- @tparam string script Raw JavaScript code to execute.
-- @treturn string|nil result The result of the script execution as a string, or `nil` if an error occurred.
-- @treturn string|nil err An error message if execution failed, or `nil` if successful.
function View:evaluateScript(script) end

--- Adds a Lua callback function accessible from JavaScript under a given namespace.
-- If the namespace object does not exist in JavaScript, it will be created automatically.
--
-- When the JavaScript function is called, the provided Lua callback is invoked with
-- the JS arguments converted to Lua types (note: tables/objects are not supported).
-- The Lua callback can return multiple values, which will be returned back to JavaScript as an array.
--
-- It is recommended to call this method inside the `onDocumentReady` callback
-- to ensure the JavaScript environment is fully initialized before adding functions.
--
-- @function addFunction
-- @tparam string namespace The JS object name under which the function will be added.
-- @tparam string name The name of the function inside the namespace.
-- @tparam function callback The Lua callback to be called from JS.
--   Receives the JS call arguments as Lua values (`...`), and can return values back to JS.
--   Tables/objects are not supported in arguments or return values.
-- @see onDocumentReady
--
-- @usage
-- webview:onDocumentReady(function(isMainFrame)
--   if not isMainFrame then return end
--
--   webview:addFunction("myNamespace", "greet", function(name)
--     print("JS called greet with:", name)
--     if type(name) ~= "string" then
--       return "Invalid argument"
--     end
--     return "Hello, " .. name
--   end)
-- end)
--
-- -- JavaScript:
-- -- let result = myNamespace.greet('World')
-- -- console.log(result) // Outputs: Hello, World
function View:addFunction(namespace, name, callback) end

--- Registers a callback fired when a document is ready.
--
-- This callback is useful for initializing JavaScript functions or performing actions
-- @function onDocumentReady
-- @tparam function callback Function called as `callback(isMainFrame, url)`.
-- @tparam boolean callback.isMainFrame `true` if this is the main frame, `false` if it's an iframe.
-- @tparam string callback.url The URL of the loaded document.
function View:onDocumentReady(callback) end

--- Sets a callback that is triggered when the document starts loading.
-- This is called whenever a new URL starts loading into any frame.
-- You can use `isMainFrame` to check whether the event relates to the main document or an iframe.
--
-- @function onBeginLoadingDocument
-- @tparam function callback A function receiving:
-- @tparam boolean callback.isMainFrame `true` if the loading document is the main frame, `false` if it's a subframe (iframe).
-- @tparam string callback.url The URL that is being loaded.
function View:onBeginLoadingDocument(callback) end

--- Registers a callback called when the document finishes loading in a frame.
-- This event is triggered after the page (or iframe) has fully loaded.
--
-- @function onFinishLoadingDocument
-- @tparam function callback Called with parameters:
-- @tparam boolean callback.isMainFrame True if the loaded document is in the main frame.
-- @tparam string callback.url The URL of the loaded document.
function View:onFinishLoadingDocument(callback) end

--- Registers a callback called when the document fails to load in a frame.
-- This event is triggered if loading a page or iframe fails.
--
-- @function onFailLoadingDocument
-- @tparam function callback Called with parameters:
-- @tparam boolean callback.isMainFrame True if the failure occurred in the main frame.
-- @tparam string callback.url The URL of the document that failed to load.
-- @tparam string callback.description A textual description of the error.
-- @tparam string callback.errorDomain The name of the module that triggered the error.
-- @tparam number callback.errorCode The numeric error code.
function View:onFailLoadingDocument(callback) end

--- Sets a callback that is triggered when the JavaScript `window` object is created for a new page load.
--
-- This is the earliest point at which you can inject custom JavaScript into a page.
-- It is triggered before any scripts on the page are executed and before the DOM is parsed.
-- This is useful for defining global functions, shimming APIs, or instrumenting script behavior.
--
-- If you need access to DOM elements or page content, use `onDocumentReady` instead.
-- This event will not fire if the page does not create a JS context (i.e. no scripts are present).
--
-- @function onObjectReady
-- @tparam function callback The function to call.
-- @tparam boolean callback.isMainFrame `true` if this is the main frame, `false` if it's an iframe.
-- @tparam string callback.url The URL of the page that is about to run its scripts.
-- @see onDocumentReady
function View:onObjectReady(callback) end

--- Sets a callback triggered when the page title changes.
-- This is typically called when the document's title is updated.
--
-- @function onChangeTitle
-- @tparam function callback A function receiving:
-- @tparam string callback.newTitle The new title of the page.
--
-- @see getTitle
function View:onChangeTitle(callback) end

--- Registers a callback invoked when the target URL of a navigation changes.
--
-- @function onChangeTargetUrl
-- @tparam function callback A function called with the new target URL.
-- @tparam string callback.url The URL to which navigation is about to occur.
function View:onChangeTargetUrl(callback) end

--- Registers a callback triggered when the session history changes.
-- This happens on navigation events such as `pushState`, `replaceState`, `goBack`, `goForward`,
-- or other changes to the browsing history.
-- Useful for tracking internal navigation in single-page applications or for synchronizing
-- UI state with navigation.
-- @function onUpdateHistory
-- @tparam function callback A function called with no arguments whenever the history changes.
function View:onUpdateHistory(callback) end

--- Sets a callback triggered when a new child View is requested (e.g., window.open in JS).
-- This occurs when a page attempts to create a new view, such as via `window.open()` or a link with `target="_blank"`.
-- You can use this to decide whether to allow, block, or manually handle the creation of popups or tabs.
--
-- @function onCreateChildView
-- @tparam function callback A function receiving:
-- @tparam string callback.sourceURL The URL of the page that requested the new child view.
-- @tparam string callback.targetURL The URL that the new child view should load.
-- @tparam boolean callback.isPopup `true` if this is a popup window (e.g., from `window.open()`), `false` if it's a normal link.
function View:onCreateChildView(callback) end

--- Registers a callback called when the tooltip text changes.
-- This usually happens when the mouse hovers over an element that has a "title" attribute.
--
-- @function onChangeTooltip
-- @tparam function callback Called with:
-- @tparam string callback.tooltip The new tooltip text.
function View:onChangeTooltip(callback) end

--- Registers a callback for console messages from the web view.
-- The callback receives detailed information about each console message.
--
-- @function onConsoleMessage
-- @tparam function callback Called with the following arguments:
-- @tparam number callback.messageSource The source of the message
-- <pre>
--   0 = XML
--   1 = JS
--   2 = Network
--   3 = ConsoleAPI
--   4 = Storage
--   5 = AppCache
--   6 = Rendering
--   7 = CSS
--   8 = Security
--   9 = ContentBlocker
--   10 = Media
--   11 = MediaSource
--   12 = WebRTC
--   13 = ITPDebug
--   14 = PrivateClickMeasurement
--   15 = PaymentRequest
--   16 = Other </pre>
-- @tparam number callback.messageLevel The level of the message.
-- <pre>
--   0 = Log
--   1 = Warning
--   2 = Error
--   3 = Debug
--   4 = Info </pre>
--
-- @tparam string callback.message The console message text.
-- @tparam number callback.lineNumber The line number where the message originated.
-- @tparam number callback.columnNumber The column number where the message originated.
-- @tparam string callback.sourceId The source ID of the message.
function View:onConsoleMessage(callback) end

return View
