/// Implements default funcions of a component for rtoy engine
pub trait RustBehaviour {
    fn on_ready(&mut self) {}
    fn on_update(&mut self, delta: f32) {}
    fn on_physics_update(&mut self, delta: f32) {}
}
