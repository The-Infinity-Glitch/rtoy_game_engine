use rtoy_engine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = rtoy_engine::engine::RToyEngine::new();

    engine.load_component("../../tests/lua_component/my_component.lua")?;

    for component in engine.clone().list_components() {
        println!("{} registered", component);
    }

    let my_component = engine.get_component("my_component.lua")?;

    my_component.ready()?;

    for _ in 0..10 {
        my_component.update(10.0)?;
    }

    Ok(())
}
