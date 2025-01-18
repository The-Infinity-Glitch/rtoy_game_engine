use crate::*;
use std::str::FromStr;

pub use glfw;

#[derive(Clone)]
pub struct Glfw {
    context: glfw::Glfw,
}

impl Glfw {
    pub fn new(
        context_version: (u32, u32),
        profile: glfw::OpenGlProfileHint,
    ) -> Result<Self, String> {
        let mut glfw = match glfw::init(glfw_error_callback_function) {
            Ok(glfw) => glfw,
            Err(code) => return Err(format!("Failed to initialize glfw -> {}", code.to_string())),
        };
        glfw.window_hint(glfw::WindowHint::ContextVersion(
            context_version.0,
            context_version.1,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(profile));

        Ok(Self { context: glfw })
    }

    pub fn get_context(&self) -> glfw::Glfw {
        self.context.clone()
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events();
    }
}

pub fn glfw_error_callback_function(_err: glfw::Error, _description: String) {}

pub struct GlfwWindow {
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl GlfwWindow {
    pub fn new(
        glfw_context: Glfw,
        title: &str,
        width: u32,
        height: u32,
        mode: window::WindowMode,
    ) -> Result<Self, String> {
        let (mut window, events) = match glfw_context.get_context().create_window(
            width,
            height,
            title,
            Self::window_mode_to_glfw_window_mode(mode),
        ) {
            Some(ok) => ok,
            None => return Err(String::from_str("Failed to create a new glfw window").unwrap()),
        };

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        Ok(Self { window, events })
    }

    pub fn window_mode_to_glfw_window_mode<'a>(mode: window::WindowMode) -> glfw::WindowMode<'a> {
        match mode {
            window::WindowMode::Windowed => glfw::WindowMode::Windowed,
            window::WindowMode::FullScreen => glfw::WindowMode::Windowed,
        }
    }

    pub fn process_glfw_window_events(&self) -> Vec<input::Key> {
        let mut events: Vec<input::Key> = Vec::new();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                    events.push(input::Key::from_glfw(key, scancode, action, modifiers));
                }
                _ => {}
            }
        }

        events
    }
}
