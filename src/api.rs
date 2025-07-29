use crate::clipboard::{clipboard_on_clear, clipboard_on_get_text, clipboard_on_set_text};
use crate::filesystem::{
    filesystem_set_on_file_exists_callback, filesystem_set_on_get_file_charset_callback,
    filesystem_set_on_get_file_mime_type_callback, filesystem_set_on_open_file_callback,
    filesystem_update,
};
use crate::ultralight_renderer::{renderer_draw, renderer_init, renderer_quit, renderer_update};
use crate::ultralight_view::UltralightView;
use mlua::prelude::*;

pub fn lua_create_view(lua: &Lua, _: ()) -> LuaResult<UltralightView> {
    UltralightView::new(lua)
}

pub fn lua_update(lua: &Lua, _: ()) -> LuaResult<()> {
    filesystem_update(lua);
    renderer_update();
    Ok(())
}

pub fn lua_draw(_: &Lua, _: ()) -> LuaResult<()> {
    renderer_draw();

    Ok(())
}

pub fn lua_quit(_: &Lua, _: ()) -> LuaResult<()> {
    renderer_quit();

    Ok(())
}

// Clipboard handling functions
fn lua_clipboard_on_get_text(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    clipboard_on_get_text(lua, callback)
}

fn lua_clipboard_on_set_text(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    clipboard_on_set_text(lua, callback)
}

fn lua_clipboard_on_clear(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    clipboard_on_clear(lua, callback)
}

// Filesystem handling functions
pub fn lua_filesystem_on_file_exists(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    filesystem_set_on_file_exists_callback(lua, callback)
}

pub fn lua_filesystem_on_get_file_mime_type(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    filesystem_set_on_get_file_mime_type_callback(lua, callback)
}

fn lua_filesystem_on_get_file_charset(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    filesystem_set_on_get_file_charset_callback(lua, callback)
}

fn lua_filesystem_on_open_file(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    filesystem_set_on_open_file_callback(lua, callback)
}

pub fn init_webview_module(lua: &Lua) -> LuaResult<LuaTable> {
    renderer_init(lua)?;

    let exports = lua.create_table()?;
    exports.set("createView", lua.create_function(lua_create_view)?)?;
    exports.set("update", lua.create_function(lua_update)?)?;
    exports.set("draw", lua.create_function(lua_draw)?)?;
    exports.set("quit", lua.create_function(lua_quit)?)?;
    exports.set("version", env!("CARGO_PKG_VERSION"))?;

    let clipboard = lua.create_table()?;
    clipboard.set("onGetText", lua.create_function(lua_clipboard_on_get_text)?)?;
    clipboard.set("onSetText", lua.create_function(lua_clipboard_on_set_text)?)?;
    clipboard.set("onClear", lua.create_function(lua_clipboard_on_clear)?)?;
    exports.set("clipboard", clipboard)?;

    let filesystem = lua.create_table()?;
    filesystem.set(
        "onFileExists",
        lua.create_function(lua_filesystem_on_file_exists)?,
    )?;
    filesystem.set(
        "onGetFileMimeType",
        lua.create_function(lua_filesystem_on_get_file_mime_type)?,
    )?;
    filesystem.set(
        "onGetFileCharset",
        lua.create_function(lua_filesystem_on_get_file_charset)?,
    )?;
    filesystem.set(
        "onOpenFile",
        lua.create_function(lua_filesystem_on_open_file)?,
    )?;
    exports.set("filesystem", filesystem)?;

    Ok(exports)
}
