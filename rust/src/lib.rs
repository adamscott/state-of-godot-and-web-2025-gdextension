use godot::classes::INode;
use godot::classes::Node;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct HelloRust {
    base: Base<Node>,
}

#[godot_api]
impl INode for HelloRust {
    fn init(base: Base<Node>) -> Self {
        godot_print!("Hello, World!");
        Self { base }
    }
}

struct StateOfTheWeb2025GDExtension;

#[gdextension]
unsafe impl ExtensionLibrary for StateOfTheWeb2025GDExtension {}
