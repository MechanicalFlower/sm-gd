#![allow(unused_variables)]

mod states;
mod cube;
mod machine;
mod resource;

use gdnative::prelude::{godot_init, InitHandle};

/// Registers all exposed classes to Godot.
fn init(handle: InitHandle) {
    handle.add_class::<cube::Cube>();
    handle.add_class::<resource::CubeResource>();
}

// Macros that create the entry-points of the dynamic library.
godot_init!(init);
