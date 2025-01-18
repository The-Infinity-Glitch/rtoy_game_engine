mod message;

use rtoy_engine;

fn main() {
    let mut engine = rtoy_engine::engine::RToyEngine::new();
    let mut message_stack = message::MessageStack::new();

    message_stack
        .collect_error(engine.load_component("../../tests/lua_component/my_component.lua"));

    for component in engine.clone().list_components() {
        println!("{} registered", component);
    }

    let my_component = message_stack.collect_error(engine.get_component("my_component"));

    message_stack.collect_error(my_component.ready());

    for _ in 0..10 {
        message_stack.collect_error(my_component.update(10.0));
    }
}
