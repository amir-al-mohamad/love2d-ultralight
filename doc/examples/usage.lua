-- This example demonstrates how to create and render a webview, handle user input, and draw the result to the screen.

local ffi = require("ffi")
local ultralight = require("love_ultralight")

local webviewInstance
local img
local imgData

function love.load()
  webviewInstance = ultralight.createView()

  local width, height = love.graphics.getDimensions()
  imgData = love.image.newImageData(width, height)
  img = love.graphics.newImage(imgData)

  webviewInstance:setDimensions(width, height)
  webviewInstance:loadHTML([[
    <!DOCTYPE html>
    <html>
    <body>
        <style>
          body {
            color: #ffffff;
          }
        </style>

        <h1>Hello from Ultralight!</h1>
        <p>This is a simple example of using Ultralight with LÖVE.</p>
      </body>
    </html>
  ]])
end

function love.update()
  ultralight.update()
end

function love.resize(w, h)
  webviewInstance:setDimensions(w, h)
end

function love.mousepressed(x, y, button)
  webviewInstance:mousePress(x, y, button)
end

function love.mousereleased(x, y, button)
  webviewInstance:mouseRelease(x, y, button)
end

function love.mousemoved(x, y)
  webviewInstance:mouseMove(x, y)
end

function love.wheelmoved(x, y)
  webviewInstance:mouseWheelMove(x * 200, y * 200) -- Increase sensitivity for wheel movement
end

function love.keypressed(_, scancode)
  webviewInstance:keyPress(scancode)
end

function love.keyreleased(_, scancode)
  webviewInstance:keyRelease(scancode)
end

function love.textinput(text)
  webviewInstance:textInput(text)
end

function love.draw()
  ultralight.draw()

  local data, w, h = webviewInstance:getFrameBuffer()
  local imgDataW, imgDataH = imgData:getDimensions()

  if imgDataW ~= w or imgDataH ~= h then
    imgData:release()
    imgData = love.image.newImageData(w, h)
    img:release()
    img = love.graphics.newImage(imgData)
  end

  ffi.copy(imgData:getFFIPointer(), data, #data)

  img:replacePixels(imgData)

  love.graphics.draw(img, 0, 0)
end

function love.quit()
  ultralight.quit()
end

-- Workaround: Override Love2D error handler to ensure Ultralight shuts down properly on unhandled errors.
local oldErrorHandler = love.errhand
function love.errorhandler(err)
  ultralight.quit()

  local errorDrawFunction = oldErrorHandler(err)

  while true do
    local ret = errorDrawFunction()
    if ret then break end
  end
end
