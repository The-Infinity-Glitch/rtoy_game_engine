use rtoy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_component =
        rtoy::component::LuaComponent::new("../../tests/lua_component/my_component.lua")?;

    my_component.ready()?;

    for _ in 0..10 {
        my_component.update(10.0)?;
    }

    Ok(())
}
