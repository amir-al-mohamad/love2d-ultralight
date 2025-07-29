use api::init_webview_module;
use mlua::prelude::*;

mod api;
mod callbacks;
mod clipboard;
mod filesystem;
mod keyboard;
mod ultralight_renderer;
mod ultralight_view;

#[mlua::lua_module]
fn love_ultralight(lua: &Lua) -> LuaResult<LuaTable> {
    init_webview_module(lua)
}
