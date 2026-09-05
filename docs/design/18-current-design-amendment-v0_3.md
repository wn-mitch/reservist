# Reservist: Current Design Amendment and Implementation Reconciliation

**Document ID:** `RDA-2026-09-05-01`  
**Version:** `0.3-fable-reviewed`  
**Prepared:** September 5, 2026  
**Document status:** Reviewed draft awaiting owner disposition. The user decisions referenced in Section 3 are adopted; the Section 5 proposals and the reviewer's replacement text are not.  
**Review applied:** `docs/review/RDR-2026-09-05-01-fable-design-review.md` (findings F-01 through F-21).  
**Intended use:** HumanLayer design review, followed by owner disposition, agent-performed consolidation, and a broadly authorized implementation handoff.  
**Implementation status:** This document records requirements and migration obligations. It does not report a completed Rust/Godot implementation, repository edit, or newly executed acceptance test.

> Version 0.3 applies the Fable review's corrections to v0.2 (`17-current-design-amendment-review-draft-v0_2.md`, retained as history). Changes: §2.3 baseline replaced with the observed repository state (F-01); §2.2 citations corrected (F-03, F-18); §4 rows Calendar, Chairmanship endpoint, and the D14 rows reworded (F-04, F-05, F-14); §5 gains the reviewer's replacement text under each clause, marked as proposal; §7.6 locator updated (F-19); §10 records H's path. No clause is promoted to Canonical.

## 1. Purpose, authority, and review boundary

This amendment carries the user’s **Open Questions Decision Handoff** into the existing Reservist corpus. It closes architecture choices, records deliberate supersessions, and identifies the remaining interfaces that require precise acceptance rules. It is not a replacement for the detailed Representation Bible, economic contracts, legal research, catalog content, or original evidence.

The controlling source for the new decisions is the user handoff, reproduced in Section 3 with heading depth and paragraph spacing normalized. Its requirements have not been merged with reviewer recommendations. References of the form **H — heading** identify that source. **D04**, **D05**, and similar references resolve to the original artifact paths in Section 10; they are not bundle numbers.

Authority is scoped, not determined by repetition or document date alone:

- **Adopted handoff decisions** govern the subjects they explicitly decide or supersede.
- **Compatible inherited contracts** retain their existing authority: D04 for causal interfaces, D05 for representation, D06 for catalog contracts and roster commitments, and D09–D11 for compatible media, interface, and slice decisions.
- **Implementation reports and audits** describe particular artifacts or revisions. They neither override the adopted design nor prove that a newly adopted requirement is implemented.
- **Proposed clarifications, generated views, and this review draft’s editorial organization** acquire no additional authority through inclusion here.

A reviewer must identify a concrete contradiction, missing interface, unsupported assertion, or implementation consequence before proposing a change to an adopted decision. A preference for Python, another engine, a general task graph, a universal actor, continuous time, no headline score, or runtime language generation is not itself a defect in this design.

A genuine incompatibility in adopted requirements must still be reported. The instruction not to reopen settled choices is not permission to conceal contradictions. State the incompatible clauses, demonstrate the conflict, and distinguish a repair within the chosen design from a proposed change to the choice itself.

### Status vocabulary

| Status | Meaning in this packet |
|---|---|
| **Canonical** | An adopted design commitment, including the user decisions in Section 3. This does not mean implemented or validated. |
| **Superseded** | A prior decision or proposal replaced in the specified scope. Unaffected parts of the old artifact remain usable. |
| **Tentative** | A favored formulation awaiting explicit acceptance, including the proposed wording in Section 5. |
| **Open** | A decision or interface detail not yet resolved. |
| **Contradictory** | Incompatible operative-looking requirements or artifacts remain unreconciled. |
| **Derived** | A generated view, compilation product, or analytical conclusion; not an independent design authority. |
| **Historical** | A prior state, rationale, implementation attestation, or rejected alternative retained for provenance. |

Keep implementation and proof status separate from these labels. An adopted interface can be canonical and unimplemented; an implemented behavior can be historical or nonconformant; a generated report can be fresh but answer the wrong acceptance question.

### What this review should produce

Review the fidelity of the amendment, the interactions among its decisions, the proposed boundary clauses, and the source-integration obligations. Return exact findings and replacement passages using the review instructions in Section 9. Do not start an implementation, rewrite the repository, or promote proposed clauses to accepted decisions merely because they appear in a review response.

The companion document, `Reservist_Agent_First_Design_Bible_Workflow.md`, replaces the earlier living-bible workflow proposal. It is a **development-workflow proposal**, not additional game canon. Its operating premise is recorded below; its concrete process remains subject to review.

### Agent-first operating premise and review responsibility

**Additional user-supplied project constraint:** Reservist is a vibe-code-first project. The human design owner expects to review design documents, implementation shape, and playable behavior, not routinely inspect source code or maintain implementation bookkeeping. After review and disposition, the intended handoff is a broadly authorized agent build against the current bible.

This changes the organization of the work, not the adopted game mechanics in Section 3. Integration and implementation agents must own source retrieval, dependency planning, code and content edits, test execution, debugging, source-document reconciliation, and continuation records. Verification must inspect actual source and execution evidence and return a design-level account of what was implemented. A builder's narrative alone is not evidence of conformance.

The review should distinguish material owner decisions from ordinary delegated engineering choices. Do not turn every missing implementation detail into a required human approval, require the human to author each subtask, or make routine source-diff review the final correctness safeguard. Do not silently amend accepted design merely to keep implementation moving. Batch genuine design conflicts, describe their consequences, and identify independent work that can proceed.

After owner disposition, agents should integrate accepted changes into the current source chapters and produce a build-ready reading package and execution mandate. Historical artifacts remain available to the agents; the human should not have to reconstruct precedence or apply a stack of amendments mentally. This document alone does not authorize that later implementation: the present task remains review.

The methodology is experimental. The review should assess whether the proposed documentation and evidence surfaces support the intended delegation, rather than claiming that a particular model, a green test total, or multiple model approvals guarantee a correct build.

**Revision note:** Version 0.2 adds this operating premise and corresponding review/completion guidance. Section 3's accepted handoff, P-01 through P-07, and G-01 through G-24 are unchanged. The v0.1 packet remains a historical draft, not a second current review target.

## 2. Current design and inherited constraints

### 2.1 Product model after the handoff

Reservist is a turn-based institutional crisis-management simulator centered on the Federal Reserve Chair’s office. The player works through decision packets, slide decks, bounded conversations, staff requests, proposals, communications, and commitments. A calendar board presents dated institutional anchors and discretionary opportunities. The world advances only through player-authorized resolution, while internal obligations and observations retain their own dates and ordering.

The player’s difficulty comes from judgment, incomplete evidence, institutional capacity, and other actors’ decisions—not from wall-clock pressure, reading speed, routine access administration, or a universal action-point pool. The player can inspect delivered material without spending capacity. Effectful choices remain penciled until the relevant folder is handed off; their subsequent consequences follow the represented institutions and mechanisms.

The simulation is a pure Rust core, connected to Godot through a thin in-process bridge. Authoring data configures compiled mechanics through stable contracts. The content compiler produces a validated frozen scenario, while static runtime registration resolves handler keys. Neither Godot nor GDScript owns economic behavior or canonical persistence.

Campaign play continues across valid Chair successions. Institutional state and official commitments survive; private cognition and personal relationships do not transfer. The campaign receives a signed, unbounded Stewardship headline total, backed by an inspectable dossier and immutable review-linked entries. The number is extradiegetic and never becomes an in-world resource, reputation, authority, or actor input.

**Source:** H — Temporal cadence; Runtime selection; Client boundary; Commit and submit; Chair succession and campaign bounds; Stewardship score.

### 2.2 Constraints preserved from the earlier corpus

The amendment does not replace the following compatible boundaries:

**Owned state and typed mutation.** Canonical material, institutional, and cognitive facts have declared owners and accepted transitions. Conserved transfers reconcile through transactions or explicitly modeled sources, sinks, production, losses, revaluations, or discrepancies. A report or interface label cannot bypass this boundary. **Source:** D04 — Foundational causal contracts; D05 — Representation Invariants.

**Distinct stages of causality.** Proposal, authorization, execution, counterparty take-up, clearing, settlement, observation, and downstream effects remain separately owned and witnessed. A visible principal does not absorb execution performed by others. **Source:** D04 — Foundational causal contracts; Treasury basis-trade proof.

**Distinct information objects.** Canonical events, observations, deliveries, beliefs, claims, reports, and staff assessments remain different objects. Publication does not establish attention, agreement, action, or truth. The narrow binding-publication exception belongs to an authorized `PublishedReference` with declared binders, not to ordinary claims or caches. **Source:** D04 — Canonical state, derived caches, and stable references (`04…kernel.md:317`); D05 — Representation Invariants, item 6; D09 — Keep event, evidence, claim, and report distinct; Make publication change exposure before behavior.

**Representation rather than a universal actor.** The twenty-eight representation kinds and closed fidelity tiers remain the baseline. Institutions operate through people and procedure; sovereign containers, consolidated views, regions, Pop lenses, product families, and instrument families do not acquire independent ownership merely for convenient presentation. The detailed kinds table remains owned by D05 and is not replaced with a shortened table here. **Source:** D05 — Running table: representation kinds and required state; Representation Invariants; H — Fidelity traits.

**Exact continuity.** Behavior-affecting state must persist or rebuild exactly from persisted inputs. Stable identities, private cognition, pending obligations, evidence access, and committed history are not reconstructed approximately on load. The handoff relaxes Python-to-Rust byte equality, not this continuation obligation. **Source:** D04 — Persistence and deterministic identity; H — Rewrite and proof.

**Non-causal presentation.** Presentation may render delivered evidence and authored content, but cannot manufacture state, authority, history, or additional knowledge. The animal-satire constraints and prohibition on presentation-driven causal promotion remain intact. **Source:** D05 — Representation Invariants; D09 — Keep presentation metadata causally inert; H — Identity and presentation; Language models.

**Uneven implementation depth.** A named catalog subject is not evidence of a working mechanic. Broad world content and the full financial-network proof do not become implemented because a port is selected. Conversely, existing executable behavior is not disposable merely because it belongs to a small prototype. **Source:** D04 — Intentionally uneven design resolution; D11 — What we’re not doing; H — Rewrite and proof.

### 2.3 Evidence baseline

D12 and R13 report a working Python meeting-to-meeting FOMC cycle. The earlier corpus audit reports mismatches among catalog fidelity, eligibility, source identities, coverage declarations, and the reported frozen scenario. Those findings are migration inputs, not newly verified facts about an unseen current repository revision.

The executable baseline is present in this repository and was exercised during the Fable review at revision `97fc926a` (Python 3.14.3, zero third-party dependencies): `engine/`, `scenarios/mvp_2006_cycle/`, and `tests/`. Observed: 86 tests pass; the 10 acceptance gates pass; `just replay` reproduces identical state hashes and transcript bytes for all three packages; `just validate` and `just freeze` agree on scenario hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066`. The catalog (`catalog/`) and the design chapters (`docs/design/`) were migrated into the repository at that revision from a gitignored external symlink; all 198 artifacts match the SHA-256 values in the original index. The global catalog does not validate: `catalog.py validate` reports 214 structural issues and `catalog/test_catalog.py` has 2 failing tests, both pre-existing. No Rust implementation exists. The as-built inventory is `docs/BUILD_REVIEW.md`.

**Sources:** observed execution recorded in RDR-2026-09-05-01; D12 — completed MVP phases and verification; R13 — runtime, catalog-freezing, testing, and actor-layer findings. The earlier `Reservist_Corpus_Audit_*` artifacts were never present on disk and are not cited further.

## 3. Adopted decision source

**Authority: Canonical user decisions.** The following is the supplied handoff, preserved as the review baseline. Its presentation references to other games describe intended experience, not requirements to reproduce those games’ engines. No dependency versions, economic coefficients, phase enumeration, or unprovided legal facts are inferred from those references.

> Note: the handoff text is not duplicated here. The authoritative copy is `15-open-questions-decision-handoff.md` in this directory, which is byte-identical to the file the owner supplied. Section-level references of the form **H — heading** resolve to that file's `###` headings (and the `##` headings "External providers" and "Still open or deferred", which carry decisions directly).

## 4. Supersession and integration ledger

This ledger describes the effect of H. It does not convert the whole earlier corpus into historical material. Supersession applies to the conflicting proposition, not every paragraph in its source artifact.

| Subject | Earlier source or formulation | Current governing decision | Integration consequence |
|---|---|---|---|
| Runtime | D14 question 1 recommended (not resolved) retaining Python and pinning language-neutral byte identity. | H — Runtime selection; Rewrite and proof. | Godot plus pure Rust is selected. Python becomes a temporary behavioral oracle and leaves production after parity. |
| Cross-language replay | D14 recommended cross-language canonical byte conformance; D04 (`04…kernel.md:2725`) fixes canonical JSON (RFC 8785) + SHA-256 as the interchange and remains inherited. | H — Rewrite and proof. | Compare semantic invariants exactly and calibrated values under declared tolerances. Preserve deterministic identity inside the new runtime. |
| General scheduler | D04 — Hybrid task graph and cadence; D14 question 3. | H — Ordering and invalidation; Runtime dispatch. | Use fixed forward phases and static registration. Do not build a general reactive task-graph engine. |
| Calendar | D04’s elastic-calendar table with weekly agenda turns and live intermeeting simulation (`04…kernel.md:2002`, `:3064`). No source defines a numeric action budget. | H — Temporal cadence. | No autonomous real-time progression or universal points. Dated internal events resolve inside player-authorized advances. |
| Client connection | D14 question 2 recommended a separate process and JSON IPC protocol. | H — Client boundary. | One process with command/projection isolation; serialization may support replay without implying IPC. |
| Repository | Catalog stored outside the runtime repository; D14 question 5 left alternatives open. | H — Repository. | Authoritative sources, runtime, client, tests, and assets belong to one version-controlled repository. |
| Fidelity permissions | Hand-maintained catalog/type permissions that drifted from instance selections. | H — Fidelity traits. | Rust compatibility, catalog narrowing, and scenario choice are distinct, validated levels. Generate the permission matrix. |
| Chief of staff | D11 resolved question 6 selected limited role-holder cognition; current catalog supplied an office-only representation. | H — Chief of staff. | Introduce the bound persistent Person at named cognition. Keep office powers and person-owned state separate. |
| Chairmanship endpoint | D04 resolved question 16 (`04…kernel.md:3167-3172`): “Resignation, death, final removal, or term expiry ends the playable role. The game does not switch the player to an omniscient Fed or successor.” Its first clause (regime transitions continue while the Chair retains agency) survives. | H — Chair succession and campaign bounds. | Control follows legal succession until the campaign endpoint. Preserve institutional burdens without transferring private cognition. |
| Headline evaluation | D04 — Term, failure, and legacy (`04…kernel.md:2160-2180`) prohibited an overall grade or single victory score; the Python baseline enforces this in gate 09 and `tests/test_population_conservation.py:41`. | H — Stewardship score. | Add the explicit signed headline ledger. Preserve dossier depth and separation from all causal variables. |
| Choice construction | Earlier fragment-based communication and unconsolidated selection surfaces. | H — Commit and submit; Authored options and mechanical previews. | Authored complete lines bind to typed effects. Folder handoff is the commitment point. |
| Omniscient synthesis | Any reading of D10’s “material developments must appear” as a canonical-significance guarantee. | H — Evidence routing. | Staff synthesize from accessible institutional information and may miss or misprioritize connections. No hidden truth-based routing. |
| Access friction | Routine access questions left as individual player-facing work. | H — Evidence access and reporting friction. | Routine purpose-bound authorization is handled automatically; consequential conflicts expose bounded feasible alternatives. |
| External interface | A universal provider abstraction or a recorded adapter valid only by checking the opening moment. | H — External providers. | Use narrow channel traits, declared state and responses, and recorded-provider validity across supported player-reachable inputs. |
| Runtime language models | D04 permitted display-only model prose; D14 question 9 proposed a speculative renderer seam. | H — Language models. | No runtime model integration or speculative model-renderer boundary. Offline authoring remains permitted. |
| Asset cooker | D14 question 7 favored a custom content-addressed cooker. | H — Asset pipeline. | Use the Godot importer, tracked source provenance and metadata, and separate presentation identity. |
| Alias machinery | D04 (`04…kernel.md:2844`) mandates alias cleanup with import-only aliases that cannot be selected, own state, or enter hashes. Compatible; retained. | H — Identity aliases. | Repair canonical references directly in pass one; no general reconciliation system. Legal identity transitions remain explicit. |

The old ends-play rule is D04 resolved question 16, quoted in the table. Its supersession is explicit and binding and applies only to the ends-the-playable-role sentence; the compatible person/office continuity rules and the regime-transition sentence survive. D14's nine items were recommendations; D14 records no resolutions, so the rows above supersede recommendations, not decisions.

Within H, the External providers section clarifies the earlier forward-phase wording: output may reach a later phase in the same period; feedback toward an earlier phase waits for the next period unless an explicitly owned bounded clearing loop applies. This is not permission for an implicit general feedback cycle.

The handoff does not independently revoke every earlier serialization choice. D04’s versioned JSON interchange and content-identity requirements remain inherited where compatible. The new runtime must declare its own exact frozen-data and save formats; relaxing Python byte parity is not permission to accept ambiguous or unversioned data.

## 5. Boundary clauses proposed for review

**Status of this entire section: Tentative.** The adopted principles are identified separately from proposed operational wording. These formulations are intended to make neighboring decisions compose; they are not additional user decisions. A reviewer should recommend acceptance, an exact amendment, or rejection for each `P-` item.

### P-01. Atomic handoff is not atomic downstream success

**Adopted basis:** H — Commit and submit; Validation and fallback. D04 separates authorization, execution, take-up, and settlement and forbids implied package-wide financial atomicity.

**Proposed contract wording:**

The folder handoff atomically admits a valid slate of player decisions and the immediate effects those decisions are authorized to create. Before commitment, Rust validates the combined slate, including shared resource demands, mutual exclusions, access and authority requirements, and the bindings behind each option. An invalid slate does not partially change canonical world state or create partial resource reservations.

Admission of a proposal is not approval by its decision body. Admission of a facility-related command is not counterparty take-up. Admission of a trade or operating instruction is not completed settlement. Later stages retain their own owners, dates, typed outcomes, and witnesses. A valid submitted slate may therefore produce mixed downstream results without violating submission atomicity.

The practical card for the combined slate describes its aggregate immediate commitments and relevant interactions. It does not guarantee contingent future outcomes or merely concatenate individually affordable cards whose total is unaffordable. Repeated delivery of the same handoff must not execute it twice; the exact receipt/idempotency representation is implementation work.

**Disposition:** D2 fixes the admission boundary and intermediate speaking commits in the adopted clause below. A penciled question never executes an inquiry or reveals a counterfactual answer.

**Adopted clause (review P-01, owner decision D2):** Admission means three things happen together or not at all: the slate passes Rust validation (authority, access, eligibility, mutual exclusion, and binding resolution), every capacity the slate consumes is reserved as a dated commitment, and every command the slate authorizes is enqueued with its receipt. Nothing downstream of the queue is part of admission. A call, interview, or meeting is itself a folder. Authored lines that only speak are penciled like any other choice. An authored line that asks a question, makes an inquiry, or discloses something commits when the player selects it and the turn passes to the counterpart, because the counterpart's answer is a delivery that cannot be un-heard. The practical card for such a line marks it "commits on speaking". A folder may therefore contain several commit points; each is witnessed separately.

**Candidate acceptance case:** Two independently affordable staff requests oversubscribe the same combined window. No partial assignment leaks through an invalid handoff. A valid alternative slate is admitted once; its later FOMC proposal can still be rejected through ordinary procedure.

### P-02. Invalidation changes drafts, derivations, and the prospective path—not history

**Adopted basis:** H — Ordering and invalidation; Commit and submit; Historical temporality.

**Proposed contract wording:**

Distinguish three operations. Revising a penciled decision invalidates affected draft calculations without mutating canonical state. A committed transition invalidates derived projections or not-yet-resolved future work whose declared inputs changed. A later cancellation, correction, amendment, or reversal creates a new canonical transition under the relevant lifecycle.

None of these operations deletes an already committed decision, delivered disclosure, consumed resource, completed payment, historical enactment, or recorded actor response. A prospective legal amendment can cancel or replace a future effective-date obligation while retaining the original enactment and all prior consequences. Historical corrections and score adjustments are additive records, not silent replacement of the accepted past.

Cached previews cannot become alternative execution paths. When a cached value has affected a committed decision, the record needed to explain that decision remains attributable even if the cache itself is later invalidated.

**Review still required:** Define the concrete versioning and cancellation rules for queued work and distinguish them from developer-only counterfactual experiments. Do not infer player-facing rollback or optimistic-conflict mechanics.

**Candidate acceptance case:** A submitted communication produces a public receipt. A later correction changes future interpretation opportunities but does not erase the initial publication, its deliveries, or actions already taken in response.

### P-03. Calendar periods and internal resolution boundaries are different concepts

**Adopted basis:** H — Temporal cadence; Ordering and invalidation; External providers.

**Proposed vocabulary, not new Rust type names:**

| Term | Meaning to preserve |
|---|---|
| Calendar date/time | The institutional timestamp of a commitment, observation, deadline, meeting, or other modeled occurrence. |
| Calendar span | The interval organized around scheduled institutional anchors. |
| Discretionary activity period | A bounded player opportunity to allocate concrete capacities between anchors; not a universal point or necessarily one day. |
| Resolution period | An internal unit in which the fixed forward-phase ordering applies. Its duration and relationship to dates must be declared. |
| Phase | A coarse forward ordering boundary for work inside a resolution period. |
| Decision context | The settled information and procedural context in which a folder is deliberated. |

**Proposed contract wording:**

An authorized calendar advance may process several dated observations, settlements, releases, capacity completions, and reviews before returning control. One date may contain several player decision contexts, and several dates may resolve without an empty player turn. Godot frame timing, animation duration, and the player’s reading time never decide these transitions.

A resolution specification declares its phase sequence, stable tie-break order, treatment of same-time arrivals, and the point at which completion releases capacity versus makes a deliverable available. An output can feed a later phase in the same period. A dependency toward an earlier phase is scheduled prospectively. A bounded clearing loop must name its owner, state boundary, stopping rule, and typed failure result.

Calendar stop rules consume only the institutional observations, schedules, monitoring doctrine, and commitments permitted by H. The resolution engine may know hidden state; the decision to foreground an interruption cannot use that knowledge as an undeclared input.

**Disposition:** D3 fixes the internal resolution period and interruption boundary below. The concrete phase roster is builder latitude; it cannot introduce a continuously running player clock or equate a resolution period with a Godot frame.

**Adopted clause (review P-03, owner decision D3):** The resolution period is one calendar timestamp's worth of due work. Within it the phase roster is fixed and forward. The builder derives the roster from the baseline's integer priorities (`engine/clock.py`, priorities 10 through 90) and names it; the expected shape is `OPEN`, `RELEASE`, `STAFF`, `AUTHORITY`, `EXECUTION`, `CLEARING`, `SETTLEMENT`, `PUBLICATION`, `RECEPTION`, `COMMITMENT`, `REVIEW`. Each handler declares its phase, reads, and writes; two handlers writing one state in one phase is a compile error. An institutionally observed emergency never rewrites an open folder's decision context. It enqueues an interruption that presents itself after the current handoff or before the next folder opens, with a banner in any still-open folder saying new material exists.

**Candidate acceptance case:** The same command sequence under different UI frame rates and reading delays produces identical simulation order. An intervening observed deadline can stop an authorized advance; an undisclosed latent vulnerability alone cannot.

### P-04. Stewardship findings cannot become a hidden-information channel

**Adopted basis:** H — Stewardship score; Retrospective conclusions; Evidence routing. D10 makes staff postmortems ordinary, fallible, access-limited institutional work.

**Proposed contract wording:**

Separate the in-world review, the extradiegetic evaluation, and the score ledger. The review carries attributable conclusions and their evidence. Authored findings identify the criteria and signed values relevant to that review. The ledger records the resulting accepted delta and explicit later adjustments. None is a causal-world variable or an input into actor behavior.

A proposed finding is eligible only on the evidentiary basis permitted for that review. A canonical witness can establish the identity or occurrence of a referenced event, but an otherwise inaccessible developer fact cannot silently determine a visible award or deduction. Unestablished attribution stays unestablished. Later lawful evidence may support a supplemental finding and a visible adjustment rather than retrospectively pretending the initial review knew more.

Standing criteria may be visible from the beginning. A contingent subject, criterion, or score explanation is not surfaced before that subject is institutionally known. The magnitude rubric must not reveal the same hidden distinction indirectly through its selected point band.

**Review still required:** Confirm this evidence-grounded eligibility interpretation or provide an alternative that satisfies both witnessed-fact evaluation and non-omniscient disclosure. H establishes those obligations but does not fully specify how uncertain conclusions become eligible findings. In particular, state how disputed attribution, incomplete evidence, and later exoneration affect entries without making a staff opinion synonymous with canonical truth.

**Candidate acceptance case:** Two runs differ only in an inaccessible precursor. Before evidence of that distinction becomes available, the scorecard cannot expose it through a hidden-truth-specific finding or value. A later disclosed fact may cause an explicit supplemental adjustment.

### P-05. Review disposition, score persistence, and non-suppression require a terminal lifecycle

**Adopted basis:** H — Stewardship score; Rewrite and proof; Chair succession and campaign bounds.

**Proposed contract wording:**

A completed review has a stable identity and version, an author, evidence timing, conclusions, available disposition actions, and applicable findings. Accepting it records disposition, not compulsory agreement. A Chair response is retained alongside the review. A requested revision states the evidence-backed issue and creates the actual institutional work and capacity consequences it requires.

Score entries retain their source review/version and constituent findings. The same accepted finding cannot be awarded again merely because a report is reopened, redelivered, or discussed in another scene. A supplemental review distinguishes new findings from adjustments to earlier evaluated findings. A downward or upward revision adds an explicit compensating entry; it does not mutate the original ledger entry.

The score is non-causal but still authoritative persisted campaign data. Presentation cannot independently author, recalculate, or discard that ledger. Save/resume and succession preserve the total, provenance, pending review state, and the ability to audit adjustments without transferring private cognition.

**Disposition:** D1 fixes the terminal institutional event below. Its campaign implementation belongs to M3. D5 preserves the in-world review's lack of a universal verdict while requiring a separate extradiegetic scorecard in M2.

**Adopted clause (review P-05, owner decisions D1 and D5):** A completed review that remains undispositioned when the next scheduled institutional review anchor of the same kind arrives is registered as "acknowledged without response" by the office that commissioned it. That registration is a witnessed institutional event, not a player action. The review's findings are evaluated and its delta posts at that moment. The Chair may still attach a response or commission a supplemental review afterward; those create new entries, never edits. Score-ledger ownership is `reservist-core`; the Godot client renders it and can neither compute nor persist it. The in-world `StaffReview` keeps "no universal verdict"; the extradiegetic scorecard is a separate object; gate 09 keeps its in-world assertion and gains a scorecard assertion.

**Additional implementation output:** Assign the authoritative review/evaluation/ledger owners within the selected Rust architecture. They must not become GDScript persistence or a hidden continuous utility score.

**Candidate acceptance cases:** Reopening the same accepted review does not duplicate its points; a later adverse correction lowers the current total through a new entry; an unfinished adverse review cannot remain permanently non-evaluable solely through UI avoidance once the chosen terminal rule applies.

### P-06. One-way rewrite parity distinguishes preserved behavior from adopted changes

**Adopted basis:** H — Rewrite and proof; Chief of staff; Temporal cadence; campaign and scoring decisions. D11’s original cast and cut differ from several newly adopted requirements.

**Proposed contract wording:**

Before changing the executable baseline, identify and preserve the Python revision, scenario inputs, representative command sequences, tests, and outputs used as the temporary oracle. Inventory the complete executable behavior to be replaced; do not substitute a new narrow demo while leaving working features on the Python production path.

Classify every material comparison as preserved behavior, authorized design change, or new-runtime guarantee. Preserved behavior compares exact conservation, ownership, authority, required event order, and categorical outcomes. Calibrated comparisons use explicitly declared channel tolerances. Authorized changes name the H clause they implement, the old expectation they replace, and their new acceptance evidence. New-runtime guarantees test determinism, exact checkpoint/resume, isolation, and complete persistence inside Rust/Godot.

A known defect is not required behavior merely because Python exhibits it. Conversely, an unexplained regression cannot be renamed an authorized change. Keep differences attributable. Temporary test tooling may use the frozen oracle; production does not retain a Python sidecar or two competing causal engines after acceptance.

**Review still required:** State the exact replacement acceptance scope, including how immediately applicable adopted changes are tested. The chief’s named cognition is an explicit override, not an optional backward-compatible default. Campaign scoring and succession are adopted end-state commitments, but were excluded from the old executable MVP. The reviewer must assign their implementation milestone explicitly rather than treating their design adoption as proof they already existed or silently expanding “port the complete executable” into “implement every historical design artifact.”

**Reviewer proposal (RDR-2026-09-05-01, P-06; pending owner decision D4):** The replacement scope is the executable behavior of `engine/cli.py` at revision `97fc926a`: `validate`, `freeze`, `run`, `replay-check`, and `play` with its twelve REPL verbs, across the three policy packages, plus every assertion in the 86 tests and 10 gates. Preserved behavior: conservation, ownership, authority stages, receipt chains, event order, categorical outcomes, the delivered-record boundary, and dated capacity with displacement and release (already implemented in `engine/staff/capacity.py` and `engine/commitments.py`). Authorized changes, each citing its H clause: chief of staff as a named Person; calendar board and discretionary periods; folder handoff; typed commands and projections; extradiegetic scorecard alongside the in-world review. New-runtime guarantees: checkpoint/resume (absent in Python), static registration, generated permission matrix, compiler fail-closed loading. Milestones: M1 parity and new-runtime guarantees; M2 the authorized changes; M3 succession and the full score ledger; M4 channel providers and world content.

**Candidate acceptance case:** A matching Treasury transaction conserves the same exact stocks, while the chief’s newly expanded behavior is tested against H rather than forced to reproduce the former limited model. Every divergence has a recorded category and evidence.

### P-07. Scenario closure covers supported future actions, legal changes, and succession

**Adopted basis:** H — Catalog and scenario distinctions; Legal regime; Historical temporality; External providers; Chair succession. D05 prohibits presentation-driven invention of unrepresented causal history.

**Proposed contract wording:**

A scenario’s supported action and transition space must remain representable under its frozen contracts throughout the declared campaign bounds. A provider is not safely `RECORDED` merely because the player has no relevant action at the opening instant. Supported future legal changes, interventions, and office transitions must be considered when declaring whether that channel can receive player-reachable feedback.

A transition that requires responsive behavior must have the necessary compiled handler, state contract, and initialization/provenance path. A successor must have a valid legal and representational path into the office without fabricating inherited private beliefs, personal relationships, or preexisting accounts. This obligation does not require every future person to be named at startup; the permissible construction or reserve mechanism must preserve the Bible’s history and fidelity constraints.

Within the game’s represented action vocabulary, missing implementation is not a lawful refusal and a manifest is not an arbitrary permission list. The compiler rejects an unsupported composition rather than letting a broken binding appear later as a plausible economic outcome.

**Review still required:** Specify the bounded authoring and validation mechanism for these future dependencies. Do not assume an exhaustive general reachability solver, universal dynamic entity creation, or a new plugin engine. Identify which paths are guaranteed by the selected fixture and which broader paths remain outside its declared scope.

**Reviewer proposal (RDR-2026-09-05-01, P-07):** The scenario manifest declares `supported_transitions`: the legal changes, interventions, office transitions, and provider interventions that campaign play may reach. The content compiler checks each declared transition against the registered handlers, provider execution modes, and fidelity permissions. A `RECORDED` provider on a channel named by any supported intervention is a compile error. Transitions not declared are outside the fixture's declared scope and the compiler records them as such; they are not lawful refusals. No reachability solver is built.

**Candidate acceptance case:** A supported future law permits an intervention into an external channel. A recorded-only provider that cannot consume that intervention is rejected or replaced through a previously valid composition contract; the action is not silently removed from the player’s lawful options.

## 6. Acceptance obligations and evidence plan

These are **target proof obligations**, not checked boxes. “Required” means demanded by adopted H or compatible inherited contracts. “Pending” means the exact assertion depends on a Section 5 disposition. No row claims an existing passing test.

| ID | Obligation | Minimum evidence | Basis |
|---|---|---|---|
| G-01 | Frozen bindings are complete and stable. | Reject duplicate contract IDs, unresolved owners, missing handlers, incompatible schemas, missing witnesses, and invalid period/fidelity selections before start or resume. Store no executable pointers in frozen data. | Required: H — Catalog contract binding. |
| G-02 | Representation is not universal dispatch. | Passive/non-owning catalog entries are not forced into jobs; executable contracts resolve only through the appropriate registered lifecycle. | Required: H — Activation semantics. |
| G-03 | Fidelity validation checks all three levels. | Reject Rust-incompatible clades, catalog-unpermitted models, and missing model-required state. Compare generated permissions with the actual registrations. | Required: H — Fidelity traits. |
| G-04 | Owned-state and port semantics are enforced. | Negative tests for wrong-owner writes, another producer’s state, unknown input/output ports, unit/schema mismatch, and stock ownership by a non-owner. A passing baseline must fail for the intended mutation. | Required: D04/D05 and H — bindings/validation. |
| G-05 | Defects and economic outcomes are different. | A lawful inability or clearing failure produces its typed outcome and witness. A missing handler or accounting violation fails as a defect and is not wrapped in gameplay fallback. | Required: H — Runtime dispatch; Validation and fallback. |
| G-06 | Ordering is fixed, deterministic, and forward. | Ambiguous writes or prohibited cycles are rejected; triggered work follows declared phase dependencies and stable ordering; bounded loops expose termination/failure. | Required principle; exact ordering cases pending P-03. |
| G-07 | Presentation and wall time cannot run mechanics. | Vary reading delay, frame timing, animation, and purely presentational content while keeping the same explicit commands. Causal state and applicable ordering remain unchanged. | Required: H — Temporal cadence; Client boundary. |
| G-08 | Capacity is dated, owned, and released by lifecycle. | A task’s reservations persist across advance and resume, constrain displaced alternatives, and release once under completion/cancellation/other declared conditions. Delivery timing is separately inspectable. | Required: H — Temporal cadence. |
| G-09 | Folder handoff respects the complete slate. | Draft changes have no effects; combined conflicts are visible; invalid admission does not partially mutate; later valid economic failures remain possible. | Required principles; exact atomic boundary pending P-01. |
| G-10 | Committed history is never silently rewritten. | Correction, amendment, cancellation, and repeal preserve earlier witnesses while updating the prospective path. | Required: H — Commit and submit; Historical temporality; P-02 details. |
| G-11 | Authority and policy stages remain separate. | Trace a proposal through relevant vote/delegation, authorized execution, counterparty behavior, settlement, and later observation, including negative paths. | Required: D04; H — Legal regime. |
| G-12 | Godot receives only permitted commands/projections. | No canonical references are exposed; invalid options are rejected by Rust; GDScript contains no causal behavior or persistence. Serialization does not create a privileged UI path. | Required: H — Runtime selection; Client boundary. |
| G-13 | Information routing is evidence-bounded. | Demonstrate that routing cannot read hidden crisis identity, future damage, or canonical importance; show lawful routine access and a consequential access conflict. | Required: H — Evidence routing; Evidence access. |
| G-14 | Reporting remains useful without inventing certainty. | A hard deadline supplies multiple feasible, costed institutional alternatives and conditional judgments; it grants neither unavailable evidence nor unlawful delay. | Required: H — Evidence access and reporting friction. |
| G-15 | The chief is a named person bound to an office. | Persistent private cognition, officeholding, recommendations, and appropriate access; replacing the holder transfers duties/records but not private state. | Required: H — Chief of staff. |
| G-16 | Succession preserves institutional continuity. | Each supported succession cause follows the legal path, rebinds control, preserves official burdens, and separates personal from institutional continuity. | Required campaign contract; milestone pending P-06/P-07. |
| G-17 | Stewardship is review-linked, immutable, and non-causal. | Check explicit findings and point values, adjustment entries, non-duplication, hidden-subject protection, no actor read path, and persistence across resume/succession. | Required principles; evidence eligibility/finalization pending P-04/P-05. |
| G-18 | Rewrite comparisons are attributable. | Preserved behavior, authorized differences, and new-runtime guarantees each have identified fixtures and expected results. Every executable feature in the replacement scope is accounted for. | Required: H — Rewrite and proof; scope detail P-06. |
| G-19 | Checkpoint/resume is exact inside the new runtime. | Save at consequential supported boundaries with outstanding work, private state, reservations, events, and records; continued behavior equals an uninterrupted run. | Required: H — Rewrite and proof; D04 persistence. |
| G-20 | Provider execution matches supported interventions. | Recorded providers have no unsupported reachable feedback; responsive providers retain behavior-affecting state and declared responses; residual/accounting boundaries reconcile. | Required: H — External providers; P-07 detail. |
| G-21 | Authored options and previews match mechanics. | A selected option resolves to its reviewed binding; the exact card matches immediate mechanics; assessed outcomes cite institutional evidence and never forecast from hidden truth. | Required: H — Authored options and mechanical previews. |
| G-22 | Production contains no runtime model dependency. | Exercise the game without a model/network service and inspect relevant executable dependencies; authored templates, options, and bindings remain sufficient. | Required: H — Language models. |
| G-23 | Assets retain independent identity and provenance. | Validate approved source references and metadata, pinned importer settings, excluded generated caches, and separation of presentation from simulation identity. | Required: H — Asset pipeline. |
| G-24 | The mechanism teaches a transferable distinction. | Where a mechanism has genuinely different regimes, collect player-observation evidence on sibling and holdout fixtures; test inspection and reasoning, not memorized package choice. | Required research direction: H — Transferable learning. |

An untriggered path is not execution proof. A coverage flag without a fixture and witness is not proof. A compiler test is not a market-validity test; a replay test is not evidence that people understand the game. Readiness reports must retain these distinctions rather than emit one generic completion percentage.

## 7. Implementation handoff and corpus migration

**Status:** Proposed organization of work under the adopted one-way rewrite. This section is not an incremental production migration plan and does not authorize an unapproved scope change.

### 7.1 Establish the temporary oracle and its limitations

Identify the exact Python revision, frozen scenario, content inputs, runnable commands, and behavioral tests. Preserve representative fixtures before catalog migrations alter the source data. Record any known deviations from adopted contracts. The historical implementation report is a locator and claim to verify, not a substitute for obtaining the executable baseline.

Inventory runtime behavior, not only named modules: initialization, commands, authorization, staff work, cognition, clearing, settlement, records, calendar progression, failure paths, review output, and persistence actually supported by the old executable. Mark unavailable or absent capabilities honestly. Do not claim parity for an old capability that was never implemented.

### 7.2 Encode the selected contract system as part of the rewrite

Implement the Rust registrations, generated fidelity permissions, content compilation, fixed scheduling boundaries, and fail-closed loading needed by the complete replacement scope. Reconcile source IDs and definitions directly rather than adding a general alias resolver.

Resolve the authoring dependency between compiled Rust metadata and the content compiler explicitly: the compiler needs an authoritative registry/schema/permission description matching the target runtime. The packaging mechanism is an implementation choice; frozen content must not depend on Godot presentation code or a separately hand-maintained shadow registry.

### 7.3 Port behavior and incorporate approved differences

Reimplement the executable program and its player-facing interaction path through the Rust/Godot command/projection boundary. Use the three comparison categories from P-06. Maintain the provenance of commands, witnesses, and evidence while replacing the implementation substrate.

Keep authored option text separate from its stable binding and display metadata. Preserve the adopted distinction between a practical card’s exact mechanics and its fallible institutional assessment. Do not achieve preview convenience by giving Godot access to canonical state.

### 7.4 Prove the replacement, then remove production Python

Run the ported behavioral comparisons and new-runtime guarantees. Explicitly account for authorized differences. Demonstrate the complete selected replacement without the old runtime in the production path. Historical oracle fixtures may remain as test evidence; Python is not the permanent causal authority, client sidecar, or fallback engine.

The full basis-trade composition proof remains a distinct obligation with its own member activation and witnesses. It is not replaced by passing a small market fixture, and it is not silently added to the player-facing scope merely because the original corpus discusses it extensively. **Sources:** D04 — closed basis proof and per-member acceptance; D11 — thin live cycle and independent deeper proof.

### 7.5 Update the owning specifications

| Owner/source | Change required | Preserve |
|---|---|---|
| D04 — kernel/whole-game architecture | Replace general scheduling, player-time, language-model, campaign-end, and evaluation formulations where H supersedes them. Integrate accepted boundary dispositions. | Ownership, accounting, stage witnesses, information separation, exact continuation, and independent economic proofs. |
| D05 — Representation Bible | Encode fidelity enforcement and named chief requirements; integrate campaign-continuity amendment without weakening identity boundaries. | The kinds table, non-owners, person/office split, residuals, no invented causal history, and satire invariants. |
| D06 and catalog sources/schema/validator | Encode complete contracts, canonical IDs, three eligibility concepts, channel-specific providers, permission generation, and legitimate readiness states. | Researched instance content and provenance where still applicable. |
| D09 — media | Remove permission for runtime prose generation where applicable; bind authored options and reports without changing independent audience stages. | Claim registry, living/institutional source attribution, audience-owned interpretation/action, and non-supernatural presentation. |
| D10 — interface | Make the new calendar, packets, practical cards, routine access handling, folder handoff, and review-linked score presentation operative. | Accessible records, optional depth, no traversal tax, source-specific uncertainty, and fallible postmortems. |
| D11–D12 — MVP and implementation history | Record adopted cast/cadence/interface changes and a separately identified rewrite acceptance scope. | Historical reports of what Python implemented; do not rewrite them into Rust completion claims. |
| D14 — architecture alternatives | Replace the operative unresolved-choice status with exact H dispositions and links to owners. | Rationale and rejected alternatives as historical evidence. |
| Generated views and exporters | Regenerate after changing their source and acceptance logic; remove hardcoded pre-runtime status assumptions. | Source hashes, coverage boundaries, and distinct structural/execution/calibration verdicts. |

### 7.6 Previously identified discrepancies become conformance work

The earlier audit identified permission-matrix drift, selected entries that remained ineligible, scenario failure declarations disconnected from catalog fallback, duplicated stable subjects, weak composition coverage, missing ownership/port checks, and a generator that still said no manifest existed. The handoff now provides decisions against which those artifacts should be repaired.

Do not “fix” those findings by merely setting selectable flags, deleting legal actions, assigning mechanical fidelity to every subject, giving lenses ownership, weakening assertions, or treating all failure as fallback. Repair the contract and its concrete binding. Where the current repository has already changed, record the verified replacement instead of repeating a stale audit result.

`catalog/catalog.py:837` (`write_completeness`) literally emits “scenario representation manifest remains unauthored” while `scenarios/mvp_2006_cycle/manifest.json` exists and validates. That is a source-level reporting assumption, not a discovered runtime fact. Re-running the same generator cannot turn it into reliable current evidence. The line must become conditional on manifest discovery or be removed, and `catalog/generated/` regenerated.

## 8. Remaining work register

### 8.1 Actual boundary decisions still requiring disposition

| Item | What is decided already | What remains |
|---|---|---|
| P-01 | Free draft revision and atomic folder handoff; separate downstream owners. | Exact admission/immediate-effect boundary and treatment of intermediate effectful dialogue. |
| P-02 | Immutable committed history and prospective legal/action lifecycles. | Concrete invalidation, versioning, and queued cancellation rules. |
| P-03 | Player-authorized calendar and fixed forward phases. | Phase roster, resolution-period meaning, ordering ties, and any open-context interruption protocol. |
| P-04 | Evidence-backed review-linked score and non-omniscient information. | Finding eligibility under uncertainty and the precise boundary between canonical witnesses and review-admissible evidence. |
| P-05 | Immutable compensating score entries; no suppressing adverse results by refusing closure. | Terminal review-disposition owner/event/deadline and concrete ledger ownership. |
| P-06 | Complete one-way executable rewrite, not permanent Python production. | Explicit scope of immediate adopted changes versus later campaign implementation, plus versioned comparison cases. |
| P-07 | Dynamic law, continued succession, and recorded-provider feedback restrictions. | Bounded authoring/validation of supported future transitions without arbitrary action whitelists or fabricated history. |

These questions are not permission to reopen the selected engine, headline score, no-model rule, or calendar style. A reviewer may close a listed item by finding an already controlling source, but must give that source rather than choosing an answer silently.

### 8.2 Mechanism, content, and balancing work

H explicitly leaves numerical calibration budgets, selected-provider schemas and parameters, period-specific accounts and counterparties, effective-period contracts, residual quantities, opening-state research, and human transfer-learning evidence unfinished. The Stewardship section also explicitly leaves exact point bands for balancing against representative postmortems.

These obligations need versioned inputs, evidence, and acceptance criteria. They do not imply that the ownership vocabulary or runtime substrate is undecided. A particular coefficient should be promoted to a default only with its scope and evidence, not because it was used once in a fixture.

### 8.3 Implementation work under closed choices

Compiled registration, the permission generator, compiler diagnostics, source migration, Rust domain modules, the thin bridge, Godot scenes/themes, authored bindings, access checks, save/replay implementation, asset validation, and all target tests remain implementation outputs unless separately evidenced. A design decision, a diagram, or a checklist cannot attest that they exist.

## 9. Instructions for the HumanLayer review

Review this document as a **design amendment with an embedded accepted baseline**, not as an invitation to redesign Reservist from first principles. Section 3 is the user decision source. Section 5 is explicitly proposed reconciliation. Section 6 is an acceptance plan, not evidence of tests run. Read the compatible source contracts named in Section 10 when a finding depends on them.

If the complete corpus is available, use it to resolve apparent gaps and exact source precedence. If a file, implementation, or revision is unavailable, name the limitation. Do not infer completion from generated coverage, a previous model’s confidence, or a source filename. Do not require outside research to reselect a choice already made; use research only for a concrete feasibility or factual issue, labeled separately.

### Required reviewer output

**A. Overall judgment.** State whether the adopted choices compose, which proposed boundary clauses can be accepted, and which actual contradictions or blockers survive. Do not collapse product acceptance, static validity, runtime conformance, economic calibration, and human comprehension into one verdict.

**B. Findings table.** For each material finding, provide:

| Finding ID | Classification | Exact document/source locator | Conflicting or missing contract | Minimal correction | Affected owners/tests | Confidence |
|---|---|---|---|---|---|---|

Use classifications such as `contradiction`, `missing interface`, `unsupported claim`, `scope ambiguity`, `migration requirement`, `calibration/content`, and `already resolved`. These are review categories, not new runtime enums. Explain why the item matters and whether the supplied source actually resolves it.

**C. Clause dispositions.** Address P-01 through P-07 individually with `accept as drafted`, `accept with replacement text`, `reject with reason`, or `requires owner decision`. Supply exact replacement paragraphs rather than a second expansive architecture essay. A reviewer’s acceptance is a recommendation for the design owner, not automatic promotion to canon.

**D. Accepted-source fidelity.** Identify any omission, weakening, accidental scope expansion, or falsely attributed supersession relative to Section 3. Preserve the deliberate headline score, named chief, full one-way rewrite, no-runtime-model rule, and separation of eligibility concepts.

**E. Propagation plan.** Identify the owning source sections, catalog/schema changes, generated outputs, and acceptance evidence that must change together. Do not silently edit historical attestations into claims about the new runtime.

**F. Remaining register.** Return only genuine unanswered boundary decisions and explicit mechanism/content/research obligations. Remove questions answered by cited sources. Do not turn all unimplemented behavior into an open design choice.

**G. Agent-build readiness.** Identify whether the accepted baseline and proposed dispositions give an implementing agent enough authority and concrete contracts to execute the agreed program without repeated human task authoring or routine source review. Separate actual design blockers from choices the builder can make under the adopted constraints. Specify any missing completion or evidence obligation. Return a concise owner-decision packet and proposed Build Mandate; do not create a new mandatory documentation platform or promote your recommendations without disposition. The mandate must preserve the complete executable rewrite requirement and distinguish it from later campaign/world work, not silently shrink the task to a demo.

A successful subsequent implementation handoff includes a runnable result and an evidence-backed intended-versus-as-built shape report. Agents inspect the code, maintain source/proof links and current documents, and expose failures or uncertainty. The human-facing report must explain relevant ownership, flow, state transitions, player behavior, and departures in design language. This is a target deliverable, not a claim that an implementation was run during this review.

### Particular failure modes to test

Check for atomic submission being misread as guaranteed success; prospective invalidation becoming historical rollback; internal resolution becoming wall-clock or daily-turn play; a score or preview leaking hidden facts; adverse reviews being suppressible; authoritative evaluation living in presentation; a frozen scenario prohibiting a lawful supported action because its implementation is absent; a recorded provider receiving undeclared intervention feedback; and parity either preserving known defects or hiding new regressions.

Do not manufacture issues merely to populate the table. A precise confirmation with supporting evidence is preferable to another speculative subsystem.

## 10. Source register and citation method

### H — New decision authority

**Title:** `Open Questions Decision Handoff`  
**Origin:** User-supplied decision record in this conversation, reproduced in Section 3.  
**Role:** Adopted choices and explicit supersessions.  
**Repository path:** `docs/design/15-open-questions-decision-handoff.md`, byte-identical to the owner-supplied file. It was never a numbered HumanLayer artifact.

### Original corpus sources

The original task prefix was `federal-reserve-chair-crisis-management-simulator/`. All artifacts now live at `docs/design/<relative path>` (chapters) and `catalog/<relative path>` (catalog), byte-identical to the migration snapshot formerly indexed by `reservist-omni-split/00-index.md`. D6 authorized deletion of that duplicate tree after verification. Line numbers in this document and in RDR-2026-09-05-01 refer to the original-path files at revision `97fc926a`.

| Key | Original relative path | Sections used here |
|---|---|---|
| D04 | `04-design-discussion-minimum-simulation-kernel.md` | Foundational causal contracts; Hybrid task graph and cadence (line 267); Persistence and deterministic identity (line 655); Term, failure, and legacy (line 2160); resolved Q4 (line 3062) and Q16 (line 3167); alias rule (line 2844); interchange (line 2725). |
| D05 | `05-design-discussion-representation-bible.md` | Kinds table (28 kinds, line 158); closed fidelity tiers (Q44, line 935); legal and external-process ownership; Representation Invariants (line 1076). |
| D06 | `06-design-discussion-representation-catalog.md` | Instance contracts; manifest selection; eligibility and profiles; external-channel and composition contracts. |
| D07 | `07-design-discussion-burrow-composition-probe.md` | Owner-specific witnesses and distinction between paper composition, initialized runtime, and calibration. |
| D09 | `09-design-discussion-economist-pundit-media.md` | Resolved questions 1–7; event/evidence/claim/report distinction; stage-specific witnesses; non-causal presentation. |
| D10 | `10-design-discussion-epistemic-fairness-interface.md` | Resolved questions 1–12; evidence-bounded interface; ordinary staff postmortems; learning-transfer criterion. |
| D11 | `11-design-discussion-bernankey-mvp-slice.md` | Live cycle; non-goals; resolved questions 2, 5, 6, and 8; separation from full basis proof. |
| D12 | `12-structure-outline-bernankey-mvp-cycle.md` | Historical implementation phases, selected content, completion claims, and unresolved catalog integration. |
| R13 | `13-research-game-architecture.md` | Historical codebase inspection, frozen-slice construction, implemented actor inventory, test evidence, and presentation/hash finding. |
| D14 | `14-design-discussion-game-architecture.md` | Nine formerly open architecture questions and their recommendations; no resolutions had been recorded in that snapshot. |
| CATALOG | `catalog/schema.json`, `catalog/catalog.py`, and the relevant `catalog/inventory/*` / normalized CSVs | Authoring and validation structures to reconcile against adopted contracts. |

`00-index.md` provides original payload hashes and the mapping into nine thematic bundles. A bundle number is a transport locator, not an authority ranking. Older line numbers inside the artifacts may refer to earlier revisions; prefer exact original path and section plus a snapshot identifier where available.

The earlier `Reservist Current Design Synthesis`, `Reservist_Corpus_Audit_Appendix.md`, and `Reservist_Corpus_Audit_Findings.json` were never present on disk. Their claims survive only where this document or the Fable review re-verified them against the repository.

## 11. Integration completion criterion

This review cycle is complete when the design owner has dispositioned the boundary proposals, each adopted requirement has a designated current source location, conflicting operative text has been replaced or explicitly scoped as historical, and implementation obligations have been separated from decisions still requiring an answer. Agents perform the integration and bookkeeping after disposition; the owner should receive a coherent current bible, the material change summary, and an execution mandate rather than instructions to reconcile the sources manually. Internal implementation checkpoints do not each require new human authorization within the agreed scope.

The subsequent rewrite milestone is complete only when the full agreed executable replacement, approved differences, frozen bindings, information boundary, deterministic behavior, and checkpoint/resume evidence meet their acceptance contracts without Python on the production path.

These are two different completion claims. Neither a successful review nor a regenerated omnibus proves the other.
