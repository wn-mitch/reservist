# Current runtime

**ID:** `evidence.current-runtime`
**Status:** `evidence`
**Depends on:** `none`
**Verifies:** `mandate.milestone-order`, `design.runtime.client-and-repository-boundary`

**Inspected on accepted M1/M2 change `mkstwyuv`:** `reservist-core` owns causal state, capacity, history, persistence, markets, accounting, evidence, and scoring. `reservist-content` validates the global authored catalog and compiles frozen scenario slices. `reservist-godot` exposes typed session commands and immutable player-safe projections. GDScript remains presentation-only. Diagnostic snapshots and whole-run export are CLI tooling, not gameplay state.

Production commands are native Rust. Python remains in the retained oracle and authoring-side catalog tests. The runtime is pinned to Rust 1.98.1 and godot-rust 0.5.5 with the Godot 4.7 extension API. The observed Godot runtime was 4.7.2.

M2 starts paused. Reading and previewing do not advance the calendar or reserve capacity. Submission uses explicit typed commands. Unchosen proposals do not become default policy. Supported saves retain queue, reservations, command idempotency, interruption state, admitted work, and scorecard state at quiescent boundaries.

No inspected type or interface is claimed as M3 succession gameplay, full Stewardship lifecycle, M4 provider response, broader world content, economic calibration, or player-comprehension evidence.
