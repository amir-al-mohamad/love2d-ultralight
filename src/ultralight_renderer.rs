use crate::clipboard::Clipboard;
use crate::filesystem::FileSystem;
use mlua::prelude::*;
use std::{cell::RefCell, sync::Arc};
use ul_next::{Library, config::Config, platform, renderer::Renderer};

thread_local! {
    static ULTRALIGHT_RENDERER: RefCell<Option<UltralightRenderer>> = RefCell::new(None);
}

pub struct UltralightRenderer {
    lib: Arc<Library>,
    renderer: Arc<Renderer>,
}

impl UltralightRenderer {
    pub fn init(lua: &Lua) -> Result<Self, LuaError> {
        let ul_lib = Library::linked();
        let cache_path = Self::get_cache_path(lua)?;
        let config = Config::start()
            .cache_path(&cache_path)
            .build(ul_lib.clone())
            .ok_or_else(|| LuaError::external("Failed to create config"))?;

        platform::enable_platform_fontloader(ul_lib.clone());
        platform::set_filesystem(ul_lib.clone(), FileSystem);
        platform::set_clipboard(ul_lib.clone(), Clipboard);

        let renderer = Renderer::create(config)
            .map_err(|e| LuaError::external(format!("Failed to create renderer: {}", e)))?;

        Ok(UltralightRenderer {
            lib: ul_lib,
            renderer: Arc::new(renderer),
        })
    }

    fn get_cache_path(lua: &Lua) -> Result<String, LuaError> {
        let globals = lua.globals();
        let love_table: LuaTable = globals.get("love")?;
        let filesystem_table: LuaTable = love_table.get("filesystem")?;
        let get_save_directory: LuaFunction = filesystem_table.get("getSaveDirectory")?;
        let save_dir: String = get_save_directory.call(())?;
        Ok(save_dir)
    }

    pub fn update(&self) {
        self.renderer.update();
    }

    pub fn draw(&self) {
        self.renderer.refresh_display(0);
        self.renderer.render();
    }

    pub fn get_lib(&self) -> Arc<Library> {
        self.lib.clone()
    }

    pub fn get_renderer(&self) -> Arc<Renderer> {
        self.renderer.clone()
    }
}

pub fn renderer_get_lib() -> Arc<Library> {
    ULTRALIGHT_RENDERER.with(|cell| {
        cell.borrow()
            .as_ref()
            .expect("Renderer not initialized")
            .get_lib()
    })
}

pub fn renderer_get_renderer() -> Arc<Renderer> {
    ULTRALIGHT_RENDERER.with(|cell| {
        cell.borrow()
            .as_ref()
            .expect("Renderer not initialized")
            .get_renderer()
    })
}

pub fn renderer_init(lua: &Lua) -> LuaResult<()> {
    ULTRALIGHT_RENDERER.with(|cell| {
        let mut singleton = cell.borrow_mut();
        if singleton.is_none() {
            let renderer = UltralightRenderer::init(lua)?;
            *singleton = Some(renderer);
        }
        Ok(())
    })
}

pub fn renderer_update() {
    ULTRALIGHT_RENDERER.with(|cell| {
        if let Some(ref renderer) = *cell.borrow() {
            renderer.update();
        }
    });
}

pub fn renderer_draw() {
    ULTRALIGHT_RENDERER.with(|cell| {
        if let Some(ref renderer) = *cell.borrow() {
            renderer.draw();
        }
    });
}

pub fn renderer_quit() {
    ULTRALIGHT_RENDERER.with(|cell| {
        if let Some(ref mut renderer) = *cell.borrow_mut() {
            renderer.update();
        }
        *cell.borrow_mut() = None;
    });
}
