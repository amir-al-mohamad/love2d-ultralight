use crate::callbacks::UltralightViewCallbacks;
use crate::keyboard::keyboard_key;
use crate::ultralight_renderer::{renderer_get_lib, renderer_get_renderer};
use mlua::UserData;
use mlua::prelude::*;
use ul_next::event::KeyEventType;
use ul_next::event::{MouseButton, MouseEvent, MouseEventType, ScrollEvent, ScrollEventType};
use ul_next::{View, view::ViewConfig};

fn parse_mouse_button(button_code: u8) -> MouseButton {
    match button_code {
        1 => MouseButton::Left,
        2 => MouseButton::Right,
        3 => MouseButton::Middle,
        _ => MouseButton::None,
    }
}

fn fire_mouse_event(
    view: &View,
    event_type: MouseEventType,
    x: i32,
    y: i32,
    button: MouseButton,
) -> LuaResult<()> {
    if matches!(button, MouseButton::None)
        && matches!(
            event_type,
            MouseEventType::MouseDown | MouseEventType::MouseUp
        )
    {
        return Ok(());
    }

    let lib = renderer_get_lib();
    let scale = view.device_scale();

    let x = (x as f64 / scale) as i32;
    let y = (y as f64 / scale) as i32;

    let event = MouseEvent::new(lib, event_type, x, y, button).map_err(mlua::Error::external)?;

    view.fire_mouse_event(event);

    Ok(())
}

pub struct UltralightView {
    pub(crate) callbacks: UltralightViewCallbacks,
    pub(crate) view: View,
}

impl UltralightView {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        let ul_lib = renderer_get_lib();
        let renderer = renderer_get_renderer();

        let view_config = ViewConfig::start()
            .is_transparent(true)
            .font_family_fixed("Consolas")
            .font_family_sans_serif("Segoe UI")
            .font_family_standard("Segoe UI")
            .build(ul_lib)
            .ok_or_else(|| mlua::Error::external("Failed to create view config"))?;

        let session = match renderer.create_session(false, "default") {
            Ok(session) => session,
            Err(e) => {
                return Err(mlua::Error::external(format!(
                    "Failed to create session: {}",
                    e
                )));
            }
        };

        let view = renderer
            .create_view(800, 600, &view_config, Some(&session))
            .ok_or_else(|| mlua::Error::external("Failed to create view"))?;

        Ok(UltralightView {
            view,
            callbacks: UltralightViewCallbacks::new(lua.clone()),
        })
    }
}

impl UserData for UltralightView {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getDimensions", |_, this, ()| {
            Ok((this.view.width() as i32, this.view.height() as i32))
        });

        methods.add_method("getWidth", |_, this, ()| Ok(this.view.width() as i32));

        methods.add_method("getHeight", |_, this, ()| Ok(this.view.height() as i32));

        methods.add_method_mut("setDimensions", |_, this, (width, height): (i32, i32)| {
            this.view.resize(width as u32, height as u32);
            Ok(())
        });

        methods.add_method_mut("setWidth", |_, this, width: i32| {
            this.view.resize(width as u32, this.view.height());
            Ok(())
        });

        methods.add_method_mut("setHeight", |_, this, height: i32| {
            this.view.resize(this.view.width(), height as u32);
            Ok(())
        });

        methods.add_method(
            "getScale",
            |_, this, ()| Ok(this.view.device_scale() as f32),
        );

        methods.add_method_mut("setScale", |_, this, scale: f32| {
            this.view.set_device_scale(scale as f64);

            Ok(())
        });

        methods.add_method_mut("getFrameBuffer", |lua, this, ()| {
            let mut surface = this.view.surface().unwrap();
            let width = surface.width();
            let height = surface.height();

            let mut pixels = surface.lock_pixels().unwrap().to_vec();
            // BGRA -> RGBA
            for chunk in pixels.chunks_exact_mut(4) {
                chunk.swap(0, 2);
            }

            let pixels_res = lua
                .create_string(&pixels)
                .map_err(|e| mlua::Error::external(e))?;

            Ok((pixels_res, width, height))
        });

        methods.add_method("focus", |_, this, ()| {
            this.view.focus();
            Ok(())
        });

        methods.add_method("unfocus", |_, this, ()| {
            this.view.unfocus();
            Ok(())
        });

        methods.add_method("hasFocus", |_, this, ()| Ok(this.view.has_focus()));

        methods.add_method("hasInputFocus", |_, this, ()| {
            Ok(this.view.has_input_focus())
        });

        methods.add_method_mut("loadURL", |_, this, url: String| {
            this.view
                .load_url(&url)
                .map_err(|e| mlua::Error::external(format!("Failed to load URL: {}", e)))?;

            Ok(())
        });

        methods.add_method_mut("loadHTML", |_, this, html: String| {
            this.view
                .load_html(&html)
                .map_err(|e| mlua::Error::external(format!("Failed to load HTML: {}", e)))?;

            Ok(())
        });

        methods.add_method("getURL", |_, this, ()| {
            this.view
                .url()
                .map_err(|e| mlua::Error::external(format!("Failed to get URL: {}", e)))
        });

        methods.add_method("getTitle", |_, this, ()| {
            this.view
                .title()
                .map_err(|e| mlua::Error::external(format!("Failed to get title: {}", e)))
        });

        methods.add_method("isLoading", |_, this, ()| Ok(this.view.is_loading()));

        methods.add_method_mut("refresh", |_, this, ()| {
            this.view.reload();
            Ok(())
        });

        methods.add_method_mut("cancelLoad", |_, this, ()| {
            this.view.stop();
            Ok(())
        });

        methods.add_method("canGoBack", |_, this, ()| Ok(this.view.can_go_back()));

        methods.add_method_mut("goForward", |_, this, ()| {
            if this.view.can_go_forward() {
                this.view.go_forward();
            }
            Ok(())
        });

        methods.add_method("canGoForward", |_, this, ()| Ok(this.view.can_go_forward()));

        methods.add_method_mut("goBack", |_, this, ()| {
            if this.view.can_go_back() {
                this.view.go_back();
            }
            Ok(())
        });

        methods.add_method("mousePress", |_, this, (x, y, button): (i32, i32, u8)| {
            fire_mouse_event(
                &this.view,
                MouseEventType::MouseDown,
                x,
                y,
                parse_mouse_button(button),
            )
        });

        methods.add_method("mouseRelease", |_, this, (x, y, button): (i32, i32, u8)| {
            fire_mouse_event(
                &this.view,
                MouseEventType::MouseUp,
                x,
                y,
                parse_mouse_button(button),
            )
        });

        methods.add_method("mouseMove", |_, this, (x, y): (i32, i32)| {
            fire_mouse_event(
                &this.view,
                MouseEventType::MouseMoved,
                x,
                y,
                MouseButton::None,
            )
        });

        methods.add_method("mouseWheelMove", |_, this, (x, y): (i32, i32)| {
            let lib = renderer_get_lib();
            let event = ScrollEvent::new(lib, ScrollEventType::ScrollByPixel, x, y)
                .map_err(mlua::Error::external)?;

            this.view.fire_scroll_event(event);
            this.view.set_needs_paint(true);
            Ok(())
        });

        methods.add_method(
            "keyPress",
            |lua, this, (scancode, is_repeat, modifiers): (String, bool, Option<LuaTable>)| {
                keyboard_key(
                    lua,
                    &this.view,
                    KeyEventType::KeyDown,
                    &scancode,
                    Some(is_repeat),
                    modifiers,
                )
            },
        );

        methods.add_method(
            "keyRelease",
            |lua, this, (scancode, modifiers): (String, Option<LuaTable>)| {
                keyboard_key(
                    lua,
                    &this.view,
                    KeyEventType::KeyUp,
                    &scancode,
                    None,
                    modifiers,
                )
            },
        );

        methods.add_method(
            "textInput",
            |lua, this, (text, modifiers): (String, Option<LuaTable>)| {
                keyboard_key(lua, &this.view, KeyEventType::Char, &text, None, modifiers)
            },
        );

        methods.add_method(
            "evaluateScript",
            |lua, this, script: String| -> LuaResult<(LuaValue, LuaValue)> {
                match this.view.evaluate_script(&script) {
                    Ok(Ok(value)) => {
                        let lua_value = lua.create_string(&value).map_err(mlua::Error::external)?;
                        Ok((LuaValue::String(lua_value), LuaValue::Nil))
                    }
                    Ok(Err(e)) => {
                        let lua_error = lua
                            .create_string(&format!("Script error: {}", e))
                            .map_err(mlua::Error::external)?;
                        Ok((LuaValue::Nil, LuaValue::String(lua_error)))
                    }
                    Err(e) => {
                        let lua_error = lua
                            .create_string(&format!("Creation error: {}", e))
                            .map_err(mlua::Error::external)?;
                        Ok((LuaValue::Nil, LuaValue::String(lua_error)))
                    }
                }
            },
        );

        methods.add_method_mut(
            "addFunction",
            |lua, this, (namespace, name, callback): (String, String, LuaFunction)| {
                this.callbacks
                    .add_function(lua, &this.view, namespace, name, callback)
            },
        );

        // Event handlers
        methods.add_method_mut("onDocumentReady", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_dom_ready_callback(lua, &this.view, callback)
        });

        methods.add_method_mut(
            "onBeginLoadingDocument",
            |lua, this, callback: LuaFunction| {
                this.callbacks
                    .set_begin_loading_callback(lua, &this.view, callback)
            },
        );

        methods.add_method_mut(
            "onFinishLoadingDocument",
            |lua, this, callback: LuaFunction| {
                this.callbacks
                    .set_finish_loading_callback(lua, &this.view, callback)
            },
        );

        methods.add_method_mut(
            "onFailLoadingDocument",
            |lua, this, callback: LuaFunction| {
                this.callbacks
                    .set_fail_loading_callback(lua, &this.view, callback)
            },
        );

        methods.add_method_mut("onObjectReady", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_window_object_ready_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onChangeTitle", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_change_title_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onChangeTargetUrl", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_change_url_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onUpdateHistory", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_update_history_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onCreateChildView", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_create_child_view_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onChangeTooltip", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_change_tooltip_callback(lua, &this.view, callback)
        });

        methods.add_method_mut("onConsoleMessage", |lua, this, callback: LuaFunction| {
            this.callbacks
                .set_add_console_message_callback(lua, &this.view, callback)
        });
    }
}
