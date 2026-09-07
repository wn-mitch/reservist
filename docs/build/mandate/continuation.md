# Continuation

**ID:** `mandate.continuation`
**Status:** `mandate`
**Depends on:** `mandate.baseline`, `mandate.evidence-obligations-m1-m2`, `mandate.evidence-obligations-m3-m4`

The build review is the as-built evidence tree and is updated at each coherent checkpoint. It records the current runtime, runnable path, accepted milestones, work-package results, boundaries, and focused research checkpoints. It does not accumulate an execution diary or change canonical design.

Each evidence statement identifies inspected implementation or observed execution and the revision or working change concerned. Uncommitted work is described honestly. A passing check is reported only after execution. Missing runtime, fixture, catalog, calibration, or player-study work remains explicit rather than inferred from types, plans, or design prose.

A fresh agent starts at `CLAUDE.md`, follows the current design, mandate, and build review in order, verifies the baseline identities relevant to the task, and edits the narrow existing owner. Documentation changes run `just docs-check`; catalog changes run catalog validation and tests; scenario changes use the established freeze and replay flow; production changes run the relevant focused exercise and full repository gate before completion.

Repository history is the recovery path for superseded plans and reports. Do not create active archive copies, redirect stubs, duplicate current rules, or alternate evidence ledgers.
