use rtoy_engine;

pub enum RToyRuntimeWindowBackend {
    Glfw,
}

pub struct RToyRuntime {
    engine: rtoy_engine::engine::RToyEngine,
    window_backend: Option<rtoy_engine::backend::WindowBackend>,
    windows: Vec<rtoy_engine::window::Window>,
    message_stack: rtoy_engine::message::MessageStack,
}

impl RToyRuntime {
    pub fn new() -> Self {
        Self {
            engine: rtoy_engine::engine::RToyEngine::new(),
            window_backend: None,
            windows: Vec::new(),
            message_stack: rtoy_engine::message::MessageStack::new(),
        }
    }

    pub fn init(&mut self, window_backend: RToyRuntimeWindowBackend) {
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
    }

    pub fn new_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
        mode: rtoy_engine::window::WindowMode,
    ) {
        self.windows.push(
            self.message_stack
                .collect_error(rtoy_engine::window::Window::new(
                    self.window_backend.as_ref().unwrap().clone(),
                    title,
                    width,
                    height,
                    mode,
                )),
        );
    }

    pub fn run(&mut self) {
        loop {
            for window in &mut self.windows {
                let events = window.process_window_events();

                if events.len() > 0 {
                    dbg!(events);
                }
            }

            if self.windows.get(0).unwrap().window_should_close() {
                break;
            }

            self.window_backend.as_mut().unwrap().poll_events();
        }
    }
}
