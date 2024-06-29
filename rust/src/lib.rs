mod player;
mod bullet;

use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
