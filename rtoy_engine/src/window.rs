use crate::*;

pub enum WindowMode {
    Windowed,
    FullScreen,
}

pub enum WindowData {
    GlfwWindow(backend::glfw::GlfwWindow),
}

pub struct Window {
    backend: backend::WindowBackend,
    data: WindowData,
}

impl Window {
    pub fn new(
        backend: backend::WindowBackend,
        title: &str,
        width: u32,
        height: u32,
        mode: WindowMode,
    ) -> Result<Self, String> {
        match backend.clone() {
            backend::WindowBackend::GlfwBackend(glfw) => {
                let window = match backend::glfw::GlfwWindow::new(glfw, title, width, height, mode)
                {
                    Ok(ok) => ok,
                    Err(code) => return Err(code),
                };

                Ok(Self {
                    backend,
                    data: WindowData::GlfwWindow(window),
                })
            }
        }
    }

    pub fn window_should_close(&self) -> bool {
        match self.backend {
            backend::WindowBackend::GlfwBackend(_) => match &self.data {
                WindowData::GlfwWindow(glfw_window) => return glfw_window.window_should_close(),
            },
        }
    }

    pub fn process_window_events(&self) -> Vec<input::Key> {
        match self.backend {
            backend::WindowBackend::GlfwBackend(_) => match &self.data {
                WindowData::GlfwWindow(glfw_window) => {
                    return glfw_window.process_glfw_window_events()
                }
            },
        };
    }
}
