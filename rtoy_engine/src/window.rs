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
}
