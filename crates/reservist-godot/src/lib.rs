mod serialization;
mod session_node;

use godot::init::{ExtensionLibrary, gdextension};

struct ReservistExtension;

// SAFETY: registration uses godot-rust's generated ABI and library lifecycle.
#[gdextension]
unsafe impl ExtensionLibrary for ReservistExtension {}
