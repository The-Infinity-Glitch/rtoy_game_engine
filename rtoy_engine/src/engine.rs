use crate::component::LuaComponent;
use crate::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct RToyEngine {
    components: HashMap<String, LuaComponent>,
}

impl RToyEngine {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn load_component(&mut self, path: &str) -> Result<(), String> {
        let new_component = match LuaComponent::new(path) {
            Ok(component) => component,
            Err(code) => return Err(code),
        };

        let component_name = match utils::fs::read_file(path) {
            Ok(name) => name.0,
            Err(code) => return Err(code),
        };

        if self.components.contains_key(&component_name) {
            return Err(format!("\"{}\" already registered", component_name));
        }

        self.components.insert(component_name, new_component);

        Ok(())
    }

    pub fn get_component(self, component_name: &str) -> Result<LuaComponent, String> {
        match self.components.get(&component_name.to_string()) {
            Some(component) => return Ok(component.to_owned()),
            None => return Err(format!("\"{}\" doesn't exists", component_name)),
        }
    }

    pub fn list_components(self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        for component in self.components {
            result.push(component.0);
        }

        result
    }
}
