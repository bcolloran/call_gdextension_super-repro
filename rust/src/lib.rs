use godot::classes::Node;
use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct Tester {
    base: Base<Node>,
    #[var]
    init_timestamp: u32,
    #[var]
    ready_timestamp: u32,
}

#[godot_api]
impl INode for Tester {
    fn init(base: Base<Node>) -> Self {
        let init_timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            % 1_000) as u32;

        godot_print!(
            "Tester::init (from Rust); init_timestamp: {}",
            init_timestamp
        );

        Self {
            base,
            init_timestamp,
            ready_timestamp: 0,
        }
    }

    fn ready(&mut self) {
        self.ready_timestamp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            % 1_000) as u32;
        godot_print!(
            "Tester::ready (from Rust); init_timestamp: {}, ready_timestamp: {}",
            self.init_timestamp,
            self.ready_timestamp
        );
    }
}
