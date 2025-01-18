pub mod glfw;

#[derive(Clone)]
pub enum WindowBackend {
    GlfwBackend(glfw::Glfw),
}

impl WindowBackend {
    pub fn poll_events(&mut self) {
        match self {
            WindowBackend::GlfwBackend(glfw) => glfw.poll_events(),
        }
    }
}

#[derive(Clone)]
pub enum GraphicsBackend {
    OpenGlBackend,
}
