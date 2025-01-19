use rtoy_engine;
mod runtime;

fn main() {
    let mut runtime = runtime::RToyRuntime::new();
    runtime.init(runtime::RToyRuntimeWindowBackend::Glfw);

    runtime.new_window("Test", 800, 600, rtoy_engine::window::WindowMode::Windowed);

    runtime.run();

    /*
    let mut message_stack = rtoy_engine::message::MessageStack::new();

    message_stack
        .collect_error(engine.load_component("../../tests/lua_component/my_component.lua"));

    for component in engine.clone().list_components() {
        println!("{} registered", component);
    }

    let my_component = message_stack.collect_error(engine.get_component("my_component"));

    message_stack.collect_error(my_component.ready());

    let mut backend = rtoy_engine::backend::WindowBackend::GlfwBackend(
        message_stack.collect_error(rtoy_engine::backend::glfw::Glfw::new(
            (3, 3),
            rtoy_engine::backend::glfw::glfw::OpenGlProfileHint::Core,
        )),
    );

    let window = message_stack.collect_error(rtoy_engine::window::Window::new(
        backend.clone(),
        "Test",
        800,
        600,
        rtoy_engine::window::WindowMode::Windowed,
    ));

    for _ in 0..10 {
        message_stack.collect_error(my_component.update(10.0));
    }

    loop {
        let events = window.process_window_events();

        if events.len() > 0 {
            dbg!(events);
        }

        backend.poll_events();
    }
    */
}
