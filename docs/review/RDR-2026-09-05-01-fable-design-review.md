# Reservist: Fable Design Review of the v0.2 Amendment

**Document ID:** `RDR-2026-09-05-01`  
**Reviewed:** `docs/design/17-current-design-amendment-review-draft-v0_2.md` (RDA-2026-09-05-01 v0.2) and its companion `docs/design/16-agent-first-design-bible-workflow-v0_2.md` (RDW-2026-09-05-01 v0.2).  
**Decision source:** `docs/design/15-open-questions-decision-handoff.md` (H).  
**Repository revision reviewed:** jj change `oyrstukq` / git `97fc926a` ("chore: Migrate design corpus and catalog into repo"), whose parent is `main` at `8679992a`. Locators below are repo paths and line numbers at that revision.  
**Reviewer:** Claude Fable 5.1, 2026-09-05.  
**Status:** Review output. Nothing here is canon. Clause dispositions are recommendations for the design owner. No Rust/Godot implementation was started.

## How this review was conducted

The review read the amendment against three things the Section 9 instructions require: the accepted handoff (H), the compatible inherited chapters (D04, D05, D09, D10, D11, D12, R13, D14), and the actual repository. Two facts about the repository changed the review's footing and are reported first because the amendment assumes otherwise.

1. **The executable baseline is present.** `engine/` (106 Python files, Python 3.14.3, zero third-party dependencies), the frozen scenario `scenarios/mvp_2006_cycle/`, and `tests/` are in this repository. Observed execution at this revision:

   | Command | Result |
   |---|---|
   | `python3 -m unittest discover -s tests -t .` | 86 tests, OK |
   | `just gates` | 10 tests, OK |
   | `just validate` | `validation passed: sha256:493ca024…8d066` |
   | `just replay` | three packages, two runs each, state hashes and transcript bytes equal |
   | `just freeze` | reseals to the same scenario hash; no diff under `scenarios/` |
   | `just catalog-test` | 17 tests, 2 failures (pre-existing) |
   | `python3 catalog/catalog.py validate` | 214 structural issues (pre-existing) |

2. **The catalog and design chapters were outside the repository.** Until the migration commit named above, `engine/catalog_slice.py`, `tests/test_phase6_catalog_slice.py`, and the `justfile` pointed at a gitignored symlink (`.humanlayer/tasks/... -> ~/.humanlayer/riptide/artifacts/...`). `just freeze`, `just catalog-*`, `test_phase6_catalog_slice.py`, and `test_replay.py` (which calls `seal_scenario`) could not run from a clean clone. The migration copied all 198 artifacts byte-identically into `docs/design/` and `catalog/`, verified every SHA-256 against the former `reservist-omni-split/00-index.md`, and repointed the three references. D6 authorized deletion of the duplicate tree after verification. This satisfies H — Repository for the sources that exist today; it does not repair the catalog's validation state.

Where a finding rests on observed execution, the table says so. Where it rests on reading source, it says "inspected". Where it rests on reading design text, it gives the chapter and line.

## A. Overall judgment

**The adopted choices compose.** No two H clauses contradict each other. The External providers phase-feedback wording refines, and does not conflict with, Ordering and invalidation.

**Two adopted decisions overturn facts the baseline enforces or a prior chapter resolved.** The Stewardship Score reverses a test-enforced invariant (gate 09 and one unit test assert that no verdict or score exists). The chief of staff at `NAMED_COGNITION` overrides D11 resolved question 6, which chose limited role-holder cognition. Both are explicit in H, both are legitimate supersessions, and both become migration items with named tests and catalog rows. Neither is a blocker.

**Boundary clauses.** P-02 and P-04 can be accepted as drafted. P-01, P-03, P-06, and P-07 can be accepted with the replacement text in Part C. P-05 needs one owner decision (the terminal rule for an undispositioned review), for which Part C gives a recommendation.

**Citation defects.** Four attributions in the amendment's Sections 2 and 4 are wrong or loose (Part B, F-03 through F-05, F-18). None changes an adopted decision. The "ends play" rule that H supersedes exists and is located (F-05).

**Separate verdicts, as instructed:**

| Dimension | Verdict | Basis |
|---|---|---|
| Product acceptance | Not assessable yet; no Rust/Godot artifact exists | inspected |
| Static validity of content | Partly false: 214 catalog structural issues, 2 failing catalog tests, `completeness.txt` reports a manifest as unauthored while `scenarios/mvp_2006_cycle/manifest.json` exists | observed execution |
| Runtime conformance | Established for the Python baseline only, at the level its 86 tests and 10 gates assert | observed execution |
| Economic calibration | No evidence; H leaves channel budgets open | design text |
| Human comprehension | No evidence; gate 09 checks rendered strings, not players | inspected |

## B. Findings table

Classifications follow Section 9: `contradiction`, `missing interface`, `unsupported claim`, `scope ambiguity`, `migration requirement`, `calibration/content`, `already resolved`, plus `editorial` for wording-only items. Confidence is High when the locator was read and, where applicable, executed in this session.

| ID | Class | Locator | Conflicting or missing contract | Minimal correction | Affected owners / tests | Conf. |
|---|---|---|---|---|---|---|
| F-01 | unsupported claim | Amendment §2.3 ¶2: "the uploaded historical corpus contains … but not the complete engine and frozen runtime fixture" | The repo contains `engine/`, `scenarios/mvp_2006_cycle/`, `tests/` (86 tests OK, 10 gates OK). What was absent was the catalog, now migrated. | Replace ¶2 with the observed baseline: revision, commands, counts, and the catalog's former external location. | Amendment §2.3, §7.1; `docs/BUILD_REVIEW.md` | High |
| F-02 | already resolved | Amendment §4 last ¶ and §2.2 "Exact continuity": D04 versioned JSON interchange | Confirmed. `docs/design/04…kernel.md:2725` states the canonical interchange is UTF-8 JSON under a versioned JSON Schema, hashed as RFC 8785 JCS + SHA-256. `engine/canon.py` implements exactly that. | None. Record the locator. | none | High |
| F-03 | unsupported claim | Amendment §2.2 "Distinct information objects", source list cites D09 for the `PublishedReference` binder exception | D09 (`09…media.md`) contains no binder clause. The clause is D04 `04…kernel.md:317` and D05 `05…bible.md:1085` (invariant 6). | Cite D04:317 and D05 invariant 6; keep the D09 citations for event/evidence/claim/report and publication-before-behavior. | Amendment §2.2 | High |
| F-04 | editorial | Amendment §4 row "Calendar": "older weekly action-budget language" | D04 has weekly agenda turns (`04…kernel.md:2002`, `:3064`) but no action budget; the corpus audit states no numeric weekly action limit exists. | Reword to "D04's elastic-calendar table with weekly agenda turns and live intermeeting simulation". | Amendment §4 | High |
| F-05 | already resolved | Amendment §4 after the table: "The exact historical Bible locator for the old ends-play rule is not supplied by H." | The rule exists: D04 resolved question 16, `04…kernel.md:3167-3172`: "Resignation, death, final removal, or term expiry ends the playable role. The game does not switch the player to an omniscient Fed or successor." It is in D04, not D05. | Supply the locator. Note that D04:3169's first sentence (regime transitions continue while the Chair retains agency) is compatible and survives. | Amendment §4; D04 Q16 becomes Historical | High |
| F-06 | migration requirement | H — Stewardship score vs `tests/acceptance/test_mvp_gates.py:170` (gate 09 requires the literal "No universal verdict is assigned."), `tests/test_population_conservation.py:41` (asserts no `economy_score`), `engine/postmortem.py:100` (`"verdict": None`), `engine/harness/wire.py:36` (anti-score copy) | The baseline enforces D04's "no single victory score" (`04…kernel.md:2175`), which H supersedes. | Repair within the design, not by deleting the tests: the in-world `StaffReview` keeps "no universal verdict" (D10 Q11 still governs it); the extradiegetic scorecard is a separate object carrying the authored verdict and signed delta. Gate 09 keeps its in-world assertion and gains a scorecard assertion. `test_population_conservation` stays as written (the score is not a population field). Owner confirms this split (decision D5). | D04 Term/failure/legacy; D10 Q11; gate 09; `postmortem.py`; `wire.py` | High |
| F-07 | migration requirement | H — Chief of staff vs D11 resolved Q6 (`11…mvp-slice.md:507-510`: "the chief of staff use limited role-holder cognition"); `scenarios/mvp_2006_cycle/manifest.json` selects only the chief-of-staff office at `LIMITED_ROLE_HOLDER`; `grep -ri chief engine scenarios` returns nothing | H is an explicit override. The catalog has no chief-of-staff `Person` entry and no `OfficeHolding` row binding one. | Add a `Person` entry at `NAMED_COGNITION`, a dated `OfficeHolding` to the existing office, and required cognition state; classify under P-06 as an authorized change with H — Chief of staff as its clause. D11 Q6 becomes Historical for this cast member only. | D05 person/office; D06/catalog `entities.csv`, `relationships.csv`, `type_fidelity.csv`; manifest; G-15 | High |
| F-08 | migration requirement | `catalog/catalog.py validate` → 214 structural issues; `catalog/test_catalog.py` → 2 failures; D12 leaves `just catalog-test` unchecked (`12…cycle.md:183`) | H — Repository is now satisfied for location. H — Validation and fallback ("invalid catalog structure fails catalog validation") is not satisfied by the global catalog. | Treat as M1 conformance work under §7.6's constraint (no flag-flipping, no deleting legal actions). The frozen MVP slice validates today; the global catalog does not. | D06; `catalog/`; G-01, G-03 | High |
| F-09 | missing interface | P-03 "Review still required": phase roster and resolution period. `engine/clock.py:20-32`: ordering key is (time, integer `phase_priority`, sequence, id). Priorities in use: 10 book open, 20 release, 25 staff completion, 30 checkpoint, 35 repo, 40 FOMC, 50 statement, 55 media, 60 reception, 70 orders, 80 commitment expiry, 90 next book | The baseline has a de facto forward order but no named phases and no declared reads/writes. | Builder latitude: derive a named roster from these priorities (Part C, P-03 text). Owner decides only the open-folder interruption protocol (D3). | D04 scheduling; G-06 | High |
| F-10 | scope ambiguity | P-06 and G-19: checkpoint/resume. `engine/` has no save/load; replay is re-run from `initialization.json` and comparison of state hash and transcript bytes (`engine/cli.py:74`, gate 07) | The oracle cannot supply checkpoint/resume vectors. | Classify exact checkpoint/resume as a new-runtime guarantee. The oracle supplies replay identity, event order, receipts, and conserved stocks only. | P-06; G-18, G-19 | High |
| F-11 | missing interface | H — Client boundary vs `engine/cli.py:182` (`runtime.package_id = package` mutates the runtime from the REPL); player verbs are direct method calls on `ScenarioRuntime`; `tests/test_presentation_boundary.py` is a substring check for `display_name`/`species`/`portrait_path` | No typed command objects, receipts for every command, or projection boundary exist. The presentation-boundary test does not exercise behavior. | Classify the command/projection boundary as a new-runtime guarantee. G-12 evidence must be behavioral (rejected command leaves the state hash unchanged; projection contains no canonical reference), not a token scan. | D04 client; G-07, G-12 | High |
| F-12 | missing interface | H — External providers vs `engine/adapters/treasury_demand.py:56` (single stub returning `ADAPTER_NO_PRICE_FORMATION`), `provider_binding` an unenforced manifest string (`engine/manifest.py:60-70`), gate 10 forbids network imports (`test_mvp_gates.py:178`) | No RECORDED/RESPONSIVE distinction or channel traits exist. | New-runtime guarantee under M4 (Part G). Preserve gate 10: providers are process-local; no network dependency inside the game bounds. | D06 external channels; G-20, G-22 | High |
| F-13 | already resolved | H — Runtime dispatch example `FailedToClear` vs `engine/markets/treasury_secondary.py:20` `FAILED_TO_CONVERGE` | Naming only; the typed-outcome contract is already honored (two-phase settlement also has `FAILED_PREPARE`/`FAILED_COMMIT`). | None. Builder chooses names. | none | High |
| F-14 | scope ambiguity | Amendment §4 rows citing "D14 question N recommended …" | D14's own "Resolved Design Questions" section (`14…architecture.md:611-613`) says "None yet. All questions above are open for review." | Keep the rows; word them as "recommendation superseded", never "decision superseded". | Amendment §4; D14 status | High |
| F-15 | contradiction (minor) | D10 `10…interface.md:544`: a commodity restriction "must become a briefing bullet when it materially affects an active Fed-facing channel" vs H — Evidence routing ("staff may miss connections … routing does not receive … canonical importance") | The D10 sentence, read alone, guarantees delivery on canonical materiality. | Replacement text for D10 Q3: "should ordinarily reach the briefing through institutional routing when the institution possesses the observation and a monitoring path exists; routing may fail for institutional reasons and never consults canonical materiality." | D10 Q3; G-13 | High |
| F-16 | already resolved | Amendment §4 row "Alias machinery" vs D04 `04…kernel.md:2844`: "Alias cleanup is mandatory … Historical IDs may survive only as import aliases that cannot be selected, own state, or enter hashes." | Compatible with H — Identity aliases (no general system; source-specific label only when an import workflow requires it). | None. Record D04:2844 as the surviving rule. | none | High |
| F-17 | already resolved | G-08 and workflow §3 example vs `engine/staff/capacity.py` (`CapacityBook.reserve/release`, `Deliverable.displace`, statuses SCHEDULED/DISPLACED/MISSED/DELIVERED) and `engine/commitments.py` (dated commitments with `reserved_units`, release on expiry/breach) | Dated, owned, releasable capacity already exists and is tested (`tests/test_displaced_work.py`, `test_request_lifecycle.py`, `test_commitment_carryover.py`). | Classify under P-06 as preserved behavior. The only new part of G-08 is persistence across resume. | G-08 | High |
| F-18 | editorial | Amendment §2.2 cites "D05 — final representation invariants" | The heading is `## Representation Invariants` (`05…bible.md:1076`, 27 items). | Use the actual heading. | Amendment §2.2, §10 | High |
| F-19 | migration requirement | `catalog/catalog.py:837`: hard-coded literal "scenario representation manifest remains unauthored" written to `catalog/generated/completeness.txt`, while `scenarios/mvp_2006_cycle/manifest.json` exists and validates | A generator emits a false readiness statement. | Make the line conditional on manifest discovery or remove it; regenerate. Already listed in amendment §7.6; the locator there ("bundle 04, source window 884–925") should become `catalog/catalog.py:837`. | catalog generator; §7.6 | High |
| F-20 | migration requirement | P-06 oracle pin | The oracle must be pinned to something reproducible: Python 3.14.3, revision `97fc926a` (post-migration, so freeze works), scenario hash `sha256:493ca024…8d066`, the three package transcripts from `just replay`. | Record these in `docs/BUILD_MANDATE.md`; add a `pyproject.toml` or `.python-version` pin as M1 work (no runtime change). | P-06; G-18 | High |
| F-21 | scope ambiguity | Amendment §3 replaced by a pointer in the checked-in copy | The v0.2 file in this repo points to `15-open-questions-decision-handoff.md` instead of duplicating H. The pointed-to file is byte-identical to the owner's handoff. | None; noted so no reader believes Section 3 was dropped. | none | High |

Nothing in the table proposes reopening the engine choice, the headline score, the no-runtime-model rule, or the calendar style.

## C. Clause dispositions

Each disposition is a recommendation. Replacement paragraphs are exact text the integration agent can paste into the owning chapter once the owner accepts.

### P-01 — accept with replacement text

Replace the second sentence of the first proposed paragraph with:

> Admission means three things happen together or not at all: the slate passes Rust validation (authority, access, eligibility, mutual exclusion, and binding resolution), every capacity the slate consumes is reserved as a dated commitment, and every command the slate authorizes is enqueued with its receipt. Nothing downstream of the queue is part of admission.

Add after the third proposed paragraph:

> A call, interview, or meeting is itself a folder. Authored lines that only speak are penciled like any other choice. An authored line that asks a question, makes an inquiry, or discloses something commits when the player selects it and the turn passes to the counterpart, because the counterpart's answer is a delivery that cannot be un-heard. The practical card for such a line marks it "commits on speaking". A folder may therefore contain several commit points; each is witnessed separately. [Owner decision D2 confirms this rule.]

Rationale: H — Commit and submit forbids a penciled question secretly executing an inquiry, and forbids extra confirmation layers. Marking speaking-commits on the card satisfies both.

### P-02 — accept as drafted

Builder latitude covers queued-work versioning and cancellation. No owner input needed.

### P-03 — accept with replacement text

Replace "Select the concrete phase roster and define the internal resolution period" in "Review still required" with:

> The resolution period is one calendar timestamp's worth of due work. Within it the phase roster is fixed and forward. The builder derives the roster from the baseline's integer priorities and names it; the following is the expected shape, not a mandate on names: `OPEN` (books and calendars open), `RELEASE` (scheduled observations and external tape), `STAFF` (task completion and deliverables), `AUTHORITY` (bodies vote, delegations, legal transitions), `EXECUTION` (desk and facility commands), `CLEARING`, `SETTLEMENT`, `PUBLICATION` (claims, statements, reports), `RECEPTION` (audience and participant belief revision, orders intended for the next clearing), `COMMITMENT` (expiry, breach, release), `REVIEW` (postmortems, next book). Each handler declares its phase, reads, and writes; two handlers writing one state in one phase is a compile error.

Add:

> An institutionally observed emergency never rewrites an open folder's decision context. It enqueues an interruption that presents itself after the current handoff or before the next folder opens, with a banner in any still-open folder saying new material exists. The player may hand off, or close without handoff, and open the interruption. [Owner decision D3 confirms this rule.]

### P-04 — accept as drafted

The evidence-grounded eligibility interpretation is the only one consistent with H — Retrospective conclusions. Disputed attribution stays a finding with confidence and dissent attached and awards nothing until a later review establishes it; exoneration is an explicit upward compensating entry.

### P-05 — accepted by owner decision D1

Terminal rule:

> A completed review that remains undispositioned when the next scheduled institutional review anchor of the same kind arrives is registered as "acknowledged without response" by the office that commissioned it. That registration is a witnessed institutional event, not a player action. The review's findings are evaluated and its delta posts at that moment. The Chair may still attach a response or commission a supplemental review afterward; those create new entries, never edits. Score-ledger ownership is `reservist-core`; the Godot client renders it and can neither compute nor persist it.

Why this and not a fixed timeout or automatic acceptance: it uses an institutional anchor H already requires the calendar to have, it costs nothing to a player who dispositions normally, and it cannot be gamed by avoidance because the anchor arrives regardless.

### P-06 — accept with replacement text

Replace "State the exact replacement acceptance scope" with:

> The replacement scope is the executable behavior of `engine/cli.py` at the pinned revision: `validate`, `freeze`, `run` (with transcript and endogeneity report), `replay-check`, and `play` with its twelve REPL verbs, across the three policy packages, plus every assertion in the 86 tests and 10 gates. Preserved behavior: conservation, ownership, authority stages, receipt chains, event order, categorical outcomes, and the delivered-record boundary. Authorized changes, each citing its H clause: chief of staff as a named Person (H — Chief of staff), calendar board and discretionary periods (H — Temporal cadence), folder handoff (H — Commit and submit), typed commands and projections (H — Client boundary), extradiegetic scorecard alongside the in-world review (H — Stewardship score). New-runtime guarantees: checkpoint/resume, static registration, generated permission matrix, compiler fail-closed loading. Milestones: M1 parity and new-runtime guarantees; M2 the authorized changes above; M3 succession and the full score ledger; M4 channel providers and world content. [Owner decision D4 confirms M3 ordering.]

### P-07 — accept with replacement text

Replace "Specify the bounded authoring and validation mechanism" with:

> The scenario manifest declares `supported_transitions`: the legal changes, interventions, office transitions, and provider interventions that campaign play may reach. The content compiler checks each declared transition against the registered handlers, provider execution modes, and fidelity permissions. A `RECORDED` provider on a channel named by any supported intervention is a compile error. Transitions not declared are not lawful refusals; they are outside the fixture's declared scope and the compiler records them as such. No reachability solver is built; the declared list is authored and reviewed.

## D. Accepted-source fidelity

Section 3 of the v0.2 amendment reproduced H faithfully. A heading-by-heading comparison against `15-open-questions-decision-handoff.md` found every `###` topic and the two decision-bearing `##` sections ("External providers", "Still open or deferred") present in order, and every quoted bullet identical. The checked-in copy replaces the reproduction with a pointer to the byte-identical file (F-21).

The deliberate commitments are preserved: headline score, named chief, full one-way rewrite, no runtime language model, three eligibility concepts. The only fidelity defects are the four attribution errors in Sections 2 and 4 (F-03, F-04, F-05, F-18), none of which touches Section 3.

## E. Propagation plan

What must change together when the owner accepts the dispositions. Paths are post-migration.

| Change set | Owning text | Content / schema | Tests and generated outputs |
|---|---|---|---|
| Score alongside in-world review (F-06, P-04, P-05) | D04 `04…kernel.md:2160-2180` (Term, failure, and legacy: replace the no-score sentences, keep the dossier); D10 Q11 unchanged | Catalog: `Record` subtype for scorecard entries; authored findings table | Gate 09 gains a scorecard assertion; `postmortem.py`, `wire.py` copy; G-17 cases |
| Chief of staff as Person (F-07) | D11 Q6 marked Historical for this member; D05 person/office unchanged | `catalog/entities.csv`, `relationships.csv` (OfficeHolding), `type_fidelity.csv`; `scenarios/mvp_2006_cycle/cast` and `manifest.json` | `test_manifest_closure.py`; G-15 cases; re-freeze changes scenario hash (record it) |
| Calendar and phases (F-09, P-03) | D04 `04…kernel.md:267` (Hybrid task graph and cadence) and `:3062` (Q4) replaced by H — Temporal cadence and the roster | none until Rust registration exists | G-06, G-07 |
| Client boundary (F-11) | D14 Q2 marked recommendation superseded | none | G-12 behavioral tests replace the token scan (keep the scan as a lint) |
| Providers (F-12, P-07) | D06 external channels; D14 unchanged | manifest `supported_transitions`; `external_channel_providers.csv` execution mode column | gate 10 preserved; G-20 |
| Catalog validity (F-08, F-19) | D06 | 214 issues; `catalog.py:837` | `test_catalog.py` 2 failures; regenerate `generated/` |
| Citations (F-03, F-04, F-05, F-14, F-18) | Amendment v0.3 only | none | none |
| D10 Q3 wording (F-15) | `10…interface.md:544` | none | G-13 |

Do not edit D12 or R13; they remain reports of what Python did.

## F. Remaining register

Owner decisions D1 through D6 were accepted as recommended on 2026-09-05. Everything else in amendment §8.1 is closed by Part C or delegated to the builder.

Mechanism, content, and research obligations from H — Still open or deferred remain as stated: calibration budgets per channel, provider state schemas and parameters, period-specific counterparties and accounts, opening-state research, transfer-learning playtest evidence, and score point bands.

## G. Agent-build readiness

**Readiness.** D1–D5 are dispositioned. The accepted baseline plus this review's replacement text gives an implementing agent enough authority for M1 and M2 without further design review. The builder can decide: module layout, type names, the named phase roster, versioning of queued work, receipt and idempotency representation, serialization formats (versioned, canonical), test organization, and the packaging of registry metadata for the content compiler.

**Actual blockers (not design questions):**

- No Rust toolchain, Godot, or `godot-rust` is present in the repo or referenced by any file. M1 starts with establishing them and pinning versions.
- 214 catalog structural issues and 2 failing catalog tests. M1 must reach a validating global catalog or a declared, compiler-enforced quarantine of out-of-slice rows. Flag-flipping is forbidden by §7.6.

### Owner dispositions

| ID | Decision | Disposition | Consequence |
|---|---|---|---|
| D1 | P-05 terminal rule for an undispositioned review | Register "acknowledged without response" at the next same-kind review anchor; delta posts then (Part C, P-05) | Governs M3 score ledger |
| D2 | P-01: do inquiry/disclosure lines in calls commit on speaking? | Yes, marked on the practical card. Regret after speaking is a consequence, unlike pre-handoff meeting deliberation. | Governs M2 folder handoff for calls |
| D3 | P-03: can an observed emergency change an open folder's decision context? | No; it enqueues an interruption and banners the open folder | Governs M2 calendar |
| D4 | P-06 milestone order | M1 parity → M2 adopted changes → M3 succession and score → M4 providers and world | Governs mandate order |
| D5 | F-06: keep gate 09's in-world "no universal verdict" and add a separate extradiegetic scorecard assertion | Yes | Governs M1 test porting and M3 |
| D6 | Delete `reservist-omni-split/` now that all 198 artifacts are restored at their original paths | Delete; `00-index.md` hashes are recorded in this review's migration note | Removes duplicate tree |

### Build Mandate

Published separately as the authorized `docs/BUILD_MANDATE.md`. Summary:

- **Baseline pin:** revision `97fc926a`; Python 3.14.3; scenario hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066`; the three `just replay` transcripts as oracle vectors.
- **Target:** M1 complete one-way rewrite of the executable scope named in P-06 into `reservist-core`, `reservist-content`, `reservist-godot`, with the Python oracle in tests only; then M2. M3 and M4 are authorized in principle and scheduled after M2 acceptance.
- **Binding constraints:** H in full; D04 causal contracts, D05 invariants, D09/D10 information boundaries as inherited; gate 10's no-network rule; no GDScript causal state.
- **Latitude:** the builder-decidable list above.
- **Evidence:** G-01 through G-24, with M1 owning G-01, G-04 through G-07, G-10 through G-12, G-18, G-19, G-22; M2 owning G-08, G-09, G-13 through G-15, G-21; M3 owning G-16, G-17; M4 owning G-02, G-03 (full matrix), G-20, G-23; G-24 is research.
- **Continuation:** `docs/BUILD_REVIEW.md` is the as-built record; jj commits carry the working state; uncommitted work is described, never hidden.

## Limits of this review

The review read every locator it cites and executed the commands it reports. It did not read all 14 chapters end to end; it read the sections the amendment cites plus the sections found by search for the contested claims. It did not run the catalog generator (`catalog.py generate`), which writes files. It did not evaluate economic plausibility of the MVP fixture or any player's comprehension. Two models agreeing would not have made any finding above true; the locators are there so a reader can check them.
