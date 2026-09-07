# Client and repository boundary

**ID:** `design.runtime.client-and-repository-boundary`
**Status:** `canonical`
**Depends on:** `design.state.canonical-state-and-ownership`

Reservist uses a pure Rust simulation core, a Rust content compiler, and a thin `godot-rust` GDExtension bridge in one process. Rust owns canonical state, scheduling, markets, accounting, cognition, evidence, witnesses, persistence, replay, validation, and scoring. Godot owns windows, declarative scenes, `Control` layout, input, audio, imported assets, animation, and presentation.

Godot submits typed player commands or authored option IDs. Rust validates and executes them, then returns receipts and immutable player-safe projections. Godot never holds canonical-state references. GDScript may perform local presentation behavior only; it may not own causal state, persistence, mechanics, cognition, economic decisions, or consequential scheduling. Serialization for replay and debugging does not create an external IPC protocol.

The authoritative catalog, scenario sources, Rust crates, Godot project, tests, and approved assets live in one version-controlled repository. Runtime play consumes compiled frozen scenario data. No network or language-model dependency exists inside executable game bounds.

Godot's importer is the initial asset cooker. Source assets retain stable IDs, provenance, rights, dimensions, crop, focal point, and role. Presentation build identity remains separate from simulation identity; generated import caches stay outside source control.
