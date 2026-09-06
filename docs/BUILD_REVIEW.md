# Reservist: Build Review (front door)

**Status:** WP-01 through WP-23 are implemented and verified on jj change `mkstwyuv`. M1 is the native replacement of the pinned executable; M2 is the adopted interaction model. Production uses Rust and Godot. Python remains oracle and catalog-authoring test tooling. M3 campaign mechanics and M4 world expansion are not part of this implementation.

## Runnable path

- `just play` builds, imports, and opens the default Godot client. `just cli-play` opens the retained native M2 terminal client.
- `just run`, `just validate`, `just freeze`, `just replay`, `just save`, and `just resume` use the native CLI.
- `just --set scenario scenarios/mvp_2006_cycle_m1 replay` selects the preserved M1 fixture. `just parity` selects M1 by default; the CLI also accepts `parity --fixture m1`.
- `just check` runs the locked build, native tests, formatting, strict lint for both workspace/tooling and standalone Godot builds, dependency checks, parity, catalog tests, retained oracle tests, scenario validation, and Godot boundary tests.
- Rust is pinned to 1.98.1; godot-rust to 0.5.5. Observed Godot runtime: 4.7.2, using the pinned 4.7 extension API.

## Executed acceptance

All results below were observed on change `mkstwyuv`; source statements are labeled **inspected** separately.

| Command or exercise | Observed result |
|---|---|
| `just check` | Passed end to end |
| `cargo test --workspace --locked` | 113 tests passed: CLI 3, content 21, frozen-scenario 10, core 65, canonical JSON 5, fidelity 9 |
| `cargo fmt --all --check` | Passed |
| Strict Clippy, workspace and standalone Godot without tooling features | Both passed with `-D warnings` |
| `just deps-check` and `just godot-lint` | Passed |
| `just parity` | 20 vectors passed; zero differences, integrity failures, or decimal normalizations |
| Isolated framing mutation through the actual parity CLI | Exit 1; exactly one unattributed difference at `/result/audience_receptions/0/framing`; zero integrity failures and normalizations. Pinned vectors were untouched |
| `just catalog-test` | Native content tests and all 18 Python authoring tests passed |
| `python3 catalog/catalog.py validate` | 528 entities; zero structural gaps; zero warnings |
| `just oracle-test` | All 86 retained tests passed, including the ten MVP gates, from `tools/oracle/` |
| `just oracle-vectors` | Export completed; canonical-vector and parity checks passed |
| `just catalog-generate`, twice | Identical SHA-256 values for fidelity permissions, handler registry, and phase-flow output |
| M1 sealing and replay | No-op sealing preserves frozen bytes and the pinned scenario identity; all three package replays match the oracle |
| M2 sealing and replay | All four frozen-file checksums unchanged by resealing; all three native package replays passed |
| `just godot-test` | Passed actual extension loading, command/projection boundary, room navigation, rejection, frame-delay, routing, speaking, and save/resume exercises |
| Non-headless Godot smoke | Opened the actual client and portraits; exercised keyboard/button navigation, full authored speaking preview, inquiry, routing handoff, proposal, reading, and separate scorecard. Visual checks at 1280×800 passed; process exited 0 |

### Frozen identities

| Fixture | Scenario hash |
|---|---|
| M1: `scenarios/mvp_2006_cycle_m1` | `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066` |
| M2: `scenarios/mvp_2006_cycle` | `sha256:d9f745b948d7cf2d76e364ff08664a0fcf3347a41485e107a18b02297bca2ce4` |

M1 native replay results with the default NORMAL follow-up:

| Package | State hash | Transcript bytes |
|---|---|---:|
| WAIT_AND_WARN | `sha256:424bdafb55ed42ca9c06b3a41634926351d5d26d529552bcc061f6efb626337b` | 80868 |
| MEASURED_FIRMING | `sha256:58738b57d2e8bf3d4a53c11c90d0b4730aa67bef6c9338c0024f2670be6cdd63` | 62887 |
| FIRMING_BIAS | `sha256:af3ee7c770d23610ea4cb26fb61faab35ab7fc904d08d4bc9cf3c610c0d11cb2` | 78232 |

M2 native replay state hashes are `775d1bed…` for WAIT_AND_WARN, `d332d286…` for MEASURED_FIRMING, and `2ff24126…` for FIRMING_BIAS. These are native explicit-package runs, not a claim that an interactive session submits an unchosen default proposal.

Vector checksums are retained in `tests/oracle/vectors.sha256`. Parity reports are generated under `target/parity/`. `tests/parity/attributions.toml` names the adopted M2 changes and their acceptance tests; it does not waive any M1 divergence.

### Compiler negative fixtures

`tests/content/negative/` contains mutation specifications. Tests apply them to isolated scenario/catalog copies, exercising real validation or sealing. Rehashed malformed slices still fail structurally; failed sealing leaves every frozen output unchanged.

| Mutation | Observed category |
|---|---|
| Duplicate owned-state contract ID | `duplicate_contract` |
| Unresolved opening owner | `referential_integrity` |
| Unknown work-handler key | `unknown_handler` |
| Incompatible manifest schema | `manifest_closure` |
| Missing owned-state witness kind | `missing_witness` |
| Nonexistent period variant | `manifest_closure` |
| Mechanical fidelity selected for the named Chair | `fidelity_permission` |
| Unknown declared transition | `unknown_transition` |
| Handler scheduled in the wrong phase | `phase_feedback` |
| Missing named-cognition opening state | `initialization` |
| Selectable but incomplete catalog contract | `catalog_eligibility`; scenario loading fails closed with `catalog`, even for an unrelated frozen selection |

Additional executed regressions cover unit mismatch, missing provider/fallback, queue-sequence duplication, unreconciled residuals, ambiguous same-phase writers, and reverse-phase feedback.

## Work-package evidence

| Package | As-built result and acceptance |
|---|---|
| WP-01 | Four locked crates, toolchain pins, dependency/client boundaries, native Godot import; executed build, strict lint and headless client |
| WP-02 | Media inventory repair, repository-relative asset lookup, truthful readiness, current generated inventory; executed global validation and 18 authoring tests |
| WP-03 | All 39 M1 selections have eligible contracts and selectable declarations; executed compiler eligibility validation and unchanged M1 freeze/replay |
| WP-04 | Canonical JSON and SHA-256; 15 package/request vectors, four reseeds, scripted play; executed canonical-byte tests and all 20 comparisons |
| WP-05 | Native catalog, manifest, initialization, hash and sealing validation; executed frozen-file equality and fail-closed negative fixtures |
| WP-06 | Fixed-phase clock, registry, owned transitions, witness ledger, legal authority and release path; executed authority/ordering tests and full parity |
| WP-07 | Capacity, displacement, staff tasks, assessments and belief revision; executed request-mode parity and staff conformance gates |
| WP-08 | FOMC procedure, execution, Treasury clearing, accounting, settlement, repo, population and stage receipts; executed conservation, rejection and full-cycle parity |
| WP-09 | Claims, publication, audience, media, commitments, compression and review; executed inert-publication and replay gates, including reseeds |
| WP-10 | Native parity reports and bounded attributions; executed 20 clean comparisons and deliberate framing-drift rejection |
| WP-11 | Static handler binding, phase/read/write validation and metadata generation; executed unknown-handler, unknown-transition, ambiguous-write and feedback rejections; repeated generation identical |
| WP-12 | Seven sealed fidelity models and generated permission matrix; executed matrix compatibility, model-required-state and illegal-selection tests |
| WP-13 | Exact persisted runtime and queue; executed resume equivalence at every event boundary for every package/request combination, idempotent re-save and corrupt-save rejection |
| WP-14 | Typed session projections and native interactive play; executed scripted-play parity and rejected-command invariants |
| WP-15 | Seven playable Godot rooms over the native session boundary; executed headless interaction, rejection and projection checks plus non-headless visual smoke |
| WP-16 | Production recipes use Rust; Python engine/tests reside under `tools/oracle/`; executed full native gate and 86 tests from the retained location |
| WP-17 | Typed commands, accepted/rejected receipts, witnessed command journal and idempotency; executed rejection, redelivery and resume invariants |
| WP-18 | Named chief with dated office holding and private cognition; executed holder-replacement interface proof preserving office records without transferring private cognition; M1 parity remains exact |
| WP-19 | Calendar anchors, discretionary periods, dated reservations and interruption queue; executed explicit-advance, capacity/resume, frame-delay and inspect/park/restore invariants |
| WP-20 | Folder pencils, pure practical-card previews, atomic slate admission, speaking commits and prospective corrections; executed aggregate-overbooking rollback, immutable history and changed prospective vote |
| WP-21 | Evidence-bounded access choices and dated deadline options; executed scope-safe delayed delivery, inaccessible-evidence choices, exact capacity and no preview side effects |
| WP-22 | Separate extradiegetic scorecard from witnessed findings; executed signed awards/deductions, idempotent review acceptance and unchanged in-world no-verdict review |
| WP-23 | Calendar, folder, full authored practical cards, interruptions and scorecard in Godot; executed boundary suite and actual visual interaction |

## Current boundaries and engineering dispositions

**Inspected:** `reservist-core` owns causal state, capacity, history, persistence and scoring. `reservist-content` validates global authored/normalized agreement before loading a slice. Godot sends typed commands or option IDs and renders immutable projections. Diagnostic snapshots and whole-run export belong to the CLI tooling feature, not the gameplay build.

- M2 starts paused. Reading and previewing do not advance the calendar or consume capacity. An unsubmitted proposal never becomes the default institutional decision.
- Ordinary choices remain pencils until handoff. A marked inquiry commits on speaking, independently of other pencils; closing the draft cannot revoke its work or disclosure. Preview shows the complete authored line, exact immediate mechanics, and a separate assessment.
- Admission validates the combined slate and reserves/enqueues it atomically. It does not guarantee later authorization, execution, take-up or settlement.
- New evidence does not invalidate unrelated reviewed choices or overwrite an open folder's context. Each chosen binding must remain authorized and feasible. Proposal amendments are prospective, linked records; original folders and witnesses remain immutable.
- Saving is supported only at quiescent boundaries. An open folder or a parked folder with pencils is refused rather than silently discarded. Supported saves retain command idempotency, interruption state, calendar reservations, admitted work and scorecard state.
- The in-world review retains `verdict: None` and “No universal verdict is assigned.” The scorecard is separate, signed, evidence-grounded and non-causal. Population lenses still contain no `economy_score`.
- Chief-holder replacement and compensating ledger entries have tested interface contracts. They do not expose M3 succession, supplemental-review gameplay or the D1 campaign terminal event as M2 commands.
- M4 provider execution, full world fidelity coverage and broader content remain the later milestone defined by the plan. No economic calibration or player-comprehension claim follows from these implementation tests.

H and the amendment now identify the accepted speaking and interruption dispositions without stale pending-decision labels. The baseline in the mandate remains historical; its additional M1/M2 fixture rows identify the executable inputs. The earlier corpus migration record belongs to repository history, not to current runtime evidence.
