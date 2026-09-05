# Reservist: Rust/Godot Implementation Plan (proposed)

**Document ID:** `RIP-2026-09-05-01`
**Status:** Proposed implementation plan for the authorized Build Mandate. Planning output only. No Rust workspace, catalog row, or runtime behavior was changed while producing it.
**Prepared:** 2026-09-05 by Claude Fable 5.1 against working copy `mkstwyuv` (parent `c8c8ac15`, "docs: Add Fable design review and front-door skeletons"; pinned baseline `97fc926a`).
**Authority:** `docs/BUILD_MANDATE.md` (authorized), `docs/CURRENT_DESIGN_BIBLE.md` precedence, H (`docs/design/15-open-questions-decision-handoff.md`), review `RDR-2026-09-05-01` Part G (D1 to D6 settled).
**Evidence labels:** every repository claim below is marked **inspected** (source read this session) or **executed** (command run this session). External version facts are marked **external**.

---

## 1. Executive decision

**Implementation can begin now.** No owner decision blocks M1 or M2. D1 to D6 are settled and are not reopened here. Every remaining choice in this plan is an ordinary engineering or content-authoring choice inside the Build Mandate's stated builder latitude (module layout, names, phase roster, serialization, save format, test organization, registry packaging, toolchain pins).

Two content-identity choices are made under builder latitude but are surfaced so the owner can override them without a redesign. Neither blocks work; neither is an owner-question.

1. **Consumer of the two "mass public belief" media transmissions.** `catalog/inventory/media_information/transmissions.csv` rows `tx.media.aftv.mass_public_belief` and `tx.media.gnbc.mass_public_belief` name `consuming_entry_id=UNKNOWN` and `transformation_owner_id=UNKNOWN` because "the belief owner is outside this domain inventory" (inspected). The only mass-belief owner in the global catalog is `population.us.person.cells` (a `PersonPopulationCell`; `PopLens` entries are non-owning views under D05). The plan binds both rows to that owner with provenance. The owner may substitute a different owner later; the compiler will accept any resolvable instance.
2. **Canonical identity of the 2006 chief of staff (M2).** H requires a persistent `Person` at `NAMED_COGNITION`. The plan gives the builder a fixed procedure (verify the historical officeholder from Federal Reserve primary sources; otherwise author a scenario person with `provenance=authored` and a satirical display name in the presentation register). D05 forbids inventing causal history, not naming a staff member.

Everything else the reviewer's Part G listed as blockers (toolchain absent, 214 catalog issues, 2 failing catalog tests, false readiness line) is scheduled as work packages WP-01 through WP-03 below.

---

## 2. Repository baseline

### 2.1 Implementation inspected

| Item | Fact | Basis |
|---|---|---|
| Python package | `engine/` 72 files (56 modules plus package inits), 8,061 lines; `engine/scenario.py` is 1,817 lines and owns the whole runtime (`ScenarioRuntime`); tests and catalog tooling add 3,218 lines | executed `wc -l` |
| Interpreter | Python 3.14.3, zero third-party imports | executed `python3 --version`; inspected imports |
| Tests | `just test` runs 86 tests OK in 2.5 s: 76 unit tests across 30 files plus 10 acceptance gates in `tests/acceptance/test_mvp_gates.py` | executed |
| Gates | `just gates` = the 10 `test_gate_*` methods | inspected justfile |
| Validate | `just validate` prints `validation passed: sha256:493ca024…8d066` | executed |
| Run | `just run --package MEASURED_FIRMING --report-endogeneity`: state `sha256:58738b…`, 74 events, receipts PROPOSAL/AUTHORIZATION(REJECTED)/EXECUTION(AUTHORIZED_NOT_EXECUTED)/OBSERVED EFFECT(PUBLICATION_RESPONSE_RATIONED), price 0.9857 | executed |
| Replay | `just replay`: WAIT_AND_WARN `424bda…` 80,868 bytes (97 events); MEASURED_FIRMING `58738b…` 62,887 bytes; FIRMING_BIAS `af3ee7…` 78,232 bytes; byte-identical across two runs | executed |
| Catalog | `python3 catalog/catalog.py validate`: 214 structural issues; `just catalog-test`: 17 run, 2 failures (`test_import_and_generation_are_byte_deterministic`, `test_vocabulary_boolean_and_semantic_null_validation`), both failing only because `validate`/`generate` exit 1 on the same 214 issues | executed |
| Scenario | `scenarios/mvp_2006_cycle/`: `manifest.json` (39 selected entries, 9 delivery edges), `initialization.json` (seed 20060328, 44 opening-state rows, 7 reconciliations, 5 scheduled events), `tape/releases.json` (2 releases), `cast/fomc_2006.json` (chair + 2 governors), `staff/work_2006.json` (3 units, 1 standing deliverable, 1 staff evidence delivery), 4 legal instruments, `catalog_slice.json` (39 entries) | inspected |
| Version control | jj; working copy has uncommitted front-door edits and the D6 deletion of `reservist-omni-split/`; `docs/FABLE_IMPLEMENTATION_PLAN_PROMPT.md` untracked | executed `jj status` |

### 2.2 Toolchain state

| Tool | Repository state | Machine state | Basis |
|---|---|---|---|
| Rust | no `Cargo.toml`, no `rust-toolchain.toml` | rustup default 1.91.1; installed 1.85.0, 1.93.0, 1.96.0, 1.96.1, stable, nightly | executed `rustup show` |
| Godot | no project, no reference | `/opt/homebrew/bin/godot` = `4.7.2.stable.official.ed1daf0bf`; `/Applications/Godot.app` present | executed `godot --version` |
| godot-rust | none | not installed | inspected |
| Python pin | none (`.python-version`, `pyproject.toml` absent) | 3.14.3 | inspected |
| just | `justfile` with test, gates, catalog-test, catalog-generate, validate, freeze, run, replay, play | installed | inspected |

### 2.3 Oracle pin

Revision `97fc926a`, Python 3.14.3, scenario hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066`, the three `just replay` transcripts and state hashes above, the 86 tests and 10 gates. The `run` subcommand defaults `--request` to `NORMAL` (`engine/cli.py:233`), so the default MEASURED_FIRMING run includes the Markets follow-up and a REJECTED vote; `play` runs with `request_mode=None`. Both variants are oracle vectors.

### 2.4 Production entry points (Python, to be replaced)

`engine/cli.py` subcommands: `validate`, `freeze`, `run` (`--package`, `--request`, `--transcript`, `--report-endogeneity`), `replay-check`, `play` (`--package`). The REPL accepts twelve verbs: `inspect <n>`, `ask markets [accelerated]`, `advance`, `book`, `verbs`, `fomc`, `propose <package>`, `operations`, `statement`, `wire`, `review`, `quit` (inspected `engine/cli.py:117-200`). `play` exits immediately when stdin is not a TTY.

### 2.5 Known catalog facts that the plan must not misreport

- The 214 issues come from exactly 27 rows in one inventory domain: 19 rows of `catalog/inventory/media_information/relationships.csv` and 8 rows of `catalog/inventory/media_information/transmissions.csv` (executed grouping; see Section 6). They are pre-existing.
- All 39 entries selected by the MVP manifest carry `selectable_in_manifest=false` and are reported `catalog_eligibility=false` with blocker `not_catalog_eligible` in `catalog/generated/catalog_eligibility.csv` (executed). `engine/catalog_slice.py` never reads `selectable_in_manifest`; it freezes any entry whose `completeness_state` is `probe_complete` (inspected). This is the "selected entries that remained ineligible" discrepancy in amendment §7.6.
- `catalog/generated/` is stale: `catalog_eligibility.csv` has 492 rows while `entities.csv` has 493 instances (`pop.us.public.low_attention_residual` is missing) (executed).
- `catalog/catalog.py:14` hard-codes `REPO_ROOT = Path.home() / "reservist"` for the `presentation_refs.csv` asset check (inspected). `catalog/catalog.py:837` emits the false "manifest remains unauthored" line (inspected).
- `catalog/relationships.csv` uses `HOLDS` (upper case) for the profile-required-office check and `holds` (lower case) in the media rows (inspected).

---

## 3. Target architecture

### 3.1 Workspace

```
Cargo.toml                 workspace: crates/reservist-core, crates/reservist-content,
                           crates/reservist-cli, crates/reservist-godot
rust-toolchain.toml        channel pin (Section 5)
clippy.toml                disallowed types/methods (wall clock, network, randomness)
crates/reservist-core/     library: causal core
crates/reservist-content/  library + bin `reservist-content`: catalog + scenario compiler
crates/reservist-cli/      bin `reservist`: validate, freeze, run, replay-check, play, save, resume, parity
crates/reservist-godot/    cdylib: GDExtension bridge (godot-rust)
godot/                     Godot 4.7 project (scenes, themes, minimal GDScript)
catalog/                   authoring CSVs, schema.json, catalog.py (authoring validator, retained)
scenarios/                 scenario sources and frozen outputs
tools/oracle/              Python engine and its tests after WP-16 (test tooling only)
tests/oracle/vectors/      exported Python oracle vectors (canonical JSON)
tests/parity/              attribution file and cross-runtime comparison fixtures
```

`reservist-cli` is added to the three mandated crates. Reason (repository evidence): `validate`, `freeze`, `run`, `replay-check`, and `play` are in the replacement scope (P-06) and must run headless in tests and in `just`; `reservist-core` must not own stdin, stdout, or argument parsing, and `reservist-godot` cannot be exercised without a Godot binary. The CLI is production tooling for `validate`/`freeze` and the parity driver for everything else. It contains no mechanics.

### 3.2 `reservist-core` module map and Python origin

| Module | Owns | Replaces |
|---|---|---|
| `canon` | RFC 8785 canonical JSON, SHA-256, `load_json` rejecting non-finite numbers | `engine/canon.py` |
| `time` | `Instant` parsing of offset ISO-8601 strings, formatting identical to Python `isoformat()` (`YYYY-MM-DDTHH:MM:SS±HH:MM`), `+ minutes` | `engine/clock.py:parse_time`, `datetime` uses |
| `clock` | `ScheduledEvent` (due_time, phase, stable_sequence, stable_id, owner, handler key, payload, causal_parent), binary heap ordered `(instant, phase, sequence, id)`, `schedule` rejecting past/duplicate id/duplicate sequence, `advance_to`, `advance_next` | `engine/clock.py` |
| `phase` | `Phase` enum (Section 3.5), `WorkHandler` trait, `Registry` (static table), declared reads/writes, ambiguity check | none (new-runtime guarantee; replaces `ScenarioRuntime._handle` dispatch) |
| `witness` | `DomainEvent`, `WitnessLedger` (`event.%06d` ids, `next_event_id`, transcript bytes) | `engine/witness.py` |
| `state` | `CanonicalRegistry`, `StateOwner` trait with the five concrete owners (institutional, macro adapter, published reference, market, outlet), `TypedTransition`, `StateMutationError` | `engine/state/*.py` |
| `legal` | `LegalClause`, `LegalRegistry` | `engine/legal.py` |
| `authority` | `Command`, `AuthorizationDecision`, `Directive`, `ActionResult`, statuses, `AuthorityResolver` | `engine/authority.py` |
| `records` | `ReceiptStage`, `StageReceipt`, `Record` | `engine/records.py` |
| `packages` | the three `PolicyPackage` values and `without_language` | `engine/packages.py` |
| `bodies::fomc` | `FomcBody::conduct`, `Vote`, `FomcDecision` | `engine/bodies/fomc.py` |
| `cognition` | `BoundedEstimate`, `SourceLedgerEntry`, `BeliefLedger`, `LimitedParticipant::position_for`, `revise_from_assessment` | `engine/cognition/*.py` |
| `execution::desk` | `DeskExecutor::execute`, `chair_only_command` | `engine/execution/desk.py` |
| `accounting` | `Account`, `LedgerEntry`, `AccountingTransaction`, `AccountingLedger` (reserve/release/commit/conservation/balance sheet) using `rust_decimal` | `engine/accounting/ledger.py` |
| `markets::treasury_secondary` | `TreasuryOrder`, `TreasuryFill`, `ClearingResult`, `TreasurySecondaryMarket::clear` (bounded iterations, `FAILED_TO_CONVERGE`) | `engine/markets/treasury_secondary.py` |
| `settlement` | `SettlementEnvelope` prepare/commit with reservation unwind and witnesses | `engine/settlement/envelope.py` |
| `agreements::repo` | `BilateralRepoAgreement` | `engine/agreements/repo.py` |
| `participants` | `DealerCohort`, `LeveragedFundCohort`, `ExternalBuyerResidual` | `engine/participants/*.py` |
| `adapters::treasury_demand` | `TreasuryDemandAdapter` boundary shape (test-only in Python; retained for the boundary-shape contract) | `engine/adapters/treasury_demand.py` |
| `claims` | closed grammar enums, `Claim`, `ClaimRegistry` | `engine/claims.py` |
| `communication` | `CommunicationAct::from_decision`, `authorized_claim_ids` | `engine/communication.py` |
| `delivery` | `AudienceEdge`, `AudienceReception`, `AudienceBeliefStore`, `DirectAudienceRouter` (keyed SHA-256 draws, framing interpretation) | `engine/delivery.py` |
| `observation` | `Observation`, `EvidenceDelivery`, `ObservationSystem` | `engine/observation.py` |
| `media::loonberg` | `Report`, `LoonbergOutlet::publish` | `engine/media/loonberg.py` |
| `compression` | `CompressionStep`, `IntermeetingRealization`, `IntermeetingCompressor` | `engine/compression.py` |
| `commitments` | `Commitment`, `CommitmentBook` | `engine/commitments.py` |
| `monitoring` | `MonitoringObligation`, `MonitoringBook` | `engine/monitoring.py` |
| `staff` | `CapacityBook`, `Deliverable`, `AnalyticalTask`, `RequestMode`, `Assessment`, `AssessmentBuilder`, `StaffUnit`, `UnitScopedEvidenceStore`, `StaffDirectory`, `UncertaintyKind/Note` | `engine/staff/*.py`, `engine/uncertainty.py` |
| `population` | `PersonPopulation`, `HouseholdCohorts`, `PopLensProjector`, `PopulationView` | `engine/population/*.py` |
| `postmortem` | `EpistemicLabel`, `PostmortemLink`, `NextMorningBook`, `StaffReview` (verdict `None` preserved), `PostmortemBuilder` | `engine/postmortem.py` |
| `player` | `PlayerRecordStore` (recipient and scope checks, read witness) | `engine/player/records.py` |
| `scenario` | `FrozenScenario` (loaded frozen inputs), `Runtime` (owns every subsystem above, handler implementations, `run_all`, `advance_next`, `advance_to_next_consequential_event`, `state_hash`, `material_state_hash`, `result`), `RunResult`, endogeneity report | `engine/scenario.py` |
| `save` | `SaveFile` v1 (Section 7): serialize/deserialize the complete `Runtime`, versioned and hashed | none (new-runtime guarantee, G-19) |
| `api` | the only `pub` surface for clients: `Session`, `SessionOp` (M1) / `Command` (M2), `Receipt`, projection structs (`MorningBookView`, `RecordView`, `FomcRoomView`, `OperationsView`, `StatementView`, `WireView`, `ReviewView`, `RoutingAccount`), `Rejected` | `engine/harness/*.py` data extraction (rendering itself moves to clients) |
| `fidelity` | sealed `FidelityModel` trait, one impl per tier, clade compatibility table, `permission_matrix()` export | none (new-runtime guarantee, G-03) |

Visibility rule: everything except `api`, `canon`, `phase::metadata`, `fidelity::permission_matrix`, and `save` is `pub(crate)`. A `tooling` cargo feature exposes `scenario::Runtime` internals to `reservist-cli` and tests. `reservist-godot` compiles without `tooling` and therefore cannot reference canonical state types at all.

### 3.3 `reservist-content`

| Module | Owns | Replaces |
|---|---|---|
| `schema` | typed reading of `catalog/schema.json` (tables, keys, vocabularies, lifecycle, allow_unknown) | `catalog/catalog.py` SCHEMA use |
| `inventory` | import of `catalog/inventory/*/` into normalized tables with header, key, conflict checks | `catalog.py:inspect_inventory` |
| `validate` | the full structural rule set of `catalog.py:collect_errors` (schema, key, vocabulary, type, endpoint, unit, state_transition, relationship, fallback_cycle, catalog_eligibility, profile, external_channel, market_interface, composition_probe, instrument, product, residual) with the same category names | `catalog.py:collect_errors` |
| `eligibility` | H's three concepts as distinct functions: `catalog_eligible(entry)`, `scenario_composition(manifest)`, and a hook point for runtime action eligibility that lives in core | `catalog.py:eligibility_details`, `engine/manifest.py` |
| `slice` | `freeze_catalog_slice` producing byte-identical `catalog_slice.json` | `engine/catalog_slice.py` |
| `manifest`, `initialization`, `tape`, `authority_content` | loading and closure checks with the same `ManifestValidationError` categories (`manifest_closure`, `hash_mismatch`, `missing_provider`, `missing_fallback`, `referential_integrity`, `unit`, `currency`, `initialization`, `queue_sequence`, `unreconciled_residual`) | `engine/manifest.py`, `engine/initialization.py`, `engine/scenario.py:validate_scenario/seal_scenario` |
| `bindings` | new fail-closed checks: every `work_kind` in `scheduled_events` and tape resolves to a registered handler key; every owner's `accepted_transition_kinds` resolves to a registered transition; every selected entry is catalog-eligible; fidelity selection is permitted at all three levels | none (G-01, G-03) |
| `frozen` | `FrozenScenario` document written as canonical JSON with content hashes; records handler keys and registry version, never pointers | `manifest.replay_hash` and friends |
| `generate` | regenerates `catalog/generated/` including `fidelity_permissions.csv`, `handler_registry.csv`, `phase_flow.mmd` | `catalog.py:generate` plus H "Generate Mermaid flows" |

Dependencies: `reservist-content` depends on `reservist-core` for `canon`, `phase::metadata`, and `fidelity::permission_matrix` only. `reservist-core` never depends on `reservist-content`. `catalog/catalog.py` is retained as the authoring-side validator (it is the authoritative source of the rule set until WP-05 proves equivalence, and stays as tooling afterward under `just catalog-test`).

### 3.4 `reservist-godot` and the Godot project

- One `GodotClass` node type, `ReservistSession`, wrapping `reservist_core::api::Session`. Methods exposed to GDScript: `load_scenario(path)`, `submit(op: Dictionary) -> Dictionary` (a serialized `SessionOp` in M1, `Command` in M2; returns receipt or rejection), `view(name: String) -> Dictionary` (serialized projection), `save(path)`, `resume(path)`. Dictionaries are produced by serializing `api` structs; GDScript never receives a handle to anything else.
- Godot owns: scenes for the seven rooms (Morning Book, FOMC Room, Operations Room, Statement Editor, World Wire, Review, Request), themes, input, portrait assets under `assets/headshots/`. GDScript is limited to layout, navigation, and calling the two methods above.
- Forbidden in `godot/`: `FileAccess`, `ConfigFile`, `ResourceSaver` for game state; `HTTPRequest`, `HTTPClient`, `TCPServer`, `StreamPeerTCP`, `PacketPeerUDP`, `WebSocketPeer`, `ENetConnection`; `randf`/`randi`/`RandomNumberGenerator` in any script that touches session data. Enforced by a text lint (`just godot-lint`) over `*.gd` and `*.tscn`.
- Frame timing and animation never call `submit`. Only explicit player input does. Enforced behaviorally (G-07, WP-15).

### 3.5 Phase roster (builder decision under P-03)

Derived from the integer priorities in `engine/scenario.py` and the fixture (inspected). Ordering key inside the heap becomes `(instant, Phase as u8, stable_sequence, stable_id)` and the `u8` values are the Python priorities, so M1 event order is identical by construction.

| Phase | Value | Handler keys (Python `work_kind`) | Declared writes |
|---|---|---|---|
| `OPEN` | 10 | `morning_book.open` (inert, emits `scheduled_event_handled`) | ledger only |
| `RELEASE` | 20 | `macro.publish_release`, `macro.publish_intermeeting_release` | macro adapter, published reference, observations, player records, compressor realizations; may enqueue `request_follow_up` when `--request` is set |
| `STAFF` | 25 | `staff.complete_analytical_task` | staff capacity, tasks, assessments, participant beliefs, player records |
| `CHECKPOINT` | 30 | `phase_1.checkpoint` (inert) | ledger only |
| `AGREEMENT` | 35 | `repo.process_non_roll` | repo status, leveraged fund deficit |
| `AUTHORITY` | 40 | `fomc.meeting` | policy-package record, FOMC procedure, desk authority, market clearing, accounting, repo settlement, receipts, observations, commitments, then schedules `communication.publish_fomc_statement` |
| `PUBLICATION` | 50 | `communication.publish_fomc_statement` | communication acts, commitments, monitoring, scheduled receptions |
| `MEDIA` | 55 | `media.publish_loonberg_report` | outlet state, reports, scheduled receptions |
| `RECEPTION` | 60 | `audience.receive_artifact` | audience beliefs, dealer/fund public estimates, player records; owns the bounded publication clearing loop |
| `MONITORING` | 70 | `monitoring.review` | monitoring obligations |
| `COMMITMENT` | 80 | `commitment.expire` | commitments, monitoring closure, staff capacity release |
| `REVIEW` | 90 | `morning_book.next_cycle` | next morning book, staff review, player records |

Two declared bounded loops, named per P-03: (a) the AUTHORITY handler's execution, clearing, settlement, receipt, and observation sub-stages, owner `body.us.federal_reserve.fomc` then `market.us.treasury.secondary`, stopping rule `max_iterations=64` with typed `FAILED_TO_CONVERGE`; (b) the RECEPTION handler's publication clearing, owner `market.us.treasury.secondary`, triggered once when both `cohort.us.dealer.primary` and `inst.us.leveraged_funds` have witnessed `audience_order_intended` for one artifact, same stopping rule. Each sub-stage is a stage witness (receipt), not a scheduler phase. This preserves the oracle event order exactly and satisfies H's "mechanism explicitly owns a bounded clearing loop." Separate EXECUTION/CLEARING/SETTLEMENT scheduler phases are not introduced in M1 or M2; the reviewer's roster was "expected shape, not a mandate."

Compile-time rule (G-06): the registry test asserts that no two handlers in the same phase declare a write to the same state id, and that every dynamic schedule call targets a phase value strictly greater than the scheduling handler's phase when the due time is equal to the current time (cross-phase feedback rule). The existing dynamic schedules (`statement` at 50 from AUTHORITY 40, `report` at 55 from RECEPTION 60 but +1 minute later, receptions at 60 from 50 or 55, monitoring at 70, expiry at 80) all satisfy this; the test guards regressions.

### 3.6 Dependency diagram

```
catalog/*.csv + schema.json ─┐
scenarios/<s>/*.json ────────┤
                             ▼
                   reservist-content ──reads registry metadata + fidelity matrix──▶ reservist-core::{phase::metadata, fidelity, canon}
                             │
                             ▼ writes canonical frozen data (catalog_slice.json, manifest.json, initialization.json, tape, scenario.frozen.json)
                             │
        reservist-cli ───────┼──────────────▶ reservist-core (feature "tooling")
                             │                      ▲ api only
        reservist-godot ─────┘──────────────────────┘
              ▲
        godot/ scenes + minimal GDScript (Dictionaries in, Dictionaries out)

tools/oracle (Python) ──exports──▶ tests/oracle/vectors ◀──compares── reservist-cli parity
```

Causal-state ownership: only `reservist-core::scenario::Runtime` holds mutable canonical state. `reservist-content` holds authored and frozen data, never runtime state. `reservist-cli` and `reservist-godot` hold a `Session` handle and projections. `tools/oracle` holds nothing at runtime.

---

## 4. Contract classification matrix

Classification is exactly one of: **P** preserved parity behavior, **A** authorized adopted difference, **N** new-runtime guarantee, **M3**, **M4**, **R** research/calibration obligation.

| Subsystem or item | Class | Python source | Rust destination | Evidence | Milestone |
|---|---|---|---|---|---|
| Canonical JSON + SHA-256 | P | `engine/canon.py`; `tests/test_canon.py` (4) | `core::canon` | test_canon vectors; byte equality of Rust canonicalization against every line of the three Python transcripts | M1 WP-04 |
| Scenario hash composition (catalog, manifest, initialization, tape) | P | `engine/scenario.py:scenario_hash`, `seal_scenario`, `validate_scenario` | `content::frozen`, `content::slice` | `reservist validate` prints the pinned hash; `reservist freeze` writes byte-identical files (`jj diff` empty) | M1 WP-05 |
| Manifest and initialization closure categories | P | `engine/manifest.py`, `engine/initialization.py`; `test_manifest_closure` (4), `test_initialization` (4), gate 01 | `content::manifest`, `content::initialization` | ported negative tests with identical category strings | M1 WP-05 |
| Frozen slice loading fails closed | P + N | `engine/catalog_slice.py:load_catalog_slice` | `content::slice`, `content::bindings` | ported hash/duplicate tests; new handler-availability and eligibility rejections | M1 WP-05, WP-11 |
| Event ordering key and clock rules | P | `engine/clock.py`; `test_event_order` (2) | `core::clock`, `core::phase` | ported tests; transcript order equality | M1 WP-06 |
| Fixed forward phases, static registration, declared reads/writes | N | none (integer priorities only) | `core::phase`, `content::generate` | registry ambiguity test; generated `handler_registry.csv`, `phase_flow.mmd` | M1 WP-06, WP-11 |
| Witness ledger, event ids, transcript bytes | P | `engine/witness.py` | `core::witness` | transcript semantic equality (Section 7.2) | M1 WP-06 |
| Canonical registry, typed transitions, owner rejection | P | `engine/state/*.py` | `core::state` | `test_authority` state-hash-unchanged assertions; G-04 negative tests | M1 WP-06 |
| Legal clauses, authority resolution, chair-only rejection | P | `engine/legal.py`, `engine/authority.py`; `test_authority` (4), gate 08 | `core::legal`, `core::authority` | ported tests | M1 WP-06 |
| Release publication, observation, evidence delivery, player records, read witness | P | `engine/state/macro_adapter.py`, `engine/observation.py`, `engine/player/records.py`; `test_access_boundary` behavioral test, gate 02 behavioral part | `core::observation`, `core::player` | ported behavioral assertions (record contains "Core consumer prices" and not `inflation_persistence`) | M1 WP-06 |
| Staff capacity, displacement, tasks, assessments, belief revision | P | `engine/staff/*`, `engine/cognition/revision.py`; `test_request_lifecycle` (4), `test_displaced_work` (1), `test_assessment_provenance` (3), `test_phase4_replay`, `test_phase4_vote`, gate 03 | `core::staff`, `core::cognition` | ported tests | M1 WP-07 |
| FOMC conduct, narrowing, votes, directive | P | `engine/bodies/fomc.py`, `engine/cognition/participant.py`; `test_policy_cycle` (5) | `core::bodies::fomc`, `core::cognition` | ported tests | M1 WP-08 |
| Desk execution preflight | P | `engine/execution/desk.py`; `test_authority` | `core::execution::desk` | ported tests | M1 WP-08 |
| Treasury clearing (bounded, rationed, failed) | P | `engine/markets/treasury_secondary.py`; `test_market_clearing` (3), `test_provider_parity` (1), gate 04, gate 08 | `core::markets` | ported tests; exact Decimal equality | M1 WP-08 |
| Accounting conservation, two-phase settlement, reservation unwind | P | `engine/accounting/ledger.py`, `engine/settlement/envelope.py`; `test_accounting` (3) | `core::accounting`, `core::settlement` | ported tests; conservation witness in parity | M1 WP-08 |
| Repo non-roll and settlement | P | `engine/agreements/repo.py`; `test_repo_lifecycle` (1) | `core::agreements::repo` | ported test | M1 WP-08 |
| Participant order logic | P | `engine/participants/*` | `core::participants` | transcript equality on `treasury_order_submitted` payloads | M1 WP-08 |
| Stage receipts and ordering | P | `engine/records.py`, `scenario._record_receipt`; `test_receipts` (4), gate 05 | `core::records` | ported tests | M1 WP-08 |
| Population conservation and non-owning lenses | P | `engine/population/*`; `test_population_conservation` (3) | `core::population` | ported tests, including the `economy_score` absence assertion (stays as written under D5) | M1 WP-08 |
| Claims grammar, communication authorization | P | `engine/claims.py`, `engine/communication.py`; `test_claim_grammar` (4) | `core::claims`, `core::communication` | ported tests | M1 WP-09 |
| Audience routing, staged reception, interpretation | P | `engine/delivery.py`; `test_audience_delivery` (3), `test_phase5_replay` (2), gate 10 behavioral part | `core::delivery` | ported tests; float draws bit-equal (Section 7.2) | M1 WP-09 |
| Loonberg report, publication invariants | P | `engine/media/loonberg.py`, `PublicationInvariantError`; `test_report_is_inert` (2) | `core::media`, `core::scenario` | ported: the mutating-outlet test becomes a Rust test that injects a mutating outlet through a `tooling`-only trait object and expects `PublicationInvariantError` | M1 WP-09 |
| Commitments, monitoring, contingent obligations | P | `engine/commitments.py`, `engine/monitoring.py`; `test_commitment_carryover` (3), gate 06, gate 08 | `core::commitments`, `core::monitoring` | ported tests | M1 WP-09 |
| Intermeeting compression and keyed realization | P | `engine/compression.py`; `test_multi_seed_band` (2) | `core::compression` | ported tests including reseal with four seeds | M1 WP-09 |
| Next Morning Book and StaffReview with `verdict: None` and "No universal verdict is assigned." | P | `engine/postmortem.py`, `engine/harness/review.py`; `test_postmortem_labels` (3), gate 09 | `core::postmortem`, `api::ReviewView`, CLI renderer | ported tests; D5 keeps the in-world assertion | M1 WP-09, WP-14 |
| Replay identity (same inputs, same transcript) | P | gate 07, `test_replay`, `test_phase2/3_replay` | `core::scenario`, `reservist replay-check` | ported; three packages twice | M1 WP-09 |
| The five CLI subcommands and their output lines | P | `engine/cli.py` | `reservist-cli` | output comparison in parity harness | M1 WP-05, WP-09, WP-14 |
| Twelve REPL verbs and seven room renderings | P | `engine/cli.py`, `engine/harness/*` | `api` projections + `reservist-cli` renderer + Godot scenes | `test_postmortem_labels::test_interactive_advance…` ported as a scripted CLI test; Godot headless test drives the same verbs | M1 WP-14, WP-15 |
| `propose` mutating `package_id` before the meeting | P (M1) then A (M2) | `engine/cli.py:182` | M1 `SessionOp::Propose`; M2 folder slate admission | M1 parity; M2 G-09 | M1 WP-14, M2 WP-20 |
| Import-graph and token-scan tests (`test_access_boundary` test 1, `test_presentation_boundary`, gate 02 import part, gate 04 source scan, gate 10 import part) | replaced | `tests/*` | crate visibility (`pub(crate)`), `clippy.toml` disallowed types, `just godot-lint`, `cargo tree` dependency check | compile-time and lint evidence plus the behavioral assertions above | M1 WP-01, WP-15 |
| Exact checkpoint/resume | N | none (F-10) | `core::save`, `reservist save/resume` | equivalence at every event boundary (Section 7) | M1 WP-13 |
| Generated fidelity permission matrix, three-level validation for the MVP slice | N (G-03 partial) | none | `core::fidelity`, `content::bindings` | generated CSV equals `type_fidelity.csv` closure; manifest selections validated | M1 WP-12 (full matrix M4) |
| Compiler fail-closed loading (handler keys, eligibility, bindings) | N | partial (`manifest.py`) | `content::bindings` | negative fixtures | M1 WP-11 |
| No wall clock, no randomness, no network in core | N (G-07, G-22) | gate 10 import scan | `clippy.toml`, `cargo tree` check, Godot lint | lint output recorded | M1 WP-01, WP-15 |
| Godot receives only projections; rejected ops leave state hash unchanged | N (G-12) | none (F-11) | `api`, `reservist-godot` | headless Godot test | M1 WP-15 |
| Global catalog validity | conformance | `catalog/catalog.py` (214 issues) | data repair + `content::validate` | `catalog.py validate` passes; `content validate` passes | M1 WP-02, WP-03 |
| Chief of staff as named Person with dated OfficeHolding | A (H Chief of staff) | office only, `LIMITED_ROLE_HOLDER` | catalog rows, cast, manifest, `core::cognition` | G-15 | M2 WP-18 |
| Calendar board, discretionary periods, interruption queue (D3) | A (H Temporal cadence) | `IntermeetingCompressor`, `advance_to_next_consequential_event` | `core::calendar`, `api` | G-07, G-08 | M2 WP-19 |
| Folder handoff, practical cards, slate admission, speaking commits (D2) | A (H Commit and submit) | none | `core::folder`, `api::Command` | G-09, G-10, G-21 | M2 WP-20 |
| Typed commands, receipts for every command, idempotency, immutable projections | A (H Client boundary) | method calls | `api::Command`, `api::Receipt` | G-12 behavioral | M2 WP-17 |
| Evidence routing conflicts and deadline options | A (H Evidence access) | none | `core::routing` | G-13, G-14 | M2 WP-21 |
| Extradiegetic scorecard alongside in-world review (D5) | A (H Stewardship score) | none; `verdict: None` | `core::stewardship::Scorecard` | gate 09 gains scorecard assertion; StaffReview unchanged | M2 WP-22 |
| Chair succession | M3 | none | `core::succession` | G-16 | M3 |
| Full Stewardship ledger, findings, compensating entries, D1 terminal event | M3 | none | `core::stewardship::Ledger` | G-17 | M3 |
| RECORDED/RESPONSIVE channel providers, `supported_transitions` | M4 | stub adapter, unenforced `provider_binding` string | `core::providers`, `content::bindings` | G-20 | M4 |
| Asset provenance and Godot importing | M4 | `catalog/presentation_refs.csv` | `content::assets`, Godot import settings | G-23 | M4 |
| Full fidelity matrix for all 28 clades, universal-dispatch negative proof | M4 | none | `core::fidelity` | G-02, G-03 | M4 |
| Channel calibration budgets, provider parameters, point bands, transfer-learning evidence | R | none | none | G-24 and H "Still open" | research |

Rule applied throughout: a known Python defect is not preserved by fiat, but the M1 review found none in the replacement scope that changes observable behavior; the two defects found are in authoring tooling (`catalog.py:14`, `catalog.py:837`) and are fixed in WP-02.

---

## 5. Toolchain and repository setup

| Item | Choice | Basis |
|---|---|---|
| Rust | `rust-toolchain.toml`: `channel = "1.98.1"`, components `rustfmt`, `clippy`, `rust-src`; `edition = "2024"` | external: `static.rust-lang.org/dist/channel-rust-stable.toml` reports 1.98.1 (2026-09-01); repository: none pinned; machine: 1.98.1 not yet installed, rustup will fetch it on first `cargo` |
| godot-rust | `godot = { version = "=0.5.5", features = ["api-4-7"] }` | external: crates.io `godot` max 0.5.5 (2026-08-09); `godot/Cargo.toml` on master lists `api-4-7`; workspace `rust-version = "1.94"` (MSRV satisfied) |
| Godot | 4.7.2-stable, recorded in `godot/project.godot` `config/features` and in `justfile` `godot := "/opt/homebrew/bin/godot"` overridable | machine: `4.7.2.stable.official.ed1daf0bf`; external: GitHub releases show 4.7.2-stable (2026-08-18) as latest 4.x |
| Python oracle | `.python-version` = `3.14.3`; no `pyproject.toml` dependencies (zero deps) | F-20 |
| Decimal | `rust_decimal = "1.43"` with `RoundingStrategy::MidpointAwayFromZero` for Python `ROUND_HALF_UP`, `AwayFromZero` for `ROUND_UP`; 28-digit intermediate precision mirrors Python's default context for the two divisions in `dealer_cohort.py:74` and `leveraged_fund.py:42` | external crates.io 1.43.0 |
| Hashing | `sha2 = "0.11"` | external 0.11.0 |
| Time | `jiff = "0.2"` for offset-aware parsing and formatting; formatting must reproduce Python `isoformat()` exactly (`%Y-%m-%dT%H:%M:%S%:z`) | external 0.2.35 |
| Serialization | `serde = "1"`, `serde_json = "1"` for authored files; `core::canon` implements RFC 8785 itself (about 150 lines) rather than depending on `serde_json_canonicalizer 0.3.2`, because `test_canon` and the transcripts pin exact number formatting and the Python implementation is small and fully readable | inspected `engine/canon.py` |
| CSV | `csv = "1.4"` | external 1.4.0 |
| Errors | `thiserror = "2"` | external 2.0.20 |
| Lints | `clippy.toml`: `disallowed-types = [std::time::Instant, std::time::SystemTime, std::net::*]`, `disallowed-methods = [std::time::Instant::now, std::time::SystemTime::now]`; `#![deny(clippy::disallowed_types, clippy::disallowed_methods)]` in `reservist-core`, `reservist-content`, `reservist-godot`; `cargo fmt --check`; `cargo clippy -D warnings` | G-07, G-22 |
| Dependency check | `just deps-check`: `cargo tree -e normal -p reservist-core -p reservist-content -p reservist-godot` must not contain `tokio`, `hyper`, `reqwest`, `ureq`, `rand`, `getrandom`, `chrono` with `clock`; failure is a lint failure | G-22 |
| Build determinism | `Cargo.lock` committed; `cargo build --locked` in `just`; `godot4-prebuilt` git dependency of `godot` is pinned by `Cargo.lock` revision (build-time only, not inside game bounds) | H Repository |
| `justfile` additions | `rust-build`, `rust-test`, `fmt`, `lint`, `deps-check`, `parity`, `oracle-test` (Python), `oracle-vectors`, `godot-lint`, `godot-test` (headless), `check` (all of the above); existing `validate`/`freeze`/`run`/`replay`/`play` retargeted to `cargo run -p reservist-cli --locked --` in WP-16, with `oracle-*` recipes keeping the Python entry points | continuity |
| Artifact comparison | `reservist parity` (Section 7) writes `target/parity/<vector>.report.json`; `just parity` fails on any unattributed divergence | G-18 |

---

## 6. Catalog conformance plan

### 6.1 The 214 issues by root cause (executed grouping of `catalog.py validate` output)

| Group | Rows | Issue count | Mechanism |
|---|---|---|---|
| A1 relationship placeholders | 19 rows in `catalog/inventory/media_information/relationships.csv` (`rel.belongs_to.media.*` 3, `rel.holds.media.*` 3, `rel.operates.media.*` 6, `rel.projects.media.honkbox_posting_population.honkbox`, `rel.publishes.market.credit_rating_agencies.credit_ratings`, `rel.publishes.us.bea.pce`, `rel.publishes.us.bls.cpi`, `rel.publishes.us.new_york_fed.sofr`, `rel.sources.media.*` 3) with `UNKNOWN` in `effective_period`, `observability`, `lifecycle_and_exit`, `witness_kind` | 76 `schema` + 76 `relationship` = 152 | each UNKNOWN is reported twice: "UNKNOWN is not allowed in this field" and "missing <field>" |
| A2 unresolved owner | `rel.projects.media.honkbox_posting_population.honkbox` `canonical_owner_id=UNKNOWN` | 1 `schema` + 1 `relationship` = 2 | owner must be an instance |
| A3 transmission placeholders | 8 rows in `catalog/inventory/media_information/transmissions.csv` (`tx.measurement.us_consumer_prices.cpi_pce_references`, `…pce_reference`, `tx.media.aftv.mass_public_belief`, `tx.media.gnbc.mass_public_belief`, `tx.media.honkbox.aftv`, `tx.media.honkbox.gnbc`, `tx.media.loonberg.honkbox`, `tx.media.the_herd.loonberg`): `unit` UNKNOWN (8), `fallback_behavior` UNKNOWN (8), `capacity_ref` UNKNOWN (8), `effective_delay` UNKNOWN (6), `persistence_or_expiry` UNKNOWN (6) | 36 `schema` + 8 `unit` + 8 `endpoint` = 52 | same double reporting for unit and fallback |
| A4 unresolved endpoints | the two `mass_public_belief` rows: `consuming_entry_id=UNKNOWN`, `transformation_owner_id=UNKNOWN` | 4 `schema` + 4 `endpoint` = 8 | consumer must be an instance |
| Total | 27 rows | 214 | |

Dependencies: none of the 27 rows is read by `engine/` (the runtime reads only `catalog_slice.json`; `freeze` reads `entities`, `types`, `type_fidelity`, `period_variants`, `entity_authority_sources`, `entity_fallback_contracts`, `owned_state`, `owned_state_transitions`, `schema.json`) (inspected `engine/catalog_slice.py`). Repairing them cannot change `catalog_definition_hash` or the scenario hash. Both failing catalog tests fail only because `validate`/`generate` exit non-zero; no test asserts anything about these rows' values.

### 6.2 Recommendation: full global repair in M1, no quarantine mechanism

Quarantine is not recommended. The invalid set is 27 rows in one domain, one row (`rel.publishes.us.bls.cpi`) touches an in-slice entity, and a compiler quarantine table would be a standing abstraction with one historical use. Full repair is smaller than the quarantine machinery and leaves nothing hidden.

Fail-closed behavior is stronger than quarantine and is unconditional: `reservist-content` refuses to freeze or load any scenario while the global catalog has a structural issue, and `catalog.py validate` remains the authoring gate. There is no flag, table, or environment variable that admits an invalid row. If a future inventory domain arrives invalid, the repository's `just check` fails until it is repaired or removed from `catalog/inventory/` by an ordinary commit that the Build Review records.

### 6.3 Repair content (WP-02)

- A1: author the four fields for each of the 19 rows. The 15 fictional media rows (AFTV, GNBC, HonkBox, The Herd, Wool Street Journal, their offices, companies, and sourced claims) receive `effective_period` `2006-01-01/2006-12-31` (the fixture period used by the Fed rows is `2006-02-01/2006-12-31`; media entities predate the Chair's term), `observability` `public`, `lifecycle_and_exit` `persists through 2006; exit by editorial reorganization`, `witness_kind` `publication_witness` or `office_holding_witness` as appropriate, with provenance `authored for satire register; RIP-2026-09-05-01 §6.3`. The four real-world `publishes` rows use researched values with source URLs in `provenance`: BLS CPI (monthly publication, ongoing), BEA PCE (monthly, ongoing), NY Fed SOFR (first published 2018-04-03, so `effective_period` `2018-04-03/open`, which correctly makes it out of the 2006 fixture's period), credit ratings (ongoing). Normalize the media rows' `relationship_family` casing to match `HOLDS`.
- A2: `canonical_owner_id` = `network.media.honkbox` (the projected population is a lens over the network's posting activity; the network is the state owner, the lens is non-owning per D05).
- A3: `unit` = `NONE` for `payload_kind=observation` rows (claims and observations carry no conserved unit; `NONE` is a permitted semantic value, `UNKNOWN` is not); `fallback_behavior` = `absent when the consuming outlet or network is not selected` (media rows) and `publication proceeds without the measurement transmission; reference retains last publication` (measurement rows); `capacity_ref` = `NONE`; `effective_delay` = `editorial_selection_delay` (media) and `scheduled_release_delay` already present (measurement); `persistence_or_expiry` = `claim_record_persists`.
- A4: `consuming_entry_id` and `transformation_owner_id` = `population.us.person.cells` (Section 1, item 1), `uncertainty_notes` updated to say the mass-public belief owner is the person-cell population.
- Tooling defects: `catalog.py:14` `REPO_ROOT = ROOT.parent`; `catalog.py:837` replaced by a scan of `scenarios/*/manifest.json` that emits `scenario representation manifest: <id> (<replay_hash>)` per discovered manifest or `no scenario representation manifest is authored` when none exists; regenerate `catalog/generated/` (fixes the 492/493 staleness).

Verification: `python3 catalog/catalog.py import && python3 catalog/catalog.py validate` exits 0; `just catalog-test` 17/17; `just freeze` leaves `scenarios/` unchanged (`jj diff --stat scenarios` empty) and prints the pinned hash; `just test` 86/86.

### 6.4 MVP slice contract closure (WP-03)

The 39 selected entries are `selectable_in_manifest=false` and fail `catalog.py`'s eligibility (missing `relationship`, `transmission`, `probe` checks) even though the runtime executes them. H's first eligibility concept ("validly represented") must hold for every selected entry before the Rust compiler can enforce `selected ⊆ eligible`. Repair the contract, then declare it:

1. Relationships derived from scenario sources already in the repository: `HOLDS` exist for the Chair; add `HOLDS` for `role_holder.fomc.governor_1/2` → `office.us.federal_reserve.governor` (`cast/fomc_2006.json` effective period), `MEMBER_OF` governors and Chair → `body.us.federal_reserve.fomc`, `BELONGS_TO` the three staff units → `inst.us.federal_reserve.board`, `AUTHORIZES` each legal instrument → its `target_owners` (from `legal/*.json`), `PUBLISHES` `adapter.macro.us.broad` → `reference.us.bls.cpi`, `SCHEDULES` `schedule.us.federal_reserve.fomc` → `body.us.federal_reserve.fomc`, `DERIVES` blackout → fomc schedule, `LENDS_TO` dealer → leveraged funds via `agreement.us.repo.bilateral`, `RECORDS` each `record.*` → its producing owner, `PROJECTS` each `pop.*` lens → `population.us.person.cells`, `ALLOCATES` `household.us.cohorts` → `population.us.person.cells`. Every field filled from the fixture; provenance cites the scenario file and line.
2. Transmissions derived from `manifest.json` `delivery_edges` (9 rows: statement and report edges with `payload_kind=observation`, `unit=NONE`, delay in minutes, `fallback_behavior` "edge absent when recipient not selected") plus the release tape (`adapter.macro.us.broad` → `reference.us.bls.cpi` → `person.us.ben_bernankey`), the market cycle (`cohort.us.dealer.primary`, `inst.us.leveraged_funds`, `adapter.market.us.treasury.external_buyer`, `inst.us.federal_reserve.new_york` → `market.us.treasury.secondary`, `payload_kind=quantity`, `unit=treasury_face`), the repo maturity (`agreement.us.repo.bilateral` → `inst.us.leveraged_funds`, `unit=USD`), and staff evidence (`staff.us.federal_reserve.markets` → `person.us.ben_bernankey`, observation).
3. Probe coverage: add architecture probe `probe.mvp_2006_cycle.replay_identity` rows for all 39 entries with `coverage_status=covered`, requirement "executes in the frozen MVP cycle with replay identity", provenance `just replay` state hashes. This is an executed probe, not a coverage flag.
4. Set `selectable_in_manifest=true` for the 39 entries **only after** steps 1 to 3 make `catalog.py audit-eligibility` report them eligible. The flag then records a true declaration; the validator's `catalog_eligibility` rule rejects `true` without a complete contract, which is the guard against flag-flipping.

The frozen projection excludes `selectable_in_manifest`, relationships, transmissions, and probes, so the scenario hash remains `493ca024…` (inspected `engine/catalog_slice.py`; verify with `just freeze`). `completeness.txt` "slice-candidate catalog closure" rises from 0/23 toward the number of slice candidates actually selected (14 of the 39 carry `slice_candidate`); the remaining 9 slice candidates stay honestly unclosed.

### 6.5 Sequence

WP-02 → WP-03 → (Rust compiler WP-05 enforces both) → WP-11 adds handler and binding checks → WP-12 adds fidelity checks. Each step keeps `just catalog-test`, `just test`, and the scenario hash green.

---

## 7. Parity, checkpoint/resume, and acceptance strategy

### 7.1 Oracle vectors

`tools/oracle/export_vectors.py` (Python, added in WP-04, before the engine moves) runs `ScenarioRuntime` for every combination in the table and writes canonical JSON to `tests/oracle/vectors/<package>__<request_mode>.json`: `scenario_hash`, `state_hash`, `transcript` (list of event dicts), `result` (every `RunResult` field), `state_snapshot` (the exact dict `_state_hash` hashes), `material_snapshot`, `receipts`, and `cli_stdout` for `run` and `replay-check`. Vectors: 3 packages × request modes {None, NORMAL, ACCELERATED, DECLINED, MISSED} = 15 runs, plus the four reseeded `test_multi_seed_band` runs, plus the `play` command script from `test_postmortem_labels` with captured stdout. Sizes are under 100 KB each. The vectors are committed once at revision `97fc926a` behavior and never regenerated for M1; regenerating requires a Build Review entry.

### 7.2 Comparison rules

`reservist parity --vectors tests/oracle/vectors --attributions tests/parity/attributions.toml` runs the Rust runtime with identical frozen inputs and compares:

| Field class | Rule |
|---|---|
| Event count, `event_id`, `sequence`, `completion_time` (as instant and offset), `transition_kind`, `responsible_owner`, `causal_parent`, `observation_policy` | exact |
| Identifiers, enums, statuses, categorical outcomes, claim ids, receipt stages and statuses, allocation keys | exact |
| Decimal strings (`quantity`, `price`, `limit_price`, `cash_amount`, `balance`, `reserved`, `residual`, `filled_quantity`, `liquidity_deficit`, `principal`, `haircut`, `capacity`, `delta`) | exact numeric equality after decimal normalization (Python `Decimal("5")` and Rust `5.0000` compare equal). H waives byte identity; the Rust transcript is itself deterministic and hashed |
| f64 fields (`policy_path_estimate`, `annualized_core_inflation`, belief `estimate`/`lower`/`upper`/`confidence`, claim `confidence`, edge probabilities) | exact after both sides are rendered with RFC 8785 shortest representation; the algorithms are integer SHA-256 draws divided by 2^64, IEEE adds, and Python `round(x, n)`, all reproducible bit for bit (executed spot check: Python `round` and Rust `{:.n}` agree on 0.25, 0.35, 2.675, 3.049…, 0.545). Any residual difference is reported with its ulp distance and fails unless attributed with a declared tolerance |
| Free text (reasons, headlines, explanations, question templates) | exact |
| State hashes | not compared across runtimes (they hash decimal strings); instead `state_snapshot` is compared field by field under the rules above, and the Rust state hash is compared with itself across replay and resume |

`attributions.toml` entries have `path` (JSON pointer glob), `category` ∈ {`normalization`, `authorized_difference`, `new_runtime_only`}, `clause` (H section for `authorized_difference`), `justification`. In M1 the only permitted category is `normalization` (decimal scale). Any divergence without an entry fails `just parity`. This is G-18's attribution requirement.

### 7.3 Deterministic replay, conservation, receipt-chain witnesses

- Replay: `reservist replay-check` runs each package twice, compares transcript bytes and state hash (the Python algorithm), and additionally runs `save` at event 0 then `resume`, asserting equality (ties G-19 into replay).
- Conservation witness: after every accounting commit the ledger asserts opening totals; the parity report includes `conserved_totals` per (instrument, unit) at end of run and requires exact equality with the oracle's `state_snapshot.accounting`.
- Receipt chain: parity asserts that the ordered `stage_receipt_recorded` payloads equal `result.receipts` (Python `test_receipts::test_receipts_are_witnessed_in_order`) and that every `causal_parent` names an earlier `event_id` or a scheduled `stable_id`.

### 7.4 Exact checkpoint/resume (G-19)

**Supported save boundaries.** After any `advance_next` returns and before the next event is popped (the clock is quiescent), and before the run starts. In M1 that is every event boundary of every run; in M2 the calendar board only stops at these boundaries, so folders never straddle a save. Saving mid-handler is impossible by construction (no API).

**Format.** `SaveFile` v1, canonical JSON (RFC 8785), one document:

```
save_schema_version: 1
engine_identity: { crate_version, registry_version (hash of phase::metadata), fidelity_matrix_hash }
scenario: { scenario_hash, frozen_scenario_path_hint, package_id, request_mode }
clock: { current_time, dynamic_event_sequence, queue: [ScheduledEvent…] }        # outstanding events and work
ledger: [DomainEvent…]                                                          # immutable records
registry: { owner_id: state… }                                                  # institutional and canonical state
accounting: { accounts, reservations, transactions }
participants, dealers, leveraged_funds, external_buyer, repo
population, households, population_views
claims_registry_version, communication_acts, reports, audience_receptions, audience_beliefs
commitments, monitoring, policy_commitment_id, communication_commitment_id
staff: { units: { capacity: {deliverables, reservations}, evidence } }, tasks, assessments     # reservations and commitments
compression: { steps, realizations }
publication_order_witnesses, completed_publication_market_artifacts, latest_market_result, latest_publication_market_result, latest_market_settlement, latest_repo_settlement
receipts, next_morning_book, staff_review
player_records: { delivery_order, records (with read flags) }                   # private and delivered state
body_hash: sha256 of everything above
```

Private state (participant beliefs, staff evidence stores, macro hidden state) is inside the file because the file is the runtime's own persistence, not a player projection; the file is never handed to Godot, only its path.

**Equivalence proof.** `tests/save_resume.rs`: for each package and request mode, for every k from 0 to N events: run k events, `save`, `resume` into a fresh `Runtime`, run to completion; assert `state_hash`, transcript bytes, `result` equal the uninterrupted run; assert `save` then `resume` then `save` yields byte-identical files. The CLI exposes `reservist save --after-events k` and `reservist resume <file>` for manual use.

**Failure behavior.** `resume` returns `ResumeError` with category `schema_version`, `engine_identity`, `scenario_hash`, `body_hash`, `unknown_handler`, `unknown_owner`, or `queue_integrity` (duplicate id or sequence, past due time). No partial `Runtime` is ever constructed; the constructor is total or fails before any state exists. Same category discipline as `ManifestValidationError`.

### 7.5 Test and gate mapping

| Oracle test file (count) | Work package | Disposition |
|---|---|---|
| `test_canon` (4) | WP-04 | ported |
| `test_manifest_closure` (4), `test_initialization` (4), `test_phase6_catalog_slice` (2), `test_replay::test_different_seed…` | WP-05 | ported |
| `test_event_order` (2), `test_authority` (4), `test_access_boundary::test_player_record_contains…` | WP-06 | ported; import-graph test replaced by visibility |
| `test_request_lifecycle` (4), `test_displaced_work` (1), `test_assessment_provenance` (3), `test_phase4_replay` (1), `test_phase4_vote` (1) | WP-07 | ported |
| `test_policy_cycle` (5), `test_market_clearing` (3), `test_accounting` (3), `test_repo_lifecycle` (1), `test_receipts` (4), `test_provider_parity` (1), `test_population_conservation` (3), `test_phase2_replay` (1), `test_phase3_replay` (1) | WP-08 | ported |
| `test_claim_grammar` (4), `test_audience_delivery` (3), `test_phase5_replay` (2), `test_report_is_inert` (2), `test_commitment_carryover` (3), `test_multi_seed_band` (2), `test_replay::test_same_seed…`, `test_postmortem_labels` tests 2 and 3 | WP-09 | ported |
| `test_postmortem_labels::test_interactive_advance…` | WP-14 | ported as scripted CLI test |
| `test_presentation_boundary` (1) | WP-01, WP-15 | replaced by `pub(crate)` visibility of `state`, `observation`, `cognition`, `markets`, `participants` and by the projection-type boundary; a behavioral test asserts projections serialize without `display_name`-free canonical fields leaking (the display names live in `api` views only) |
| Gate 01 | WP-05 | ported |
| Gate 02 | WP-06, WP-15 | behavioral part ported; import part replaced by visibility |
| Gate 03, 05, 06 | WP-07, WP-08, WP-09 | ported |
| Gate 04 | WP-08 | behavioral parts ported; `package_id` source scan replaced by module dependency: `core::markets` cannot name `core::packages` (enforced by a unit test over `cargo modules`-free approach: `packages` is `pub(crate)` inside `bodies` and `markets` has no `use` of it, checked by a build-time `#[cfg(test)]` assertion that `markets` compiles as a standalone module in `tests/markets_isolation.rs`) |
| Gate 07 | WP-09, WP-13 | ported plus save/resume |
| Gate 08 | WP-08, WP-09 | ported |
| Gate 09 | WP-14 (M1), WP-22 (M2 adds the scorecard assertion under D5) | ported |
| Gate 10 | WP-01, WP-09, WP-15 | behavioral part ported; import scan replaced by `clippy.toml`, `just deps-check`, `just godot-lint` |

### 7.6 Exit criteria

**M1 exit (all required):**
1. `just check` green: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --locked` (all ported tests, registry, fidelity, save/resume), `just deps-check`, `just godot-lint`, `just godot-test`, `just parity` with zero unattributed divergences and only `normalization` attributions, `just catalog-test` 17/17, `python3 catalog/catalog.py validate` exit 0, `just oracle-test` 86/86.
2. `reservist validate` prints `validation passed: sha256:493ca024…8d066`; `reservist freeze` leaves `scenarios/` byte-identical; `reservist replay-check` passes for three packages with Rust state hashes recorded in the Build Review.
3. `reservist run` output lines for every vector equal the oracle `cli_stdout` after decimal normalization.
4. `reservist play` accepts the twelve verbs and reproduces the `test_postmortem_labels` script; the Godot client drives the same twelve verbs headless and its rejection test leaves the state hash unchanged.
5. Save/resume equivalence holds at every event boundary for all 15 vectors.
6. `catalog/generated/fidelity_permissions.csv`, `handler_registry.csv`, `phase_flow.mmd` are generated and committed; the compiler rejects the negative fixtures in `tests/content/negative/` (duplicate contract id, unresolved owner, unknown handler key, incompatible schema version, missing witness kind, invalid period variant, unpermitted fidelity, `selectable=true` with incomplete contract).
7. `engine/` and `tests/` have moved under `tools/oracle/`; the `justfile` production recipes call `reservist`; nothing under `crates/` or `godot/` imports or executes Python.
8. `docs/BUILD_REVIEW.md` records the above with executed/inspected labels and the working-copy revision.

**M2 exit (all required):**
1. M1 exit criteria still hold, with `attributions.toml` now carrying `authorized_difference` entries for every M2 behavior change, each citing its H clause and replacement acceptance test.
2. G-08, G-09, G-10, G-12 (behavioral, not token scan), G-13, G-14, G-15, G-21 tests pass in `cargo test` and the relevant Godot headless tests.
3. Gate 09 asserts both "No universal verdict is assigned." in the in-world review and the presence of the extradiegetic scorecard with an authored verdict and signed delta (D5); `test_population_conservation` still asserts no `economy_score` in lenses.
4. The re-frozen scenario (chief of staff person added) has its new scenario hash recorded in the Build Review and the Build Mandate baseline table gains an "M2 fixture" row; the M1 oracle vectors remain pinned to `493ca024…` and continue to pass against a `--fixture m1` flag that loads the M1 frozen scenario retained under `scenarios/mvp_2006_cycle_m1/`.
5. `save`/`resume` equivalence includes open folders (penciled state is not persisted; a save is only offered at quiescent boundaries and the client refuses save while a folder is open), interruption queue contents, calendar periods, and the scorecard.

---

## 8. Ordered implementation work packages

Each package is a coherent commit (or two) with its own verification. Dependencies are listed; "earlier" means all prior packages.

### WP-01 Workspace and toolchain pins

- **Objective:** create the Cargo workspace, toolchain and version pins, lint configuration, and `just` recipes, with empty-but-compiling crates.
- **Why here:** everything else needs a build, and the mandate lists the absent toolchain as a blocker.
- **Inspect:** `justfile`, `.gitignore`, `PAPERCUTS.md`.
- **Paths:** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `clippy.toml`, `.python-version`, `crates/reservist-core/{Cargo.toml,src/lib.rs}`, `crates/reservist-content/…`, `crates/reservist-cli/…`, `crates/reservist-godot/{Cargo.toml,src/lib.rs}` (with `godot` dependency and an empty `ExtensionLibrary`), `godot/project.godot`, `godot/reservist.gdextension`, `.gitignore` additions (`target/`, `godot/.godot/`).
- **Interfaces:** none public yet; `reservist-core::api` module stub.
- **Behavior:** none changed; Python still production.
- **Fixtures:** none.
- **Verification:** `cargo build --locked`, `cargo test --locked`, `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, `just deps-check`, `just test` (86), `godot --headless --path godot --quit` exits 0.
- **Evidence:** command outputs in the Build Review "toolchain" row; `rustup show` after the pin.
- **Completion:** all commands above exit 0 on a clean checkout.
- **Risks:** `godot` crate build needs its prebuilt API (git dependency) on first build; record the lockfile revision. If 1.98.1 cannot be installed offline, pin the installed 1.96.1 and record it (MSRV 1.94 still satisfied).
- **Commit:** `chore: Add Rust workspace and toolchain pins`

### WP-02 Repair the media inventory and catalog tooling

- **Objective:** make the global catalog validate; fix the two tooling defects; regenerate `catalog/generated/`.
- **Why here:** independent of Rust, unblocks fail-closed compilation, and clears the two failing catalog tests.
- **Inspect/replace:** `catalog/inventory/media_information/relationships.csv` (19 rows), `catalog/inventory/media_information/transmissions.csv` (8 rows), `catalog/catalog.py:14`, `catalog/catalog.py:803-841`.
- **Paths:** the two inventory files, `catalog/catalog.py`, all normalized root CSVs touched by `import`, `catalog/generated/**`.
- **Behavior preserved:** engine behavior and scenario hash unchanged.
- **Fixtures:** Section 6.3 values.
- **Verification:** `python3 catalog/catalog.py import && python3 catalog/catalog.py validate` exit 0; `just catalog-test` 17/17; `just catalog-generate`; `cat catalog/generated/completeness.txt` shows the manifest line naming `manifest.mvp_2006_cycle`; `just freeze` then `jj diff --stat scenarios/` empty; `just test` 86/86.
- **Evidence:** validator output "validation passed: 527 entities; zero structural gaps; zero warnings" (entity count as printed), test summary, unchanged scenario hash.
- **Completion:** all of the above.
- **Risks:** the `test_import_and_generation_are_byte_deterministic` test copies the whole project tree; keep `target/` ignored so it stays fast.
- **Commit:** `fix: Repair media inventory rows and catalog readiness report`

### WP-03 Close the MVP slice catalog contract

- **Objective:** make the 39 selected entries catalog-eligible and declare them selectable.
- **Why here:** the Rust compiler (WP-05) will enforce `selected ⊆ eligible`; data must be true first.
- **Inspect:** `catalog/generated/catalog_eligibility.csv`, `catalog/relationships.csv`, `catalog/transmissions.csv`, `catalog/probe_coverage.csv`, `scenarios/mvp_2006_cycle/{manifest,initialization}.json`, `cast/fomc_2006.json`, `legal/*.json`, `staff/work_2006.json`.
- **Paths:** `catalog/inventory/mvp_phase*/relationships.csv` (new), `…/transmissions.csv` (new), `…/probe_coverage.csv` (new), `entities.csv` rows in `mvp_phase*` and `fed_treasury` (flag flip after eligibility).
- **Behavior preserved:** runtime unchanged; scenario hash unchanged.
- **Verification:** `python3 catalog/catalog.py import && python3 catalog/catalog.py audit-eligibility` reports the 39 as eligible before the flag flip; then `validate` exit 0 after the flip; `just freeze` no diff; `just test`, `just catalog-test` green; a new Python oracle test `tools/oracle/tests/test_slice_eligibility.py` asserts every manifest-selected id is `catalog_eligibility=true` (this test moves to Rust in WP-05).
- **Evidence:** `completeness.txt` before/after lines for "catalog-eligible instances" and "slice-candidate catalog closure".
- **Completion:** all 39 eligible and selectable; validation green; hash unchanged.
- **Risks:** authoring 60 to 80 rows; each needs provenance to a scenario file line, not prose.
- **Commit:** `feat: Close the MVP slice catalog contract`

### WP-04 Canonical JSON, hashing, and oracle vectors

- **Objective:** `core::canon` byte-identical to `engine/canon.py`; export the oracle vectors.
- **Why here:** every later comparison depends on it; exporting vectors before touching `engine/` freezes the oracle.
- **Inspect/replace:** `engine/canon.py`, `tests/test_canon.py`.
- **Paths:** `crates/reservist-core/src/canon.rs`, `tools/oracle/export_vectors.py`, `tests/oracle/vectors/*.json`, `tests/oracle/README` line in the Build Review instead of a new doc.
- **Interfaces:** `canon::canonical_bytes(&Value) -> Result<Vec<u8>, CanonError>`, `canon::sha256(&Value) -> String` (`sha256:` prefix), `canon::load_json(path)` rejecting NaN/Infinity.
- **Fixtures:** the four `test_canon` vectors; every transcript line of the 15 vectors.
- **Verification:** `cargo test -p reservist-core canon`; a test reads each vector's `transcript`, canonicalizes each event object, and asserts equality with the bytes of the Python transcript line (proves RFC 8785 number and string formatting on real data); `just oracle-vectors` is idempotent (re-export produces identical files).
- **Evidence:** test output; vector file hashes listed in the Build Review.
- **Completion:** all transcript lines byte-equal.
- **Risks:** float formatting of values such as `0.545` and `3.049` (Python `repr` shortest round-trip); Rust's `ryu`-based `{}` formatting is also shortest round-trip, and the vector test proves it.
- **Commit:** `feat: Add canonical JSON hashing and oracle vectors`

### WP-05 Content compiler: validate and freeze

- **Objective:** `reservist validate` and `reservist freeze` reproduce Python outputs byte for byte; the compiler enforces global catalog validity and `selected ⊆ eligible`.
- **Why here:** the runtime consumes frozen data; parity of the frozen inputs is the precondition for runtime parity.
- **Inspect/replace:** `engine/catalog_slice.py`, `engine/manifest.py`, `engine/initialization.py`, `engine/scenario.py:78-207`, `catalog/catalog.py:69-575`, `tests/test_manifest_closure.py`, `tests/test_initialization.py`, `tests/test_phase6_catalog_slice.py`, `tests/test_replay.py` (reseal).
- **Paths:** `crates/reservist-content/src/{schema,inventory,validate,eligibility,slice,manifest,initialization,tape,authority_content,frozen,lib}.rs`, `crates/reservist-cli/src/main.rs` (`validate`, `freeze`), `tests/content/negative/*` fixtures (copies of the scenario with one mutation each, generated by a test helper rather than committed).
- **Interfaces:** `content::validate_scenario(dir) -> Result<FrozenScenario, ContentError{category, message}>`, `content::seal_scenario(dir, catalog_dir) -> Result<String, ContentError>`, `content::validate_catalog(catalog_dir) -> Result<Tables, Vec<Issue>>`.
- **Behavior preserved:** identical category strings; freeze writes identical bytes.
- **Fixtures:** `scenarios/mvp_2006_cycle`, the 39-entry slice.
- **Verification:** `cargo test -p reservist-content`; `cargo run -p reservist-cli -- validate scenarios/mvp_2006_cycle` prints the pinned hash; `cargo run -p reservist-cli -- freeze scenarios/mvp_2006_cycle` then `jj diff --stat scenarios/` empty; `cargo run -p reservist-cli -- validate-catalog catalog` exit 0 and, on a temp copy with one UNKNOWN reintroduced, exit 1 with the same category the Python validator prints.
- **Evidence:** outputs recorded; negative-fixture table in the Build Review.
- **Completion:** all above; ported tests pass.
- **Risks:** CSV quoting differences between Python `csv` and the `csv` crate when re-writing normalized tables; the Rust compiler reads but does not write CSVs in M1 (`catalog.py import` remains the writer), which avoids the risk.
- **Commit:** `feat: Add content compiler validate and freeze`

### WP-06 Clock, registry, ledger, legal, and the release path

- **Objective:** first runnable vertical slice: load frozen scenario, run through `morning_book.open`, the first CPI release, observation delivery, `phase_1.checkpoint`, with the `WorkHandler` registry in place.
- **Why here:** exercises canon, clock, ledger, state owners, observation, player records end to end with a comparable transcript prefix.
- **Inspect/replace:** `engine/clock.py`, `engine/witness.py`, `engine/state/*.py`, `engine/legal.py`, `engine/authority.py`, `engine/records.py`, `engine/observation.py`, `engine/player/records.py`, `engine/scenario.py:213-364` (construction) and `:1495-1560` (`_handle` for releases), `tests/test_event_order.py`, `tests/test_authority.py`, `tests/test_access_boundary.py` test 2.
- **Paths:** `crates/reservist-core/src/{time,clock,phase,witness,state/*,legal,authority,records,observation,player,scenario/{mod,runtime,handlers/release,handlers/inert}}.rs`, `crates/reservist-cli` `run --until <time>`.
- **Interfaces:** `phase::WorkHandler { const KEY: &str; const PHASE: Phase; const READS: &[&str]; const WRITES: &[&str]; fn handle(rt: &mut Runtime, ev: &ScheduledEvent) -> Result<(), Defect> }`, `phase::Registry::lookup(key) -> Option<Erased>`, `phase::metadata() -> Vec<HandlerMeta>`; `Runtime::new(&FrozenScenario, package, request_mode)`, `Runtime::advance_next() -> bool`, `Runtime::transcript_bytes()`.
- **Behavior preserved:** `run_started`, `aleatory_path_registered`, contingent monitoring registration, `scheduled_event_handled`, `macro_release_measured`, `published_reference_updated`, `observation_produced`, `evidence_delivered`, `player_record_read`, `command_rejected`.
- **Fixtures:** vector `MEASURED_FIRMING__none` transcript prefix through 2006-03-27T09:00.
- **Verification:** `cargo test -p reservist-core clock authority player`; `reservist run --until 2006-03-27T09:00:00-05:00 --transcript out.jsonl` and `reservist parity --prefix` shows zero divergences on the first 7 events; chair-only command test asserts `REJECTED_NO_APPLICABLE_DELEGATION` and unchanged registry hash.
- **Evidence:** prefix parity report.
- **Completion:** prefix parity plus ported tests.
- **Risks:** timestamp formatting (`isoformat()` has no fractional seconds; offsets `-05:00`/`-04:00` across the DST boundary in the fixture) must be reproduced; `time` tests cover both offsets.
- **Commit:** `feat: Add clock, witness ledger, and release path`

### WP-07 Staff capacity, analytical tasks, assessments

- **Objective:** `request_follow_up` in all four modes with displacement, completion, assessment delivery, participant belief revision.
- **Why here:** staff work precedes the meeting in the fixture timeline and changes the vote.
- **Inspect/replace:** `engine/staff/*.py`, `engine/uncertainty.py`, `engine/cognition/{belief,participant,revision}.py`, `engine/monitoring.py` (registration only), `engine/scenario.py:365-520`, `tests/test_request_lifecycle.py`, `tests/test_displaced_work.py`, `tests/test_assessment_provenance.py`, `tests/test_phase4_replay.py`, `tests/test_phase4_vote.py`.
- **Paths:** `core/src/{staff/*,cognition/*,monitoring,scenario/handlers/staff}.rs`, `Runtime::request_follow_up(mode)`.
- **Behavior preserved:** stable id `scheduled.staff.markets.dealer_capacity_follow_up`, sequence 100, priority 25; capacity arithmetic; MISSED after deadline releases capacity.
- **Fixtures:** vectors `*__NORMAL`, `*__ACCELERATED`, `*__DECLINED`, `*__MISSED` through 2006-03-28T08:59.
- **Verification:** ported tests; prefix parity through the staff completion events.
- **Completion:** prefix parity for all four modes.
- **Risks:** `UnitScopedEvidenceStore.canonical_read` raising is a boundary contract; in Rust the store has no such method, and the test becomes a compile-time guarantee plus a behavioral assertion that assessments cite only delivered ids.
- **Commit:** `feat: Add staff capacity and analytical tasks`

### WP-08 FOMC authority cycle, market, settlement, repo, receipts

- **Objective:** the meeting handler with execution, clearing, settlement, repo settlement, receipts, and market observation.
- **Why here:** the fixture's central causal chain; most oracle tests depend on it.
- **Inspect/replace:** `engine/bodies/fomc.py`, `engine/packages.py`, `engine/execution/desk.py`, `engine/markets/treasury_secondary.py`, `engine/settlement/envelope.py`, `engine/accounting/ledger.py`, `engine/agreements/repo.py`, `engine/participants/*.py`, `engine/adapters/treasury_demand.py`, `engine/population/*.py`, `engine/scenario.py:600-1000, 1250-1400`, tests listed in Section 7.5.
- **Paths:** `core/src/{bodies/fomc,packages,execution/desk,markets/treasury_secondary,settlement,accounting,agreements/repo,participants/*,adapters/treasury_demand,population/*,scenario/handlers/{fomc,repo}}.rs`.
- **Behavior preserved:** vote logic, narrowing, directive authority refs, desk preflight statuses, clearing algorithm and rounding, settlement statuses and receipt ordering, repo deficit `24`, conservation.
- **Fixtures:** all vectors through 2006-03-28T09:00.
- **Verification:** ported tests; prefix parity through the AUTHORITY phase including exact Decimal equality of fills and balances.
- **Completion:** parity through the meeting for all 15 vectors.
- **Risks:** Decimal scale and division semantics (`available_cash / Decimal("0.9900")` at 28 digits then `quantize(0.0001)`); `rust_decimal` division precision differs from Python's context, so the plan uses `Decimal::checked_div` then `round_dp_with_strategy(4, MidpointAwayFromZero)` and proves equality against the vectors; if a value differs, the fix is in the Rust arithmetic path, never in the vectors.
- **Commit:** `feat: Add FOMC authority, market, and settlement cycle`

### WP-09 Publication, audience, media, commitments, compression, review

- **Objective:** full-run parity for all 15 vectors and the reseeded runs.
- **Why here:** completes the cycle so `run` and `replay-check` become comparable.
- **Inspect/replace:** `engine/claims.py`, `engine/communication.py`, `engine/delivery.py`, `engine/media/loonberg.py`, `engine/commitments.py`, `engine/monitoring.py` (attach/review/close), `engine/compression.py`, `engine/postmortem.py`, `engine/scenario.py` remaining handlers, tests listed in Section 7.5.
- **Paths:** `core/src/{claims,communication,delivery,media/loonberg,commitments,compression,postmortem,scenario/handlers/{publication,media,reception,monitoring,commitment,review}}.rs`; `reservist run`, `reservist replay-check` complete.
- **Behavior preserved:** keyed draws `sha256(f"{seed}|{edge}|{artifact}|{stage}")[:8]/2^64`, interpretation scores and framing biases, `round(…, 6)`, realization `round(3.1 + draw*0.8 + adj, 3)`, compression steps at ≥ 1 day gaps, publication invariants, `verdict: None`.
- **Fixtures:** all vectors; four reseeded runs.
- **Verification:** `just parity` zero unattributed divergences; `reservist replay-check` output equals oracle `cli_stdout` modulo decimal normalization; ported tests including `test_report_is_inert` via the `tooling` outlet injection.
- **Completion:** full parity.
- **Risks:** `round()` reproduction (Section 7.2); `min(1.0, max(0.0, …))` ordering; `timedelta(minutes=1)` report scheduling.
- **Commit:** `feat: Complete publication, audience, and review cycle`

### WP-10 Parity harness and attribution

- **Objective:** `reservist parity` as specified in Section 7.2 with reports and the attribution file.
- **Why here:** WP-06 to WP-09 used a provisional prefix comparator; this package makes attribution a first-class gate before registry and save work reshape internals.
- **Paths:** `crates/reservist-cli/src/parity.rs`, `tests/parity/attributions.toml`, `just parity`.
- **Verification:** `just parity` green; a deliberate mutation (change a framing bias) produces an unattributed divergence and a non-zero exit.
- **Completion:** both behaviors demonstrated and recorded.
- **Commit:** `test: Add cross-runtime parity harness`

### WP-11 Registry validation, metadata export, compiler binding checks

- **Objective:** G-01 and G-06: ambiguity check, cross-phase feedback check, `phase::metadata()` export, compiler verification that every frozen `work_kind` and transition kind resolves, Mermaid generation.
- **Inspect:** `Runtime::_schedule_dynamic_event` call sites; `owned_state_transitions.csv`.
- **Paths:** `core/src/phase/{registry,validate}.rs`, `content/src/{bindings,generate}.rs`, `catalog/generated/{handler_registry.csv,phase_flow.mmd}`.
- **Verification:** `cargo test -p reservist-core phase`; compiler negative fixtures (unknown `work_kind`, transition kind not accepted); `just catalog-generate` produces the two generated files deterministically (second run no diff).
- **Completion:** negatives rejected with categories `unknown_handler`, `unknown_transition`, `ambiguous_write`, `phase_feedback`.
- **Commit:** `feat: Register work handlers with declared phases`

### WP-12 Fidelity models and generated permission matrix

- **Objective:** sealed `FidelityModel` trait with the seven tiers, clade compatibility per D05 kinds table, generated `fidelity_permissions.csv`, three-level validation for the MVP slice.
- **Inspect:** `catalog/type_fidelity.csv` (67 rows), `catalog/types.csv`, `docs/design/05-design-discussion-representation-bible.md:935` (closed tiers).
- **Paths:** `core/src/fidelity/{mod,models,matrix}.rs`, `content/src/bindings.rs` (fidelity checks), `catalog/generated/fidelity_permissions.csv`.
- **Verification:** generated matrix equals the closure of `type_fidelity.csv` over the 28 clades for the tiers used by the slice, and every row of `type_fidelity.csv` is Rust-compatible; manifest negative fixture (select `MECHANICAL_OR_ADAPTER` for `person.us.ben_bernankey`) rejected with `fidelity_permission`; a model-required-state negative (remove `state.person.us.ben_bernankey.cognition` from initialization) rejected.
- **Completion:** matrix committed; three negatives rejected.
- **Commit:** `feat: Generate fidelity permission matrix`

### WP-13 Exact checkpoint and resume

- **Objective:** `SaveFile` v1 and the equivalence proof of Section 7.4.
- **Paths:** `core/src/save.rs`, `reservist save/resume`, `crates/reservist-core/tests/save_resume.rs`.
- **Verification:** equivalence at every boundary for all vectors; idempotent re-save; the seven `ResumeError` categories each triggered by a mutated file.
- **Completion:** test suite green; runtime of the exhaustive test under two minutes (97 boundaries × 15 vectors).
- **Risks:** serializing `Decimal` and f64 through canonical JSON must round-trip exactly (Decimals as strings, floats via shortest repr). The idempotent re-save test catches any loss.
- **Commit:** `feat: Add exact checkpoint and resume`

### WP-14 Session API and interactive play

- **Objective:** `api::Session` with `SessionOp` for the twelve verbs and seven projections; `reservist play` renders the same text as `engine/harness/*`.
- **Inspect/replace:** `engine/cli.py:110-200`, `engine/harness/*.py`.
- **Paths:** `core/src/api/{mod,session,ops,views}.rs`, `crates/reservist-cli/src/{play,render}.rs`, `crates/reservist-cli/tests/play_script.rs`.
- **Interfaces:** `Session::apply(SessionOp) -> Result<OpOutcome, Rejected>`; `Session::view(View) -> Projection`; every projection is an owned struct with `Serialize`, no references, no hidden fields.
- **Behavior preserved:** `propose` before the meeting sets the package and advances to the decision; `advance` = `advance_to_next_consequential_event`; `inspect` records `player_record_read`; `verbs` reflects blackout; text renderings identical.
- **Verification:** scripted test reproduces `test_postmortem_labels::test_interactive_advance…` expectations; gate 09 strings present; parity of the `play` stdout vector.
- **Completion:** all twelve verbs behave as in Python; rejected ops (`propose` after the decision, unknown package, `ask markets` twice) return `Rejected` and leave the state hash unchanged.
- **Commit:** `feat: Add session API and interactive play`

### WP-15 Godot client over the session boundary

- **Objective:** GDExtension `ReservistSession`, seven room scenes, twelve verbs, headless tests for G-12 and G-22, lints.
- **Paths:** `crates/reservist-godot/src/{lib,session_node}.rs`, `godot/scenes/*.tscn`, `godot/scripts/*.gd` (navigation only), `godot/tests/boundary.gd`, `godot/tests/run.gd`, `just godot-test`, `just godot-lint`.
- **Verification:** `godot --headless --path godot --script res://tests/run.gd` drives the `play` script through `submit`/`view`, asserts the review view carries "No universal verdict is assigned.", submits an invalid op and asserts the `state_hash` field of the receipt is unchanged, and asserts every `view()` dictionary lacks the forbidden keys (`hidden_conditions`, `opening_state`, `canonical_registry`, `inflation_persistence`, `repo_obligation`); `just godot-lint` passes; `just deps-check` passes for `reservist-godot`.
- **Completion:** headless test green; manual `godot --path godot` opens the Morning Book scene with portraits from `assets/headshots/`.
- **Risks:** `godot` crate API changes between 0.5.x patch versions are pinned by `=0.5.5`.
- **Commit:** `feat: Add Godot client over the session boundary`

### WP-16 Retire Python from the production path

- **Objective:** M1 closure. Move `engine/` and `tests/` to `tools/oracle/`, retarget `justfile`, update the Build Review, record M1 exit evidence.
- **Paths:** `tools/oracle/engine/**`, `tools/oracle/tests/**`, `tools/oracle/export_vectors.py`, `justfile` (`validate`, `freeze`, `run`, `replay`, `play` → `cargo run -p reservist-cli --locked --`; `oracle-test`, `oracle-vectors`), `CLAUDE.md` command list, `docs/BUILD_REVIEW.md`.
- **Verification:** `just check` green; `grep -r "python" crates godot justfile` shows only `oracle-*` recipes; `just oracle-test` 86/86 from the new location.
- **Completion:** M1 exit criteria (Section 7.6) recorded with executed labels.
- **Commit:** `chore: Retire Python from the production path`

### WP-17 (M2) Typed commands, receipts, idempotency, immutable projections

- **Objective:** replace `SessionOp` with `Command` (same variants plus `command_id`, `idempotency_key`), every command yields a `Receipt` (accepted or rejected, with state hash before/after and a witness event `player_command_received`), duplicate delivery is a no-op with the original receipt.
- **H clause:** Client boundary. **Old expectation replaced:** direct method calls and `propose` mutation without a receipt.
- **Paths:** `core/src/api/{command,receipt}.rs`, `core/src/scenario/handlers/command.rs`, Godot `submit` now returns receipts.
- **Verification:** G-12 behavioral tests (rejected command leaves state hash unchanged, receipts witnessed, idempotent redelivery); `attributions.toml` gains `authorized_difference` entries for the new `player_command_received` events; the M1 vectors still pass with those events attributed.
- **Commit:** `feat: Add typed player commands with receipts`

### WP-18 (M2) Chief of staff as a named Person

- **Objective:** catalog `Person` entry at `NAMED_COGNITION`, dated `HOLDS` relationship to `office.us.federal_reserve.chief_of_staff`, cognition opening state, cast entry, manifest selection; re-freeze; G-15 tests (holder replacement transfers duties and records, not private cognition).
- **H clause:** Chief of staff. **Old expectation replaced:** D11 Q6 office-only at `LIMITED_ROLE_HOLDER` (marked Historical).
- **Paths:** `catalog/inventory/mvp_phase7/{entities,relationships,type_fidelity,owned_state,owned_state_transitions,period_variants,entity_authority_sources,entity_fallback_contracts,profile_catalog_roles}.csv`, `scenarios/mvp_2006_cycle/{cast/fomc_2006.json,manifest.json,initialization.json}`, `scenarios/mvp_2006_cycle_m1/` (frozen M1 copy retained for the M1 vectors), `core/src/cognition/chief.rs`.
- **Verification:** `reservist freeze` yields a new hash recorded in the Build Review; `just parity --fixture m1` still green; new G-15 tests green; `catalog.py validate` green.
- **Commit:** `feat: Bind the chief of staff office to a named person`

### WP-19 (M2) Calendar board, discretionary periods, interruption queue

- **Objective:** `core::calendar` with anchors (FOMC meetings, statement times, releases, deadlines), discretionary activity periods between anchors, player-authorized advance to the next anchor or period, interruption queue with banner semantics (D3), dated capacity generalized from `CapacityBook` reservations. `IntermeetingCompressor` witnesses become calendar-span records.
- **H clause:** Temporal cadence; Ordering and invalidation. **Old expectation replaced:** `advance_to_next_consequential_event` heuristic.
- **Paths:** `core/src/calendar/{board,periods,interruptions}.rs`, `api::views::CalendarView`, Godot calendar scene.
- **Verification:** G-07 (reading delay and frame timing varied in the headless test, identical transcript), G-08 (reservations persist across advance and resume; displacement visible), D3 test (an observed emergency enqueues an interruption and never mutates an open folder's context).
- **Commit:** `feat: Add the calendar board and interruption queue`

### WP-20 (M2) Folder handoff, slate admission, practical cards

- **Objective:** `core::folder` with penciled choices, practical cards (exact section and assessment section), slate validation (authority, access, eligibility, mutual exclusion, binding), atomic admission (validation, dated reservations, enqueue with receipts together or not at all), speaking-commit lines (D2), no partial mutation on invalid slates.
- **H clause:** Commit and submit; P-01 replacement text. **Old expectation replaced:** immediate `propose`/`ask markets` mutation.
- **Paths:** `core/src/folder/{mod,slate,cards,admission}.rs`, `api::Command::{OpenFolder,Pencil,HandOff,CloseWithoutHandoff}`, Godot folder scenes.
- **Verification:** G-09 (two affordable requests oversubscribing one window are rejected as a slate with zero partial reservations), G-10 (correction creates a new transition and preserves earlier witnesses), G-21 (selected option resolves to its reviewed binding; card exact section equals immediate mechanics computed by dry-run validation without state mutation).
- **Commit:** `feat: Add folder handoff with practical cards`

### WP-21 (M2) Evidence routing conflicts and deadline options

- **Objective:** routing consults only delivered artifacts and metadata; a consequential access conflict surfaces choices; a hard deadline offers costed feasible options.
- **H clause:** Evidence routing; Evidence access and reporting friction; D10 Q3 replacement text.
- **Paths:** `core/src/routing/{mod,access,options}.rs`, staff scope data in `staff/work_2006.json`.
- **Verification:** G-13 (routing has no read path to `hidden_conditions`; the compile-time boundary plus a behavioral test), G-14 (deadline options each state delivery time, uncertainty, capacity cost, displaced work).
- **Commit:** `feat: Add evidence-bounded routing and deadline options`

### WP-22 (M2) Extradiegetic scorecard alongside the in-world review

- **Objective:** `core::stewardship::Scorecard` record produced at review acceptance with an authored verdict and signed delta from explicit findings evaluated against witnessed facts; `StaffReview` unchanged; gate 09 gains the scorecard assertion (D5). Ledger interfaces fixed for M3 (Section 9).
- **H clause:** Stewardship score; P-04, P-05 replacement text. **Old expectation replaced:** none (additive).
- **Paths:** `core/src/stewardship/{scorecard,findings,ledger_iface}.rs`, `catalog/inventory/mvp_phase7/stewardship_findings.csv` (authored findings for the MVP cycle with point values), `api::views::ScorecardView`, Godot review scene.
- **Verification:** gate 09 both assertions; `test_population_conservation` unchanged; no actor read path to the scorecard (compile-time: `stewardship` is not reachable from any handler module; behavioral: transcript unaffected by scorecard computation).
- **Commit:** `feat: Add the extradiegetic scorecard`

### WP-23 (M2) Godot surfaces for M2 and acceptance

- **Objective:** calendar, folders, cards, interruption banner, scorecard in Godot; M2 exit criteria recorded.
- **Verification:** headless tests for each; Build Review updated.
- **Commit:** `feat: Add calendar, folder, and scorecard scenes`

---

## 9. M3 and M4 runway (interfaces M1/M2 must preserve)

| Later need | Interface fixed now | Where |
|---|---|---|
| Succession (M3) | Player binding is `Runtime.player_id` read from the frozen scenario, never a constant; office tenure is `state.office.*.tenure` with `holder_id` and `effective_period`; `PlayerRecordStore` is keyed by recipient and can be re-pointed; person-owned state (beliefs, memory, person commitments) is stored under the person owner, institution-owned under the office or body | WP-06, WP-14, WP-18 |
| Review-linked immutable Stewardship ledger (M3) | `stewardship::ledger_iface::LedgerEntry { review_id, review_version, findings: Vec<FindingRef>, delta: i64, kind: Award|Adjustment }` append-only; `Scorecard` in WP-22 writes through this interface even though M2 has one review | WP-22 |
| D1 terminal event (M3) | Calendar anchors carry `kind` (e.g. `institutional_review`); the interruption queue can emit institutional events (`review_acknowledged_without_response`) without player action | WP-19 |
| Separate in-world review and extradiegetic scorecard | `StaffReview.verdict` remains `None` forever; `Scorecard` is a distinct record kind; renderers never merge them | WP-09, WP-22 |
| RECORDED/RESPONSIVE providers (M4) | `WorkHandler` metadata gains an optional `execution_mode` field in M1 (`None` for non-providers); frozen scenario records `provider_binding` as today and the compiler's `bindings` module has a `providers` slot that M4 fills | WP-11 |
| `supported_transitions` (M4) | Manifest schema version 2 is reserved; `content::manifest` reads `supported_transitions` as an optional array in version 1 and validates it only when present; empty means "none declared" and is recorded as such | WP-05 |
| Asset provenance and Godot importing (M4) | `presentation_refs.csv` remains the source; `godot/.godot/` ignored; `godot/project.godot` pins importer settings; asset ids are catalog ids plus role | WP-01, WP-15 |
| Broader world content (M4) | The compiler enforces global validity now, so new inventory domains arrive valid or fail `just check` | WP-05 |

Nothing else is built for M3 or M4 in M1/M2.

---

## 10. Continuation procedure

**First executable task:** WP-01. Concretely: create `rust-toolchain.toml` (`channel = "1.98.1"`), `.python-version` (`3.14.3`), the workspace `Cargo.toml` with the four crates, `clippy.toml`, `godot/project.godot` for 4.7, and the `just` recipes; then run:

```
cargo build --locked
cargo test --locked
cargo fmt --check
cargo clippy --all-targets -- -D warnings
just deps-check
just test
godot --headless --path godot --quit
```

All must exit 0. Commit as `chore: Add Rust workspace and toolchain pins` on a task-scoped jj change; do not move `main`.

**Build Review updates.** At the end of every work package, add or update rows in `docs/BUILD_REVIEW.md`: the "Result and runnable path" line with the commands executed and their outputs, the "Shape: intended versus as-built" table rows the package changed, the recorded hashes (scenario, Rust state hashes per package, vector file hashes), and the "Deviations and uncertainty" list. Each row is labeled inspected or executed with the jj change id. Uncommitted work in progress is described in a "Working copy" paragraph, never omitted.

**Evidence per completed package.** The commit message body names the package id; the Build Review row names the verification commands and their outputs; parity packages attach `target/parity/*.report.json` summaries (counts only) to the Build Review; any new attribution entry cites its H clause.

**Stop for an owner decision only when:** a preserved-behavior test cannot pass without changing gameplay, ownership, legal authority, evidence access, persistence semantics, approved technology, or scope (Build Mandate "Binding constraints"); an oracle vector is found to encode a defect whose correction changes an accepted contract; a catalog repair would require deleting a legal action or narrowing declared coverage; or an H clause is found to conflict with another in a way this plan did not resolve. Record the question in the Build Review "Owner dispositions" section and continue with every package that does not depend on the answer.

**Continue without asking when:** choosing names, module layout, serialization details, test organization, phase names, save format details, lint configuration, toolchain patch versions, catalog free-text field values with provenance, satirical display names in the presentation register, or the order of independent packages; fixing a Rust arithmetic path to match an oracle vector; adding negative fixtures; regenerating `catalog/generated/` after a repair.

---

## First five executable work packages (ordered checklist)

1. **WP-01 Workspace and toolchain pins.** Evidence: `cargo build/test/fmt/clippy --locked` exit 0, `just deps-check` exit 0, `just test` 86/86, `godot --headless --quit` exit 0, `rustup show` and `godot --version` outputs in the Build Review. Commit `chore: Add Rust workspace and toolchain pins`.
2. **WP-02 Repair the media inventory and catalog tooling.** Evidence: `python3 catalog/catalog.py validate` exit 0, `just catalog-test` 17/17, regenerated `catalog/generated/` with the manifest readiness line naming `manifest.mvp_2006_cycle`, `just freeze` with empty `jj diff --stat scenarios/`, scenario hash `493ca024…` unchanged. Commit `fix: Repair media inventory rows and catalog readiness report`.
3. **WP-03 Close the MVP slice catalog contract.** Evidence: `audit-eligibility` reports the 39 selected entries eligible before the flag flip, `validate` exit 0 after, `completeness.txt` before/after lines, scenario hash unchanged, `just test` 86/86. Commit `feat: Close the MVP slice catalog contract`.
4. **WP-04 Canonical JSON, hashing, and oracle vectors.** Evidence: `cargo test -p reservist-core canon` green including byte equality against every transcript line of the 15 vectors plus 4 reseeded runs; `tests/oracle/vectors/*.json` committed with their SHA-256 list; `just oracle-vectors` idempotent. Commit `feat: Add canonical JSON hashing and oracle vectors`.
5. **WP-05 Content compiler: validate and freeze.** Evidence: `reservist validate scenarios/mvp_2006_cycle` prints `validation passed: sha256:493ca024…8d066`; `reservist freeze` leaves `scenarios/` byte-identical; ported `test_manifest_closure`, `test_initialization`, `test_phase6_catalog_slice`, reseal test green; negative fixtures rejected with identical category strings; `reservist validate-catalog catalog` exit 0. Commit `feat: Add content compiler validate and freeze`.
