use crate::*;
use mlua::prelude::*;

#[derive(Clone)]
pub struct LuaComponent {
    lua_state: Lua,
    script: String,
}

impl LuaComponent {
    pub fn new(path: &str) -> Result<Self, String> {
        unsafe {
            let lua = Lua::unsafe_new();
            let script = match utils::fs::read_file(path) {
                Ok(script) => script.1,
                Err(code) => return Err(code),
            };

            match lua.load(script.clone()).exec() {
                Ok(_) => {}
                Err(code) => return Err(code.to_string()),
            };

            Ok(Self {
                lua_state: lua,
                script,
            })
        }
    }

    pub fn get_script(&self) -> String {
        self.script.clone()
    }

    pub fn ready(&self) -> Result<bool, String> {
        if let Some(on_ready_fn) = self
            .lua_state
            .globals()
            .get::<mlua::Function>("on_ready")
            .ok()
        {
            match on_ready_fn.call::<()>(()) {
                Ok(_) => {}
                Err(code) => return Err(code.to_string()),
            };
        }

        Ok(true)
    }

    pub fn update(&self, delta: f32) -> Result<bool, String> {
        if let Some(on_update_fn) = self
            .lua_state
            .globals()
            .get::<mlua::Function>("on_update")
            .ok()
        {
            match on_update_fn.call::<()>(delta) {
                Ok(_) => {}
                Err(code) => return Err(code.to_string()),
            };
        }

        Ok(true)
    }
}
