use crate::*;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

pub struct ComponentRegistry {
    registry: Mutex<HashMap<String, Arc<dyn Fn() -> Box<dyn traits::RustBehaviour> + Send + Sync>>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            registry: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_component<F>(&self, name: &str, constructor: F)
    where
        F: Fn() -> Box<dyn traits::RustBehaviour> + Send + Sync + 'static,
    {
        let mut registry = self.registry.lock().unwrap();
        registry.insert(name.to_string(), Arc::new(constructor));
    }

    pub fn create(&self, name: &str) -> Option<Box<dyn traits::RustBehaviour>> {
        let registry = self.registry.lock().unwrap();
        registry.get(name).map(|constructor| constructor())
    }
}

pub struct Component {
    pub name: String,
    behaviour: Box<dyn traits::RustBehaviour>,
}

impl Component {
    pub fn new(name: &str, engine: engine::Engine) -> Result<Self, String> {
        if let Some(mut component) = engine.create_component(name) {
            Ok(Self {
                name: name.to_string(),
                behaviour: component,
            })
        } else {
            Err(String::from_str("Component doesn't exist").unwrap())
        }
    }

    pub fn update(&mut self) {
        self.behaviour.on_update(60.0);
    }
}
