class_name TesterSubclass extends Tester

var set_true_on_ready: bool = false


"""
We want to be able to call `super._ready()` in the subclass,
but uncommenting it causes a parse error (script cannot even run)--

```
ERROR: res://tester_subclass.gd:9 - Parse Error: Cannot call the parent class' virtual function "_ready()" because it hasn't been defined.
ERROR: modules/gdscript/gdscript.cpp:3022 - Failed to load script "res://tester_subclass.gd" with error "Parse error".
ERROR: res://tester_subclass.gd:9 - Parse Error: Cannot call the parent class' virtual function "_ready()" because it hasn't been defined.
ERROR: modules/gdscript/gdscript.cpp:3022 - Failed to load script "res://tester_subclass.gd" with error "Parse error".
ERROR: res://tester_subclass.gd:9 - Parse Error: Cannot call the parent class' virtual function "_ready()" because it hasn't been defined.
```

But this doesn't seem right, because the parent class `Tester` does define `_ready()`, and we know that it is able to run because if the a `Tester` instance is created, it runs `_ready()` and prints the expected output.

"""
func _ready():
  set_true_on_ready = true
  # Uncommenting the next line causes a parse error.
  # super._ready()
  print("TesterSubclass._ready() (from GdScript)")
  print("    set_true_on_ready=", set_true_on_ready)
  print("    init_timestamp=", self.init_timestamp)
  print("    ready_timestamp=", self.ready_timestamp)
