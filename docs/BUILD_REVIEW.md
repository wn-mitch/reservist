# Reservist: Build Review (front door)

**Status:** Baseline as-built inventory at revision `97fc926a`, recorded 2026-09-05 by the Fable design review. No Rust/Godot artifact exists; this page describes the Python prototype the rewrite must replace. Each row states its basis: **inspected** (source read) or **executed** (command run this session).

## Result and runnable path

Executed: `just test` (86 tests OK), `just gates` (10 OK), `just validate`, `just replay` (three packages, byte-identical transcripts across two runs), `just freeze` (no-op reseal), `just catalog-test` (17 run, 2 fail, pre-existing). `just play` opens an interactive REPL and exits immediately when stdin is not a TTY (inspected, `engine/cli.py:117`).

## Shape: intended versus as-built

| Concern | Intended (H) | As-built (Python) | Basis |
|---|---|---|---|
| Time | Player-authorized calendar board; discretionary periods; no wall clock | Discrete-event heap ordered by (time, integer phase priority, sequence, id); no wall clock anywhere; gaps compressed by `IntermeetingCompressor` with a witness; no board or bookable slots | inspected `engine/clock.py`, `engine/compression.py` |
| Phases | Fixed forward named phases with declared reads/writes | Integer priorities 10–90 used as tie-breakers; no declarations | inspected `engine/scenario.py` |
| Capacity | Dated commitments with owner, allocation, release | Present: `CapacityBook.reserve/release`, `Deliverable.displace` (SCHEDULED/DISPLACED/MISSED/DELIVERED); `CommitmentBook` with `reserved_units`, release on expiry/breach | inspected `engine/staff/capacity.py`, `engine/commitments.py`; executed tests |
| Persistence | Exact checkpoint/resume | None. Replay is re-run from `initialization.json`; state hash and transcript bytes compared | inspected `engine/cli.py:74`; executed gate 07 |
| Determinism | Deterministic ordering and identity | RFC 8785 canonical JSON + SHA-256 (`engine/canon.py`); randomness only via keyed SHA-256 draws; four content hashes compose into the scenario hash | inspected; executed replay |
| Client boundary | Typed commands in, receipts and projections out; no canonical references to the client | REPL verbs are direct method calls on `ScenarioRuntime`; `propose` mutates `runtime.package_id` (`engine/cli.py:182`). Player inbox is `PlayerRecordStore` with recipient and scope checks. Boundary tests are import-graph and substring checks | inspected `engine/player/records.py`, `tests/test_access_boundary.py`, `tests/test_presentation_boundary.py` |
| Authority stages | Proposal, authorization, execution, settlement, observation separately witnessed | Present: receipts carry PROPOSAL/AUTHORIZATION/EXECUTION/SETTLEMENT/OBSERVED EFFECT with owner chain; chair-only market command rejected with unchanged state hash | executed gates 05, 08; `tests/test_receipts.py` |
| Markets and settlement | Typed economic failure, no gameplay fallback | `FAILED_TO_CONVERGE` with `price=None`; two-phase settlement with `FAILED_PREPARE`/`FAILED_COMMIT` and reservation unwind; `fallback_binding.policy` must be `FAIL` | inspected `engine/markets/treasury_secondary.py`, `engine/settlement/envelope.py`, `engine/manifest.py` |
| Providers | Channel traits, RECORDED/RESPONSIVE | One stub adapter returning `ADAPTER_NO_PRICE_FORMATION`; `provider_binding` an unenforced string; gate 10 forbids network imports | inspected `engine/adapters/treasury_demand.py` |
| Staff and cognition | Chief of staff as persistent Person at NAMED_COGNITION | Three staff offices with capacity units, no persons; cast is the Chair plus two governors; chief-of-staff office selected at LIMITED_ROLE_HOLDER; no chief anywhere in code | inspected `scenarios/mvp_2006_cycle/staff`, `cast`, `manifest.json` |
| Review and score | In-world review plus extradiegetic signed score ledger | `StaffReview` with nine epistemic labels, forbidden hidden-state terms, `verdict: None`; gate 09 asserts "No universal verdict is assigned."; no score anywhere | inspected `engine/postmortem.py`; executed gate 09 |
| Routing and access | Evidence-bounded, no canonical significance | Seeded-hash delivery edges with delay, framing, scope, attention probability; single player scope `profile.chair_scoped`; projection excludes hidden parameters | executed gate 02; inspected `engine/delivery.py` |
| Catalog | In-repo, validated, frozen slice | Now in `catalog/`; runtime reads only `catalog_slice.json` (39 entries); `freeze` reads 9 CSVs + schema; global catalog has 214 issues; `completeness.txt` falsely reports the manifest as unauthored | executed; inspected `engine/catalog_slice.py`, `catalog/catalog.py:837` |
| Rust / Godot | Pure Rust core, thin Godot bridge | None present | inspected |

## Behavioral evidence recorded

Three package transcripts from `just replay`: `WAIT_AND_WARN` state `sha256:424bda…` 80,868 bytes; `MEASURED_FIRMING` `sha256:58738b…` 62,887 bytes; `FIRMING_BIAS` `sha256:af3ee7…` 78,232 bytes. Scenario hash `sha256:493ca024…8d066` unchanged by freeze.

## Deviations and uncertainty

- Global catalog validity is false (214 issues). Frozen slice validity is true.
- `test_presentation_boundary.py` proves only that three tokens are absent from three packages.
- No evidence exists for economic calibration or player comprehension.
- Ten gates and 86 tests establish their own assertions only.

## Pending owner decisions

D1–D6, in the review Part G.

## Migration record (this revision)

198 artifacts copied from the HumanLayer artifact directory into `docs/design/` and `catalog/`; every SHA-256 matches `reservist-omni-split/00-index.md`. Paths repaired in `engine/catalog_slice.py`, `tests/test_phase6_catalog_slice.py`, `justfile`. Removed: `humanlayer-omni.md`, four `.DS_Store`. `.gitignore` now excludes `.DS_Store`. `reservist-omni-split/` retained pending D6.
