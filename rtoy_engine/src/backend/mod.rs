pub mod glfw;

#[derive(Clone)]
pub enum WindowBackend {
    GlfwBackend(glfw::Glfw),
}

#[derive(Clone)]
pub enum GraphicsBackend {
    OpenGlBackend,
}
