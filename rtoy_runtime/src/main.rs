use rtoy;

fn main() {
    let mut engine = rtoy::engine::Engine::new();
    engine.load_component("tests/component_test/target/debug/libcomponent_test.so");

    let my_component = rtoy::component::Component::new("MyComponent", engine);

    my_component.unwrap().update();
}
