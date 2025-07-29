use crate::ultralight_renderer::renderer_get_lib;
use mlua::prelude::*;
use ul_next::{
    View,
    event::{KeyEvent, KeyEventCreationInfo, KeyEventModifiers, KeyEventType},
    key_code::VirtualKeyCode,
};

fn key_to_key_code(key: &str) -> VirtualKeyCode {
    match key {
        "a" => VirtualKeyCode::A,
        "b" => VirtualKeyCode::B,
        "c" => VirtualKeyCode::C,
        "d" => VirtualKeyCode::D,
        "e" => VirtualKeyCode::E,
        "f" => VirtualKeyCode::F,
        "g" => VirtualKeyCode::G,
        "h" => VirtualKeyCode::H,
        "i" => VirtualKeyCode::I,
        "j" => VirtualKeyCode::J,
        "k" => VirtualKeyCode::K,
        "l" => VirtualKeyCode::L,
        "m" => VirtualKeyCode::M,
        "n" => VirtualKeyCode::N,
        "o" => VirtualKeyCode::O,
        "p" => VirtualKeyCode::P,
        "q" => VirtualKeyCode::Q,
        "r" => VirtualKeyCode::R,
        "s" => VirtualKeyCode::S,
        "t" => VirtualKeyCode::T,
        "u" => VirtualKeyCode::U,
        "v" => VirtualKeyCode::V,
        "w" => VirtualKeyCode::W,
        "x" => VirtualKeyCode::X,
        "y" => VirtualKeyCode::Y,
        "z" => VirtualKeyCode::Z,

        "1" => VirtualKeyCode::Key1,
        "2" => VirtualKeyCode::Key2,
        "3" => VirtualKeyCode::Key3,
        "4" => VirtualKeyCode::Key4,
        "5" => VirtualKeyCode::Key5,
        "6" => VirtualKeyCode::Key6,
        "7" => VirtualKeyCode::Key7,
        "8" => VirtualKeyCode::Key8,
        "9" => VirtualKeyCode::Key9,
        "0" => VirtualKeyCode::Key0,

        "return" => VirtualKeyCode::Return,
        "escape" => VirtualKeyCode::Escape,
        "backspace" => VirtualKeyCode::Back,
        "tab" => VirtualKeyCode::Tab,
        "space" => VirtualKeyCode::Space,

        "-" => VirtualKeyCode::OemMinus,
        "=" => VirtualKeyCode::OemPlus,
        "[" => VirtualKeyCode::Oem4,
        "]" => VirtualKeyCode::Oem6,
        "\\" => VirtualKeyCode::Oem5,
        "nonus#" => VirtualKeyCode::Back,
        ";" => VirtualKeyCode::Oem1,
        "'" => VirtualKeyCode::Oem7,
        "`" => VirtualKeyCode::Oem3,
        "," => VirtualKeyCode::OemComma,
        "." => VirtualKeyCode::OemPeriod,
        "/" => VirtualKeyCode::Oem2,

        "capslock" => VirtualKeyCode::Capital,

        "f1" => VirtualKeyCode::F1,
        "f2" => VirtualKeyCode::F2,
        "f3" => VirtualKeyCode::F3,
        "f4" => VirtualKeyCode::F4,
        "f5" => VirtualKeyCode::F5,
        "f6" => VirtualKeyCode::F6,
        "f7" => VirtualKeyCode::F7,
        "f8" => VirtualKeyCode::F8,
        "f9" => VirtualKeyCode::F9,
        "f10" => VirtualKeyCode::F10,
        "f11" => VirtualKeyCode::F11,
        "f12" => VirtualKeyCode::F12,

        "printscreen" => VirtualKeyCode::Snapshot,
        "scrolllock" => VirtualKeyCode::Scroll,
        "pause" => VirtualKeyCode::Pause,
        "insert" => VirtualKeyCode::Insert,
        "home" => VirtualKeyCode::Home,
        "pageup" => VirtualKeyCode::Prior,
        "delete" => VirtualKeyCode::Delete,
        "end" => VirtualKeyCode::End,
        "pagedown" => VirtualKeyCode::Next,
        "right" => VirtualKeyCode::Right,
        "left" => VirtualKeyCode::Left,
        "down" => VirtualKeyCode::Down,
        "up" => VirtualKeyCode::Up,

        "numlock" => VirtualKeyCode::Numlock,
        "kp/" => VirtualKeyCode::Divide,
        "kp*" => VirtualKeyCode::Multiply,
        "kp-" => VirtualKeyCode::Subtract,
        "kp+" => VirtualKeyCode::Add,
        "kpenter" => VirtualKeyCode::Return,
        "kp1" => VirtualKeyCode::Numpad1,
        "kp2" => VirtualKeyCode::Numpad2,
        "kp3" => VirtualKeyCode::Numpad3,
        "kp4" => VirtualKeyCode::Numpad4,
        "kp5" => VirtualKeyCode::Numpad5,
        "kp6" => VirtualKeyCode::Numpad6,
        "kp7" => VirtualKeyCode::Numpad7,
        "kp8" => VirtualKeyCode::Numpad8,
        "kp9" => VirtualKeyCode::Numpad9,
        "kp0" => VirtualKeyCode::Numpad0,
        "kp." => VirtualKeyCode::Decimal,

        "application" => VirtualKeyCode::Apps,
        "f13" => VirtualKeyCode::F13,
        "f14" => VirtualKeyCode::F14,
        "f15" => VirtualKeyCode::F15,
        "f16" => VirtualKeyCode::F16,
        "f17" => VirtualKeyCode::F17,
        "f18" => VirtualKeyCode::F18,
        "f19" => VirtualKeyCode::F19,
        "f20" => VirtualKeyCode::F20,
        "f21" => VirtualKeyCode::F21,
        "f22" => VirtualKeyCode::F22,
        "f23" => VirtualKeyCode::F23,
        "f24" => VirtualKeyCode::F24,
        "execute" => VirtualKeyCode::Execute,
        "help" => VirtualKeyCode::Help,
        "volumedown" => VirtualKeyCode::VolumeDown,
        "volumeup" => VirtualKeyCode::VolumeUp,

        "lang1" => VirtualKeyCode::Hangul,
        "lang2" => VirtualKeyCode::Hanja,
        "lang3" => VirtualKeyCode::Kana,
        "lang4" => VirtualKeyCode::ImeOn,
        "lang5" => VirtualKeyCode::Final,

        "exsel" => VirtualKeyCode::Exsel,
        "crsel" => VirtualKeyCode::Crsel,

        "lctrl" => VirtualKeyCode::Lcontrol,
        "lshift" => VirtualKeyCode::Lshift,
        "lalt" => VirtualKeyCode::Menu,
        "lgui" => VirtualKeyCode::Lwin,
        "rctrl" => VirtualKeyCode::Rcontrol,
        "rshift" => VirtualKeyCode::Rshift,
        "ralt" => VirtualKeyCode::Menu,
        "rgui" => VirtualKeyCode::Rwin,

        "mode" => VirtualKeyCode::Modechange,

        "audionext" => VirtualKeyCode::MediaNextTrack,
        "audioprev" => VirtualKeyCode::MediaPrevTrack,
        "audiostop" => VirtualKeyCode::MediaStop,
        "audioplay" => VirtualKeyCode::MediaPlayPause,
        "audiomute" => VirtualKeyCode::VolumeMute,
        "mediaselect" => VirtualKeyCode::MediaLaunchMediaSelect,
        "acsearch" => VirtualKeyCode::BrowserSearch,
        "achome" => VirtualKeyCode::BrowserHome,
        "acback" => VirtualKeyCode::BrowserBack,
        "acforward" => VirtualKeyCode::BrowserForward,
        "acstop" => VirtualKeyCode::BrowserStop,
        "acrefresh" => VirtualKeyCode::BrowserRefresh,
        "acbookmarks" => VirtualKeyCode::BrowserFavorites,

        "sleep" => VirtualKeyCode::Sleep,

        _ => VirtualKeyCode::Unknown,
    }
}

fn get_modifier_from_love(lua: &Lua, names: &[&str]) -> Result<bool, LuaError> {
    let love: LuaTable = lua.globals().get("love")?;
    let keyboard: LuaTable = love.get("keyboard")?;
    let is_down: LuaFunction = keyboard.get("isDown")?;

    for &key in names {
        let pressed: bool = is_down.call((key,))?;
        if pressed {
            return Ok(true);
        }
    }

    Ok(false)
}

fn get_modifiers(
    lua: &Lua,
    modifiers_tbl: Option<LuaTable>,
) -> Result<KeyEventModifiers, LuaError> {
    let resolve = |key: &str, fallback_keys: &[&str]| -> Result<bool, LuaError> {
        if let Some(tbl) = &modifiers_tbl {
            if let Ok(value) = tbl.get(key) {
                return Ok(value);
            }
        }
        get_modifier_from_love(lua, fallback_keys)
    };

    Ok(KeyEventModifiers {
        alt: resolve("alt", &["lalt", "ralt"])?,
        ctrl: resolve("ctrl", &["lctrl", "rctrl"])?,
        meta: resolve("gui", &["lgui", "rgui"])?,
        shift: resolve("shift", &["lshift", "rshift"])?,
    })
}

pub fn keyboard_key(
    lua: &Lua,
    view: &View,
    event_type: KeyEventType,
    key: &str,
    is_repeat: Option<bool>,
    modifiers_tbl: Option<LuaTable>,
) -> LuaResult<()> {
    let lib = renderer_get_lib();

    let key_code = match event_type {
        KeyEventType::Char => VirtualKeyCode::Unknown,
        _ => {
            let code = key_to_key_code(key);

            if matches!(code, VirtualKeyCode::Unknown) {
                return Ok(());
            }

            code
        }
    };

    let modifiers = get_modifiers(lua, modifiers_tbl)?;
    let is_system_key = modifiers.alt;

    let info = KeyEventCreationInfo {
        ty: event_type,
        modifiers: modifiers,
        virtual_key_code: key_code,
        native_key_code: 0,
        text: key,
        unmodified_text: key,
        is_keypad: false,
        is_auto_repeat: is_repeat.unwrap_or(false),
        is_system_key: is_system_key,
    };

    let event = KeyEvent::new(lib, info).map_err(|e| LuaError::external(e))?;
    view.fire_key_event(event);

    Ok(())
}
