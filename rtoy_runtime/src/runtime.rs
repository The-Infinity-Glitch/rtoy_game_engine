use rtoy_engine;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RToyRuntimeData {
    // Application settings
    pub name: String,

    // Rendering/Window settings
    pub viewport_width: u32,
    pub viewport_heigth: u32,
    pub window_mode: rtoy_engine::window::WindowMode,
}

impl RToyRuntimeData {
    pub fn new() -> Result<Self, String> {
        let settings_file = match rtoy_engine::utils::fs::read_file("resources/settings.ron") {
            Ok(data) => data,
            Err(code) => return Err(code),
        };

        let result: Self = match ron::from_str(settings_file.1.as_str()) {
            Ok(data) => data,
            Err(code) => return Err(format!("Game data error -> {}", code.to_string())),
        };

        Ok(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuntimeResources {
    pub components: Vec<String>,
}

impl RuntimeResources {
    pub fn new() -> Result<Self, String> {
        let resource_file = match rtoy_engine::utils::fs::read_file("resources/resources.ron") {
            Ok(data) => data,
            Err(code) => return Err(code),
        };

        let result: Self = match ron::from_str(resource_file.1.as_str()) {
            Ok(data) => data,
            Err(code) => return Err(format!("Game resources error -> {}", code.to_string())),
        };

        Ok(result)
    }
}

pub enum RToyRuntimeWindowBackend {
    Glfw,
}

pub struct RToyRuntime {
    runtime_data: Option<RToyRuntimeData>,
    resources: Option<RuntimeResources>,
    engine: rtoy_engine::engine::RToyEngine,
    window_backend: Option<rtoy_engine::backend::WindowBackend>,
    root_window: Option<rtoy_engine::window::Window>,
    message_stack: rtoy_engine::message::MessageStack,
}

impl RToyRuntime {
    pub fn new() -> Self {
        Self {
            runtime_data: None,
            resources: None,
            engine: rtoy_engine::engine::RToyEngine::new(),
            window_backend: None,
            root_window: None,
            message_stack: rtoy_engine::message::MessageStack::new(),
        }
    }

    pub fn load_resources(&mut self) {
        match &self.resources {
            Some(resources) => {
                for component in resources.components.clone() {
                    self.message_stack.collect_error(
                        self.engine
                            .load_component(format!("resources/{}", component).as_str()),
                    );
                }
            }
            None => {}
        }
    }

    pub fn init(&mut self, window_backend: RToyRuntimeWindowBackend) {
        self.runtime_data = Some(self.message_stack.collect_error(RToyRuntimeData::new()));
        self.resources = Some(self.message_stack.collect_error(RuntimeResources::new()));
        self.load_resources();

        match window_backend {
            RToyRuntimeWindowBackend::Glfw => {
                self.window_backend = Some(rtoy_engine::backend::WindowBackend::GlfwBackend(
                    self.message_stack
                        .collect_error(rtoy_engine::backend::glfw::Glfw::new(
                            (3, 3),
                            rtoy_engine::backend::glfw::glfw::OpenGlProfileHint::Core,
                        )),
                ));
            }
        }

        self.root_window = Some(self.message_stack.collect_error(
            rtoy_engine::window::Window::new(
                self.window_backend.as_ref().unwrap().clone(),
                self.runtime_data.as_ref().unwrap().name.as_str(),
                self.runtime_data.as_ref().unwrap().viewport_width,
                self.runtime_data.as_ref().unwrap().viewport_heigth,
                self.runtime_data.as_ref().unwrap().window_mode.clone(),
            ),
        ));
    }

    pub fn run(&mut self) {
        let test_component = self
            .message_stack
            .collect_error(self.engine.clone().get_component("player_movement"));

        self.message_stack.collect_error(test_component.ready());

        loop {
            let events = &self.root_window.as_ref().unwrap().process_window_events();

            if events.len() > 0 {
                dbg!(events);
            }

            self.message_stack
                .collect_error(test_component.update(10.0));

            if self.root_window.as_ref().unwrap().window_should_close() {
                break;
            }

            self.window_backend.as_mut().unwrap().poll_events();
        }
    }
}
