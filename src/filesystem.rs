use mlua::prelude::*;
use std::{
    cell::RefCell,
    sync::{Arc, LazyLock, Mutex, mpsc},
};
use ul_next::platform;

static REQUIRED_FILES: &[&str] = &["resources/cacert.pem", "resources/icudt67l.dat"];

thread_local! {
    static LUA_FILESYSTEM_FILE_EXISTS_CALLBACK: RefCell<Option<Arc<LuaRegistryKey>>> = RefCell::new(None);
    static LUA_FILESYSTEM_GET_FILE_MIME_TYPE_CALLBACK: RefCell<Option<Arc<LuaRegistryKey>>> = RefCell::new(None);
    static LUA_FILESYSTEM_GET_FILE_CHARSET_CALLBACK: RefCell<Option<Arc<LuaRegistryKey>>> = RefCell::new(None);
    static LUA_FILESYSTEM_GET_FILE_DATA_CALLBACK: RefCell<Option<Arc<LuaRegistryKey>>> = RefCell::new(None);
}

static FILESYSTEM_CHANNELS: LazyLock<(
    mpsc::Sender<(String, mpsc::Sender<bool>)>,
    Mutex<mpsc::Receiver<(String, mpsc::Sender<bool>)>>,
    mpsc::Sender<(String, mpsc::Sender<Option<String>>)>,
    Mutex<mpsc::Receiver<(String, mpsc::Sender<Option<String>>)>>,
    mpsc::Sender<(String, mpsc::Sender<Option<String>>)>,
    Mutex<mpsc::Receiver<(String, mpsc::Sender<Option<String>>)>>,
    mpsc::Sender<(String, mpsc::Sender<Option<Vec<u8>>>)>,
    Mutex<mpsc::Receiver<(String, mpsc::Sender<Option<Vec<u8>>>)>>,
)> = LazyLock::new(|| {
    let (exists_tx, exists_rx) = mpsc::channel();
    let (mime_tx, mime_rx) = mpsc::channel();
    let (charset_tx, charset_rx) = mpsc::channel();
    let (data_tx, data_rx) = mpsc::channel();

    (
        exists_tx,
        Mutex::new(exists_rx),
        mime_tx,
        Mutex::new(mime_rx),
        charset_tx,
        Mutex::new(charset_rx),
        data_tx,
        Mutex::new(data_rx),
    )
});

pub struct FileSystem;

impl platform::FileSystem for FileSystem {
    fn file_exists(&mut self, path: &str) -> bool {
        if REQUIRED_FILES.contains(&path) {
            return true;
        }

        let (tx, rx) = mpsc::channel::<bool>();
        if FILESYSTEM_CHANNELS.0.send((path.to_string(), tx)).is_ok() {
            return rx.recv().unwrap_or(false);
        }

        false
    }

    fn get_file_mime_type(&mut self, path: &str) -> String {
        if REQUIRED_FILES.contains(&path) {
            return "application/unknown".to_string();
        }

        let (tx, rx) = mpsc::channel::<Option<String>>();
        if FILESYSTEM_CHANNELS.2.send((path.to_string(), tx)).is_ok() {
            if let Ok(Some(mime_type)) = rx.recv() {
                return mime_type;
            }
        }

        "application/unknown".to_string()
    }

    fn get_file_charset(&mut self, path: &str) -> String {
        if REQUIRED_FILES.contains(&path) {
            return "utf-8".to_string();
        }

        let (tx, rx) = mpsc::channel::<Option<String>>();
        if FILESYSTEM_CHANNELS.4.send((path.to_string(), tx)).is_ok() {
            if let Ok(Some(charset)) = rx.recv() {
                return charset;
            }
        }

        "utf-8".to_string()
    }

    fn open_file(&mut self, path: &str) -> Option<Vec<u8>> {
        if REQUIRED_FILES.contains(&path) {
            return match path {
                "resources/cacert.pem" => Some(include_bytes!("../assets/cacert.pem").to_vec()),
                "resources/icudt67l.dat" => Some(include_bytes!("../assets/icudt67l.dat").to_vec()),
                _ => None,
            };
        }

        let (tx, rx) = mpsc::channel::<Option<Vec<u8>>>();
        if FILESYSTEM_CHANNELS.6.send((path.to_string(), tx)).is_ok() {
            return rx.recv().unwrap_or(None);
        }

        None
    }
}

pub fn filesystem_set_on_file_exists_callback(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    let callback = Arc::new(lua.create_registry_value(callback)?);

    LUA_FILESYSTEM_FILE_EXISTS_CALLBACK.with(move |cell| {
        *cell.borrow_mut() = Some(callback.clone());
        lua.expire_registry_values();
    });

    Ok(())
}

pub fn filesystem_set_on_get_file_mime_type_callback(
    lua: &Lua,
    callback: LuaFunction,
) -> LuaResult<()> {
    let callback = Arc::new(lua.create_registry_value(callback)?);

    LUA_FILESYSTEM_GET_FILE_MIME_TYPE_CALLBACK.with(move |cell| {
        *cell.borrow_mut() = Some(callback.clone());
        lua.expire_registry_values();
    });

    Ok(())
}

pub fn filesystem_set_on_get_file_charset_callback(
    lua: &Lua,
    callback: LuaFunction,
) -> LuaResult<()> {
    let callback = Arc::new(lua.create_registry_value(callback)?);

    LUA_FILESYSTEM_GET_FILE_CHARSET_CALLBACK.with(move |cell| {
        *cell.borrow_mut() = Some(callback.clone());
        lua.expire_registry_values();
    });

    Ok(())
}

pub fn filesystem_set_on_open_file_callback(lua: &Lua, callback: LuaFunction) -> LuaResult<()> {
    let callback = Arc::new(lua.create_registry_value(callback)?);

    LUA_FILESYSTEM_GET_FILE_DATA_CALLBACK.with(move |cell| {
        *cell.borrow_mut() = Some(callback.clone());
        lua.expire_registry_values();
    });

    Ok(())
}

fn handle_channel<T, F>(
    receiver: &Mutex<mpsc::Receiver<(String, mpsc::Sender<T>)>>,
    lua: &Lua,
    callback_tls: &RefCell<Option<Arc<LuaRegistryKey>>>,
    call_lua: F,
    default: T,
) where
    T: Clone,
    F: Fn(&LuaFunction, String) -> T,
{
    if let Ok(guard) = receiver.lock() {
        while let Ok((path, sync_tx)) = guard.try_recv() {
            let result = callback_tls
                .borrow()
                .as_ref()
                .and_then(|callback| lua.registry_value::<LuaFunction>(callback).ok())
                .map(|func| call_lua(&func, path.clone()))
                .unwrap_or(default.clone());
            let _ = sync_tx.send(result);
        }
    }
}

pub fn filesystem_update(lua: &Lua) {
    LUA_FILESYSTEM_FILE_EXISTS_CALLBACK.with(|callback_tls| {
        handle_channel(
            &FILESYSTEM_CHANNELS.1,
            lua,
            callback_tls,
            |func, path| func.call(path).unwrap_or(false),
            false,
        );
    });

    LUA_FILESYSTEM_GET_FILE_MIME_TYPE_CALLBACK.with(|callback_tls| {
        handle_channel(
            &FILESYSTEM_CHANNELS.3,
            lua,
            callback_tls,
            |func, path| func.call(path).unwrap_or(None),
            None,
        );
    });

    LUA_FILESYSTEM_GET_FILE_CHARSET_CALLBACK.with(|callback_tls| {
        handle_channel(
            &FILESYSTEM_CHANNELS.5,
            lua,
            callback_tls,
            |func, path| func.call(path).unwrap_or(None),
            None,
        );
    });

    LUA_FILESYSTEM_GET_FILE_DATA_CALLBACK.with(|callback_tls| {
        handle_channel(
            &FILESYSTEM_CHANNELS.7,
            lua,
            callback_tls,
            |func, path| {
                func.call(path)
                    .ok()
                    .flatten()
                    .map(|s: LuaString| s.as_bytes().to_vec())
            },
            None,
        );
    });
}
