# Reservist: Build Mandate (front door)

**Status:** Authorized after acceptance of decisions D1–D6 in the design review, Part G. M1 and M2 implementation acceptance is recorded in `docs/BUILD_REVIEW.md` on jj change `mkstwyuv`. M3 and M4 remain the subsequent milestones; their runway interfaces do not count as implemented campaign or world mechanics.

## Baseline pin

The original revision and interpreter rows pin the historical oracle. The fixture rows distinguish that preserved input from the adopted executable.

| Item | Value |
|---|---|
| Repository revision | git `97fc926a` (jj `oyrstukq`), "chore: Migrate design corpus and catalog into repo"; parent `main` `8679992a` |
| Interpreter | Python 3.14.3, zero third-party dependencies |
| Scenario | `scenarios/mvp_2006_cycle`, hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066` |
| Preserved M1 fixture | `scenarios/mvp_2006_cycle_m1`, hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066` |
| M2 fixture | `scenarios/mvp_2006_cycle`, hash `sha256:d9f745b948d7cf2d76e364ff08664a0fcf3347a41485e107a18b02297bca2ce4` |
| Oracle vectors | `just replay` transcripts for `WAIT_AND_WARN`, `MEASURED_FIRMING`, `FIRMING_BIAS` (state hashes `424bda…`, `58738b…`, `af3ee7…`); the 86 tests and 10 gates |
| Design source | H `docs/design/15-open-questions-decision-handoff.md` plus `docs/CURRENT_DESIGN_BIBLE.md` precedence |

## Executable target

**M1 — one-way rewrite.** Reimplement in Rust (`reservist-core`, `reservist-content`) with a Godot client (`reservist-godot` via `godot-rust`) the executable behavior of `engine/cli.py` at the pinned revision: `validate`, `freeze`, `run`, `replay-check`, `play` and its twelve REPL verbs, for the three policy packages, preserving every assertion in the 86 tests and 10 gates that is classified "preserved behavior" (amendment v0.3, P-06 reviewer proposal). Add the new-runtime guarantees: exact checkpoint/resume, static `WorkHandler` registration, generated fidelity permission matrix, compiler fail-closed loading. Python leaves the production path; it stays as test tooling only. Reach a validating global catalog or a compiler-enforced quarantine of out-of-slice rows (214 issues today) without flag-flipping.

**M2 — adopted changes.** Chief of staff as a named Person with dated OfficeHolding; calendar board with discretionary activity periods and dated capacity; folder handoff with practical cards; typed commands and immutable projections; extradiegetic scorecard alongside the in-world review.

**M3 — campaign.** Chair succession with institutional continuity; full Stewardship ledger with findings, compensating entries, and the D1 terminal rule.

**M4 — world.** Channel-specific providers with RECORDED/RESPONSIVE execution; `supported_transitions` in the manifest; asset pipeline; broader content.

The milestone order is M1 parity → M2 adopted changes → M3 succession and score → M4 providers and world (decision D4).

## Binding constraints

H in full. D04 causal contracts, D05 Representation Invariants, D09/D10 information boundaries as inherited. No network dependency inside the game (gate 10 preserved). No causal state, persistence, or mechanics in GDScript. Committed history is never rewritten. Tests that enforce accepted behavior are fixed by fixing the implementation, never weakened; the single authorized exception is the gate 09 split under decision D5. Do not change gameplay, ownership, legal authority, evidence access, persistence, approved technology, or scope without an owner decision.

## Builder latitude

Module and crate layout; type and handler names; the named phase roster derived from `engine/clock.py` priorities; queued-work versioning and cancellation; receipt and idempotency representation; frozen-data and save formats (versioned, canonical, hashed); test organization; packaging of registry metadata for the content compiler; toolchain versions (pinned once chosen). Provisional coefficients are permitted only where H leaves them open and must be labeled provisional.

## Baseline blockers (resolved)

- The native workspace, locked toolchains and Godot client now build and execute. Production has no Python sidecar.
- The global catalog validates with zero structural gaps or warnings; all 18 authoring tests pass and generated readiness names the selected manifest.
- Executed evidence for both resolutions and all 23 work packages is in the Build Review.

## Required evidence

G-01 through G-24 as written in amendment v0.3 §6, allocated: M1 owns G-01, G-04 to G-07, G-10 to G-12, G-18, G-19, G-22; M2 owns G-08, G-09, G-13 to G-15, G-21; M3 owns G-16, G-17; M4 owns G-02, G-03 (full matrix), G-20, G-23. G-24 is research. Each piece of evidence is labeled inspected implementation or observed execution, with the revision it concerns.

## Continuation policy

`docs/BUILD_REVIEW.md` is the as-built record and is updated at every coherent checkpoint. Work is committed with jj on task-scoped changes; uncommitted work is described in the Build Review, never hidden. A fresh agent starts from `CLAUDE.md`, reads this mandate and the Build Review, verifies the baseline pin, and continues.
