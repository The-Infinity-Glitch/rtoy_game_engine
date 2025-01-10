use rtoy::*;

#[component]
pub struct MyComponent {
    pub count: usize,
}

impl RustBehaviour for MyComponent {
    pub fn init() -> Self {
        Self { count: 0 }
    }

    fn on_ready(&mut self) {
        println!("MyComponent ready!");
        self.count += 1;
        println!("> {}", self.count);
    }

    fn on_update(&mut self, delta: f32) {
        println!("MyComponent update!");

        if Input::is_action_just_pressed(KeyInput::Espace) {
            self.count += 1;
            println!("{}", self.count);
        }
    }
}
