use godot::classes::Node;
use godot::prelude::*;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct Tester {
    base: Base<Node>,

    /// This is an example of some data that we'd like to store in
    /// the base class (and which we can use to distinguish between
    /// instances of the class during debugging).
    #[var]
    init_timestamp: u32,
    /// This is an example of some data that we'd like to store in
    /// the base class, but that we want to initialize during the
    /// `ready` method. To properly initialize this data in a GdScript
    /// subclass we need to ba able to call `super._ready()` from that
    /// subclass.
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
