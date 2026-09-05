# Reservist

Start here, in order: `docs/CURRENT_DESIGN_BIBLE.md` (what we are building and which text governs), `docs/BUILD_MANDATE.md` (what work is authorized and how completion is judged), `docs/BUILD_REVIEW.md` (what the executable actually does and what is unproved). Design chapters and the adopted decision handoff live in `docs/design/`; the authoring catalog in `catalog/`; production scenarios in `scenarios/`; the retained Python oracle and its tests in `tools/oracle/`; frozen parity vectors and attribution files in `tests/`.

Commands: `just check`, `just test`, `just gates`, `just validate`, `just freeze`, `just replay`, `just run`, `just play`, `just save`, `just resume`, `just parity`, `just catalog-test`, `just catalog-generate`, `just deps-check`, `just godot-lint`, `just godot-test`, `just oracle-test`, `just oracle-vectors`. Production commands are native Rust. Python 3.14 remains in oracle tooling and the authoring-side catalog tests, with no third-party Python dependencies. `just godot-import` builds and imports the native extension; `godot --path godot` opens the playable client. The default scenario is M2; pass `--set scenario scenarios/mvp_2006_cycle_m1` to `just` for the pinned M1 fixture.

Version control is jj. Do not restate game rules here; edit the owning chapter and record the change in the Build Review.
