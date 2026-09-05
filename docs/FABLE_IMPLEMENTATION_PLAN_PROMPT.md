# Fable Prompt: Reservist Implementation Plan

Create the implementation plan for Reservist’s authorized Rust/Godot rewrite.

This is a planning task only. Do not implement code, initialize the Rust workspace, change catalog data, or modify runtime behavior. Inspect the repository deeply enough that the plan names concrete existing modules, tests, fixtures, commands, dependencies, and acceptance evidence.

Start from these authoritative front doors:

1. `docs/CURRENT_DESIGN_BIBLE.md`
2. `docs/BUILD_MANDATE.md`
3. `docs/BUILD_REVIEW.md`
4. `docs/review/RDR-2026-09-05-01-fable-design-review.md`
5. `docs/design/18-current-design-amendment-v0_3.md`
6. `docs/design/15-open-questions-decision-handoff.md`

Read inherited chapters only where the front doors or acceptance obligations cite them. Inspect the actual Python implementation, scenarios, catalog, tests, and `justfile`; do not infer their contents from design prose.

## Authority and settled decisions

The Build Mandate is authorized.

The milestone order is fixed:

- M1: one-way parity rewrite
- M2: adopted design changes
- M3: succession and Stewardship Score
- M4: providers and world expansion

D1–D6 are settled as recorded in review Part G. Do not reopen them.

In particular:

- Inquiry and disclosure lines in calls commit when spoken.
- Observed emergencies queue an interruption and banner the open folder; they never rewrite its context.
- An undispositioned completed review becomes “acknowledged without response” at the next same-kind review anchor, and its score delta posts then.
- Gate 09 retains the in-world “No universal verdict is assigned” assertion. A separate assertion covers the extradiegetic scorecard.
- `reservist-omni-split/` has been deleted after restoration verification.

The checked-in v0.2 amendment points Section 3 to the handoff file instead of duplicating its approximately 30 KB because the files were byte-identical. Treat that as intentional.

The existing catalog state is reported, not repaired yet:

- 214 structural issues
- 2 failing catalog tests

Do not describe these as newly introduced regressions.

## Non-negotiable implementation constraints

- Production target: pure Rust causal core and content compiler, with a thin Godot client through `godot-rust`.
- Expected crates/components are `reservist-core`, `reservist-content`, and `reservist-godot`; adjust internal package boundaries only if repository evidence gives a concrete reason.
- Python becomes test tooling and a behavioral oracle only after parity. It must leave the production path.
- No causal state, persistence, or mechanics in GDScript.
- One process with command/projection isolation. Do not invent JSON IPC.
- No runtime language-model integration.
- No network dependency inside game bounds.
- Preserve deterministic identity inside the new runtime.
- Compare cross-runtime semantic invariants exactly and calibrated values only under declared tolerances.
- Use fixed forward phases and static handler registration, not a general reactive task graph.
- Committed history is immutable.
- Catalog, runtime, client, tests, authoritative sources, and assets remain in one repository.
- Catalog loading and scenario closure fail closed.
- Do not make catalog validation pass by flipping flags, deleting legal actions, narrowing declared coverage dishonestly, or hiding invalid rows.
- Preserve every existing behavior classified as preserved in P-06.
- Treat adopted differences and new-runtime guarantees separately from parity.
- Do not silently add compatibility shims, dual production runtimes, speculative abstractions, or deferred placeholders.

## Planning goal

Produce an implementation-ready plan that a coding agent can execute continuously without asking the owner to author routine subtasks.

The plan must turn M1 and M2 into small, coherent work packages with explicit dependency order, affected files or new package paths, observable outputs, and verification commands. Plan M3 and M4 at milestone resolution only unless an interface must be fixed earlier to avoid rework.

Do not merely restate G-01 through G-24. Convert them into executable engineering work.

## Required analysis

Before writing the plan:

1. Map the current Python production surface:
   - `validate`
   - `freeze`
   - `run`
   - `replay-check`
   - `play`
   - all twelve REPL verbs
   - all three policy packages

2. Map each Python subsystem to its proposed Rust owner:
   - canonical serialization and hashes
   - manifest and scenario loading
   - clock, phase ordering, and event dispatch
   - state and persistence
   - authority and receipts
   - claims and publication
   - evidence delivery and player records
   - staff work, capacity, commitments, and displacement
   - participant cognition and voting
   - markets, repo, accounting, and settlement
   - postmortem/review
   - adapters/providers
   - CLI and Godot presentation boundary

3. Inventory the current fixtures and tests that can serve as oracle vectors. Identify:
   - exact preserved contracts
   - authorized divergences
   - new-runtime-only guarantees
   - tests that are meaningful behavioral contracts
   - tests that are only weak structural or token checks and need behavioral replacements

4. Inspect the catalog validator and its 214 issues. Group failures by root cause and dependency. Recommend either:
   - full global repair during M1, or
   - a compiler-enforced quarantine for genuinely out-of-slice rows

   The recommendation must explain how quarantine remains fail-closed and cannot become flag-flipping or hidden invalidity.

5. Identify the minimum toolchain and repository setup:
   - Rust toolchain pin
   - Cargo workspace
   - Godot version
   - `godot-rust` version and compatibility
   - Python version pin for the oracle
   - `just` commands
   - deterministic formatting, linting, testing, and artifact comparison

   Verify current stable versions from primary documentation where version choice matters. Distinguish repository facts from external recommendations.

6. Design the parity harness:
   - how Python and Rust receive identical frozen inputs
   - canonical transcript and state projections
   - exact versus tolerant comparisons
   - attribution of every divergence
   - deterministic replay
   - conservation and receipt-chain witnesses
   - removal criteria for Python from production
   - retention of Python as test tooling

7. Design exact checkpoint/resume:
   - supported save boundaries
   - canonical versioned format
   - outstanding events and work
   - reservations and commitments
   - private and institutional state
   - immutable records and receipts
   - resumed versus uninterrupted equivalence
   - schema/version failure behavior

8. Define the M1-to-M2 boundary precisely. M1 must not accidentally implement adopted M2 behavior as “parity,” but its interfaces must not force M2 into a rewrite.

## Required output

Return one proposed implementation-plan document with these sections:

### 1. Executive decision

State whether implementation can begin without another owner decision. List only genuine unresolved design conflicts. Ordinary engineering choices belong in the plan, not in an owner-question list.

### 2. Repository baseline

Record the inspected implementation, toolchain state, scenario/oracle pin, passing tests and gates, known catalog failures, and current production entry points. Label claims as inspected or executed.

Do not rerun the known failing global catalog checks merely to confirm the reported counts unless analysis requires their detailed output.

### 3. Target architecture

Give a concrete package/module map for:

- `reservist-core`
- `reservist-content`
- `reservist-godot`
- Python oracle tooling
- shared fixtures and cross-runtime tests

For each boundary, state ownership, permitted dependencies, forbidden dependencies, serialized interfaces, and causal-state ownership.

Include a dependency diagram if it communicates real structure.

### 4. Contract classification matrix

For every current executable subsystem and relevant G acceptance item, classify it as exactly one of:

- preserved parity behavior
- authorized adopted difference
- new-runtime guarantee
- M3 contract
- M4 contract
- research/calibration obligation

Give its Python source, Rust destination, evidence, and milestone.

### 5. Ordered implementation work packages

Provide a dependency-ordered sequence for M1 and M2.

Each work package must include:

- objective
- why it is ordered here
- exact existing files/symbols to inspect or replace
- expected new or modified paths
- public interfaces introduced
- behavior preserved or intentionally changed
- fixtures used
- verification command or runnable scenario
- acceptance evidence produced
- explicit completion condition
- dependencies on earlier packages
- risks that could invalidate later work

Prefer vertical slices that produce runnable behavior. Avoid a long “foundation” phase that creates abstractions without exercised paths.

Keep commits coherent and independently verifiable. Suggest a bare Conventional Commit subject for each package.

### 6. Catalog conformance plan

Group the 214 structural issues by root cause, identify generator/schema/runtime dependencies, and sequence the repairs. Preserve legal actions and declared coverage. Explain the exact fail-closed behavior if quarantine is recommended.

Include the false readiness line currently emitted by `catalog/catalog.py` in the appropriate package.

### 7. Parity and acceptance strategy

Map the 86 tests, 10 gates, three package replays, and G-01 through G-24 to work packages and milestone exit criteria.

Do not require tests that only assert wiring, field forwarding, source text, or mock echoes. Replace weak tests with observable behavioral checks where the accepted contract needs them.

State the exact M1 exit criteria and exact M2 exit criteria.

### 8. M3 and M4 runway

Give only the interfaces M1/M2 must preserve for:

- succession
- review-linked immutable Stewardship ledger
- D1 terminal event
- separate in-world review and extradiegetic scorecard
- RECORDED/RESPONSIVE channel providers
- supported transition declarations
- asset provenance and Godot importing
- broader world content

Do not expand M3 or M4 into detailed implementation tasks unless necessary to prove an earlier architectural boundary.

### 9. Continuation procedure

Specify:

- the first concrete executable task
- commands to verify it
- how `docs/BUILD_REVIEW.md` is updated at coherent checkpoints
- what evidence must accompany each completed work package
- when an agent must stop for an owner decision
- when it must continue without asking

## Quality bar

The plan is unacceptable if it:

- restates the design without naming executable work
- treats the old Python implementation plan as the rewrite plan
- invents repository facts
- hides catalog invalidity
- weakens existing tests to obtain parity
- conflates in-world review with the scorecard
- treats Python and Rust as permanent coequal production runtimes
- creates a general scheduler, universal provider interface, runtime LLM seam, or causal GDScript layer
- leaves checkpoint/resume, parity attribution, catalog conformance, or Godot isolation as unspecified future work
- asks the owner to decide ordinary module, naming, serialization, or test-organization questions
- uses placeholders, “TBD,” “future work,” or implementation stubs inside M1 or M2

End with a concise ordered checklist of the first five executable work packages and the evidence each one must produce.
