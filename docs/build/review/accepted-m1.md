# Accepted M1

**ID:** `evidence.accepted-m1`
**Status:** `evidence`
**Depends on:** `evidence.current-runtime`
**Verifies:** `mandate.baseline`, `mandate.evidence-obligations-m1-m2`

**Observed on change `mkstwyuv`:** the one-way native replacement passed `just check`. The workspace reported 113 Rust tests across CLI, content, frozen scenario, core, canonical JSON, and fidelity. Strict formatting and Clippy passed. Dependency checks, Godot source boundaries, actual Godot extension loading, 18 Python catalog tests, 86 retained oracle tests, scenario validation, and 20 native/oracle parity vectors passed.

The M1 fixture retained scenario hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066`. All three native package replays matched the oracle. Resealing was a byte-preserving no-op. Generated fidelity, handler, and phase metadata were identical across repeated generation.

Negative fixtures rejected duplicate owned-state contracts, unresolved owners, missing handlers and witnesses, incompatible schemas, invalid fidelity, unknown transitions, wrong-phase scheduling, missing opening state, and selectable incomplete catalog contracts. Additional regressions covered units, providers, sequence duplication, residuals, ambiguous writers, and reverse phase feedback.

A deliberate isolated transcript framing mutation caused parity exit 1 with one unattributed difference, while pinned vectors remained unchanged. Production contained no Python sidecar.
