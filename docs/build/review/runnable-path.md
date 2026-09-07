# Runnable path

**ID:** `evidence.runnable-path`
**Status:** `evidence`
**Depends on:** `evidence.current-runtime`
**Verifies:** `mandate.baseline`, `mandate.evidence-obligations-m1-m2`

**Observed on accepted M1/M2 change `mkstwyuv`:** `just play` builds, imports, and opens the Godot client. `just cli-play` runs the retained native terminal client. `just run`, `just validate`, `just freeze`, `just replay`, `just save`, and `just resume` invoke the native CLI against the selected scenario.

`just --set scenario scenarios/mvp_2006_cycle_m1 replay` selects the preserved M1 fixture. `just parity` compares the M1 fixture by default; the CLI also accepts `parity --fixture m1`. `just catalog-test` runs native content tests and Python authoring tests. `just oracle-test` and `just oracle-vectors` retain the historical Python oracle evidence.

`just check` builds and tests the locked workspace, checks formatting and strict Clippy, validates dependency and Godot boundaries, compares native/oracle parity, runs catalog and oracle suites, validates the scenario, and executes Godot boundary tests. `just docs-check` now validates the compiled documentation boundary as part of that gate.

Market laboratory commands remain tooling-only: `just market-lab`, sweep, compare, suite, tier, ablate, policy, ecology, and runtime composition. They do not alter the ordinary session or Godot projection.
