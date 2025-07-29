use mlua::prelude::*;
use ul_next::View;
use ul_next::javascript::{AsJSValue, JSObject, JSPropertyAttributes, JSValue};

fn js_value_to_lua_value(
    lua: &Lua,
    js: &ul_next::javascript::JSContext,
    js_value: &JSValue,
) -> Result<mlua::Value, LuaError> {
    if JSValue::is_array(js_value) || JSValue::is_typed_array(js_value) {
        if let Ok(obj) = js_value.as_object() {
            if let Ok(length) = obj.get_property("length") {
                if let Ok(len) = length.as_number() {
                    let tbl = lua.create_table()?;
                    for i in 0..len as usize {
                        if let Ok(item) = obj.get_property(&i.to_string()) {
                            tbl.set(i + 1, js_value_to_lua_value(lua, js, &item)?)?;
                        }
                    }

                    return Ok(mlua::Value::Table(tbl));
                }
            }
        }

        Ok(mlua::Value::Nil)
    } else if JSValue::is_boolean(js_value) {
        Ok(mlua::Value::Boolean(js_value.as_boolean()))
    } else if JSValue::is_null(js_value) {
        Ok(mlua::Value::Nil)
    } else if JSValue::is_number(js_value) {
        Ok(mlua::Value::Number(js_value.as_number().unwrap_or(0.0)))
    } else if JSValue::is_object(js_value) {
        let tbl = lua.create_table()?;
        if let Ok(obj) = js_value.as_object() {
            if obj.is_function() {
                return Ok(mlua::Value::Nil);
            }

            let keys = obj.get_property_names();
            for i in 0..keys.len() {
                if let Some(key) = keys.get(i) {
                    if let Ok(value) =
                        obj.get_property_for_key(&JSValue::from_jsstring(js, key.clone()))
                    {
                        let key_str = key.to_string();
                        if let Ok(num) = key_str.parse::<i64>() {
                            tbl.set(num, js_value_to_lua_value(lua, js, &value)?)?;
                        } else {
                            tbl.set(key_str, js_value_to_lua_value(lua, js, &value)?)?;
                        }
                    }
                }
            }
        }
        Ok(mlua::Value::Table(tbl))
    } else if JSValue::is_string(js_value) {
        if let Ok(str_val) = js_value.as_string() {
            return Ok(mlua::Value::String(
                lua.create_string(&str_val.to_string())?,
            ));
        }

        Ok(mlua::Value::Nil)
    } else {
        Ok(mlua::Value::Nil)
    }
}

fn is_sequential_table(tbl: &mlua::Table) -> bool {
    let len = tbl.len().unwrap_or(0);
    for i in 1..=len {
        if tbl.get(i).unwrap_or(mlua::Value::Nil) == mlua::Value::Nil {
            return false;
        }
    }

    for pair in tbl.pairs::<mlua::Value, mlua::Value>() {
        if let Ok((key, _)) = pair {
            match key {
                mlua::Value::Integer(i) => {
                    if i < 1 || i > len as i64 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
    true
}

fn lua_value_to_js_value<'a>(
    js_ctx: &'a ul_next::javascript::JSContext,
    ret_vals: mlua::Value,
) -> Result<JSValue<'a>, LuaError> {
    if let mlua::Value::Boolean(b) = ret_vals {
        return Ok(JSValue::new_boolean(js_ctx, b));
    } else if let mlua::Value::Integer(i) = ret_vals {
        return Ok(JSValue::new_number(js_ctx, i as f64));
    } else if let mlua::Value::Nil = ret_vals {
        return Ok(JSValue::new_null(js_ctx));
    } else if let mlua::Value::Number(n) = ret_vals {
        return Ok(JSValue::new_number(js_ctx, n));
    } else if let mlua::Value::String(s) = ret_vals {
        if let Ok(str_val) = s.to_str() {
            return Ok(JSValue::new_string(js_ctx, &str_val));
        }
    } else if let mlua::Value::Table(tbl) = ret_vals {
        if is_sequential_table(&tbl) {
            let mut array_items = vec![];

            for i in 1..=tbl.len().unwrap_or(0) {
                if let Ok(value) = tbl.get(i) {
                    if let Ok(js_value) = lua_value_to_js_value(js_ctx, value) {
                        array_items.push(js_value);
                    }
                }
            }

            return JSObject::new_array(js_ctx, &array_items)
                .map(|js_array| js_array.as_value().clone())
                .map_err(|e| LuaError::external(e.as_string().unwrap().to_string()));
        } else {
            let js_obj = JSObject::new(&js_ctx);

            for pair in tbl.pairs::<String, mlua::Value>() {
                match pair {
                    Ok((key_str, value)) => {
                        if let Ok(js_value) = lua_value_to_js_value(js_ctx, value) {
                            js_obj
                                .set_property(&key_str, &js_value, JSPropertyAttributes::default())
                                .map_err(|e| {
                                    LuaError::external(e.as_string().unwrap().to_string())
                                })?;
                        }
                    }
                    Err(e) => return Err(LuaError::external(e.to_string())),
                }
            }

            return Ok(js_obj.as_value().clone());
        }
    }

    Ok(JSValue::new_undefined(js_ctx))
}

#[derive(Default)]
pub struct UltralightViewCallbacks {
    lua: Lua,
}

impl Drop for UltralightViewCallbacks {
    fn drop(&mut self) {
        self.lua.expire_registry_values();
    }
}

impl UltralightViewCallbacks {
    pub fn new(lua: Lua) -> Self {
        UltralightViewCallbacks { lua }
    }

    fn call_lua_callback_with_result<'a>(
        lua: &Lua,
        registry_key: &LuaRegistryKey,
        js_ctx: &'a ul_next::javascript::JSContext,
        args: &[JSValue],
    ) -> Result<JSValue<'a>, LuaError> {
        let mut lua_args = LuaMultiValue::new();

        for arg in args {
            lua_args.push_back(js_value_to_lua_value(lua, js_ctx, arg)?);
        }

        if let Ok(func) = lua.registry_value::<LuaFunction>(registry_key) {
            let result = func
                .call::<mlua::MultiValue>(lua_args)
                .map_err(|e| mlua::Error::external(e.to_string()))?;

            let mut js_values = Vec::new();
            for val in result {
                if let Ok(js_value) = lua_value_to_js_value(js_ctx, val) {
                    js_values.push(js_value);
                }
            }

            match js_values.len() {
                0 => Ok(JSValue::new_undefined(js_ctx)),
                1 => Ok(js_values[0].clone()),
                _ => match JSObject::new_array(js_ctx, &js_values) {
                    Ok(js_array) => Ok(js_array.as_value().clone()),
                    Err(_) => Ok(JSValue::new_undefined(js_ctx)),
                },
            }
        } else {
            Ok(JSValue::new_undefined(js_ctx))
        }
    }

    pub fn add_function(
        &mut self,
        lua: &Lua,
        view: &View,
        namespace: String,
        name: String,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let ctx = view.lock_js_context();
        let global = ctx.global_object();

        let namespace_obj = match global.get_property(&namespace) {
            Ok(existing_obj) => {
                if JSValue::is_object(&existing_obj) {
                    existing_obj.as_object().unwrap().clone()
                } else {
                    JSObject::new(&ctx)
                }
            }
            Err(_) => JSObject::new(&ctx),
        };

        let lua_clone = lua.clone();
        let js_func = JSObject::new_function_with_callback(&ctx, move |js_ctx, _this, args| {
            match Self::call_lua_callback_with_result(&lua_clone, &callback_key, js_ctx, args) {
                Ok(result) => Ok(result),
                Err(e) => {
                    match JSObject::new_error(&js_ctx, JSValue::new_string(js_ctx, &e.to_string()))
                    {
                        Ok(e_obj) => Err(e_obj.into_value()),
                        Err(_) => Ok(JSValue::new_undefined(&js_ctx)),
                    }
                }
            }
        });

        namespace_obj
            .set_property(&name, &js_func, JSPropertyAttributes::default())
            .map_err(|e| mlua::Error::external(e.as_string().unwrap().to_string()))?;

        global
            .set_property(&namespace, &namespace_obj, JSPropertyAttributes::default())
            .map_err(|e| mlua::Error::external(e.as_string().unwrap().to_string()))?;

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_dom_ready_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_dom_ready_callback(move |_, _, is_main_frame, url| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<(bool, String)>((is_main_frame, url));
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_begin_loading_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_begin_loading_callback(move |_, _, is_main_frame, url| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<(bool, String)>((is_main_frame, url));
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_finish_loading_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_finish_loading_callback(move |_, _, is_main_frame, url| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<(bool, String)>((is_main_frame, url));
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_fail_loading_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_fail_loading_callback(
            move |_, _, is_main_frame, url, description, error_domain, error_code| {
                if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                    let _ = func.call::<(bool, String, String, String, u32)>((
                        is_main_frame,
                        url,
                        description,
                        error_domain,
                        error_code,
                    ));
                }
            },
        );

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_window_object_ready_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_window_object_ready_callback(move |_, _, is_main_frame, url| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<(String, bool)>((url, is_main_frame));
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_change_title_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_change_title_callback(move |_, new_title| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<String>(new_title);
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_change_url_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_change_url_callback(move |_, url| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<String>(url);
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_update_history_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_update_history_callback(move |_| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<()>(());
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_create_child_view_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_create_child_view_callback(move |_, source_url, target_url, is_popup, _| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<(String, String, bool)>((source_url, target_url, is_popup));
            }
            None
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_change_tooltip_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_change_tooltip_callback(move |_, tooltip| {
            if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                let _ = func.call::<String>(tooltip);
            }
        });

        lua.expire_registry_values();

        Ok(())
    }

    pub fn set_add_console_message_callback(
        &mut self,
        lua: &Lua,
        view: &View,
        callback: LuaFunction,
    ) -> LuaResult<()> {
        let callback_key = lua.create_registry_value(callback)?;

        let lua_clone = lua.clone();
        view.set_add_console_message_callback(
            move |_,
                  message_source,
                  message_level,
                  message,
                  line_number,
                  column_number,
                  source_id| {
                if let Ok(func) = lua_clone.registry_value::<LuaFunction>(&callback_key) {
                    let _ = func.call::<(isize, isize, String, u32, u32, String)>((
                        message_source as isize,
                        message_level as isize,
                        message,
                        line_number,
                        column_number,
                        source_id,
                    ));
                }
            },
        );

        lua.expire_registry_values();

        Ok(())
    }
}
