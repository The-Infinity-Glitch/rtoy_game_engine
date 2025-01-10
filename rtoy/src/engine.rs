use crate::component::ComponentRegistry;
use crate::*;
use libloading;
use std::sync::{Arc, Mutex};

pub struct Engine {
    components: Vec<libloading::Library>,
    component_registry: Arc<Mutex<ComponentRegistry>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            component_registry: Arc::new(Mutex::new(ComponentRegistry::new())),
        }
    }

    pub fn load_component(&mut self, path: &str) {
        unsafe {
            let lib = libloading::Library::new(path).expect("Failed to load component library");
            let component_entry_point: libloading::Symbol<
                unsafe extern "C" fn(&Arc<Mutex<ComponentRegistry>>),
            > = lib
                .get(b"rtoy_engine_entry_point")
                .expect("Failed to find component entry point");
            component_entry_point(&self.component_registry);
            self.components.push(lib);
        }
    }

    pub fn create_component(&self, name: &str) -> Option<Box<dyn traits::RustBehaviour>> {
        self.component_registry.lock().unwrap().create(name)
    }
}
