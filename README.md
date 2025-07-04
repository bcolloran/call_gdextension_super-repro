# GDExtension Super Method Call Reproduction Case

This repository demonstrates a limitation in Godot 4.x when calling parent methods from GDScript classes that extend GDExtension classes.

## Issue Description

When a GDScript class extends a class implemented in a GDExtension (Rust in this case), calling `super` methods fails with a parse error, even though the method is properly implemented in the parent class.

### Reproduction Steps

1. The Tester class is implemented in Rust as a GDExtension with a proper [`ready()`](rust/src/lib.rs ) method implementation
2. The TesterSubclass extends this class in GDScript
3. When trying to call `super._ready()` from the GDScript subclass, the following error occurs:

```
ERROR: Cannot call the parent class' virtual function "_ready()" because it hasn't been defined.
```

This is incorrect because the parent class *does* define the `_ready()` method in the Rust implementation.

## Project Structure

- [`rust`](rust ): Contains the Rust GDExtension implementation
  - [`rust/src/lib.rs`](rust/src/lib.rs ): Defines the `Tester` class with its methods
- [`godot`](godot ): Contains the Godot project
  - [`godot/tester_subclass.gd`](godot/tester_subclass.gd ): The GDScript that attempts to extend the Rust class
  - [`godot/test_scene.tscn`](godot/test_scene.tscn ): A scene that includes both a plain Tester and TesterSubclass

## Building and Testing

1. Build the Rust extension:
   ```bash
   cd rust
   cargo build
   ```

2. Open the Godot project and run the test scene
3. Uncomment the `super._ready()` line in [`godot/tester_subclass.gd`](godot/tester_subclass.gd ) to see the error

## Environment

- Godot 4.4.1
- Rust with godot crate 0.3.2
