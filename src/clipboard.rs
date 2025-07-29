use cli_clipboard::{ClipboardContext, ClipboardProvider};
use mlua::prelude::*;
use std::cell::RefCell;
use ul_next::platform;

pub enum ClipboardOnGetAction {
    Override(String),
    Allow,
    Deny,
}

thread_local! {
    static CLIPBOARD_READ_CALLBACK: RefCell<Option<Box<dyn Fn() -> ClipboardOnGetAction + 'static>>> = RefCell::new(None);
    static CLIPBOARD_WRITE_CALLBACK: RefCell<Option<Box<dyn Fn(String) -> bool + 'static>>> = RefCell::new(None);
    static CLIPBOARD_CLEAR_CALLBACK: RefCell<Option<Box<dyn Fn() -> bool + 'static>>> = RefCell::new(None);
}

pub struct Clipboard;

impl platform::Clipboard for Clipboard {
    fn clear(&mut self) {
        let allow = CLIPBOARD_CLEAR_CALLBACK
            .with(|cell| cell.borrow().as_ref().map(|cb| cb()).unwrap_or(true));
        if !allow {
            return;
        }
        if let Ok(mut ctx) = ClipboardContext::new() {
            let _ = ctx.clear();
        }
    }

    fn read_plain_text(&mut self) -> Option<String> {
        let get_clipboard = || ClipboardContext::new().ok()?.get_contents().ok();
        match CLIPBOARD_READ_CALLBACK.with(|cell| cell.borrow().as_ref().map(|cb| cb())) {
            Some(ClipboardOnGetAction::Override(text)) => Some(text),
            Some(ClipboardOnGetAction::Allow) | None => get_clipboard(),
            Some(ClipboardOnGetAction::Deny) => None,
        }
    }

    fn write_plain_text(&mut self, text: &str) {
        let allow = CLIPBOARD_WRITE_CALLBACK.with(|cell| {
            cell.borrow()
                .as_ref()
                .map(|cb| cb(text.to_string()))
                .unwrap_or(true)
        });
        if !allow {
            return;
        }
        if let Ok(mut ctx) = ClipboardContext::new() {
            let _ = ctx.set_contents(text.to_string());
        }
    }
}

pub fn clipboard_on_get_text(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    let callback_key = lua.create_registry_value(callback)?;
    let lua_clone = lua.clone();

    CLIPBOARD_READ_CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(Box::new(move || {
            match lua_clone
                .registry_value::<LuaFunction>(&callback_key)
                .and_then(|func| func.call::<LuaValue>(()))
            {
                Ok(LuaValue::String(lua_string)) => lua_string
                    .to_str()
                    .map(|text| ClipboardOnGetAction::Override(text.to_string()))
                    .unwrap_or(ClipboardOnGetAction::Allow),
                Ok(LuaValue::Boolean(false)) => ClipboardOnGetAction::Deny,
                _ => ClipboardOnGetAction::Allow,
            }
        }));
    });

    lua.expire_registry_values();
    Ok(())
}

pub fn clipboard_on_set_text(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    let callback_key = lua.create_registry_value(callback)?;
    let lua_clone = lua.clone();

    CLIPBOARD_WRITE_CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(Box::new(move |text| {
            lua_clone
                .registry_value::<LuaFunction>(&callback_key)
                .and_then(|func| func.call::<bool>(text))
                .unwrap_or(true)
        }));
    });

    lua.expire_registry_values();
    Ok(())
}

pub fn clipboard_on_clear(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    let callback_key = lua.create_registry_value(callback)?;
    let lua_clone = lua.clone();

    CLIPBOARD_CLEAR_CALLBACK.with(|cell| {
        *cell.borrow_mut() = Some(Box::new(move || {
            lua_clone
                .registry_value::<LuaFunction>(&callback_key)
                .and_then(|func| func.call::<bool>(()))
                .unwrap_or(true)
        }));
    });

    lua.expire_registry_values();
    Ok(())
}
