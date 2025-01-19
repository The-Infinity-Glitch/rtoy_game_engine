mod runtime;

fn main() {
    let mut runtime = runtime::RToyRuntime::new();
    runtime.init(runtime::RToyRuntimeWindowBackend::Glfw);

    runtime.run();
}
