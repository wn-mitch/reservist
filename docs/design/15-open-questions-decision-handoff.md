# Open Questions Decision Handoff

This document records the decisions reached while reviewing the surviving open questions. It is intended for the next design pass. Later decisions supersede earlier formulations where noted.

## Runtime contract and scheduling

### Catalog contract binding

Use runtime registration plus a generated frozen manifest.

- Author catalog and manifest data in CSV or JSON.
- Each selected contract names a stable contract ID, owner ID, handler key, input and output schema references, witness requirements, period variant, and fidelity selection.
- A lightweight compiler validates uniqueness, referential integrity, schema compatibility, registered-handler availability, and complete bindings.
- The Rust runtime registry maps stable handler keys to compiled executable handlers.
- Frozen scenario data records resolved identifiers, versions, and content identity, not function pointers.

### Ordering and invalidation

Use fixed forward phases with declared inputs and outputs.

- Work moves forward through coarse phases.
- Declared reads and writes derive local ordering inside a phase.
- Two handlers may not ambiguously write the same state.
- Cross-phase feedback enters a later tick or period rather than creating an implicit within-tick cycle.
- Changing an upstream decision invalidates affected drafts, derived projections, and unresolved future work. It never rewrites committed history; a correction or reversal is a new witnessed transition under the relevant lifecycle.
- Presentation-only changes do not rerun mechanics.

This is a deliberate scope control. It avoids a general reactive task-graph engine while preserving inspectable causality.

### Temporal cadence

Use a visible calendar-board cadence modeled on *Fire Emblem: Three Houses*, not a running grand-strategy clock.

- The game has no continuously advancing real-time clock or speed-control loop.
- Dates, deadlines, scheduled institutional anchors, and intervening events are visible on the calendar.
- Calendar advancement occurs in discrete player-authorized steps and never tracks player wall time.
- Each calendar span is organized around scheduled institutional anchors such as materials deadlines, meetings, auctions, testimony, and press events.
- Between anchors, the player receives a finite set of discretionary activity periods rather than one mandatory turn per calendar day.
- Uneventful dates may pass during resolution without becoming empty player turns.
- Activities may complete immediately or schedule reports, meetings, and consequences for later calendar dates.
- Discretionary activities consume concrete actor, team, access, approval, meeting-window, and delivery capacities rather than a universal action-point currency.
- Capacity use is represented as a dated commitment with an owner, allocation, start, expected payoff or deliverable, and release condition.
- Committed capacity remains occupied until work completes, is explicitly cancelled, or otherwise reaches its modeled release condition.
- Completion produces its report, meeting, decision support, relationship work, or other declared payoff and returns the released capacity.
- Reading and comparing already-delivered material does not consume capacity or advance the calendar.
- Practical cards show the capacity committed, expected completion, displaced alternatives, and conditions that could delay release.
- Scheduled briefing cycles are the ordinary player cadence.
- Observation events create dated institutional artifacts between briefings without necessarily stopping play.
- Calls, emergency meetings, deadlines, and other observed or committed conditions may interrupt before the next briefing.
- New material never silently replaces an open folder's settled decision context. An interruption banner announces its presence; the player may inspect, park, or restore that attention separately from the folder's existing choices.
- Stop rules use institutional observations, schedules, monitoring doctrine, and commitments, never hidden canonical crisis significance.
- Briefings show trends and accumulated changes so conditions do not jump from invisible normality to crisis merely because intermediate reports were skipped.
- Genuine surprise remains possible when indicators were inaccessible, misleading, unmonitored, or not escalated through the institution.

### Runtime dispatch

Use a trait-driven static registry.

- `WorkHandler` has associated input and output types plus stable handler key, phase, declared reads, and declared writes.
- Registration creates a monomorphized erased function pointer for heterogeneous runtime lookup.
- The registry does not store heap-allocated entity trait objects.
- Catalog data configures compiled handlers. Adding executable mechanics requires Rust code.
- Do not add runtime scripts, general plugins, or linker-discovered registration magic.
- Generate per-entry and combined Mermaid flows from registration metadata. Machine validation remains authoritative.

Economic failure is a typed output, not a handler error. For example, a market may return `FailedToClear` with residual quantities and a witness. Schema mismatch, missing handlers, or violated accounting invariants are defects and must not be converted into gameplay fallback.

### Activation semantics

Do not define one generic activation predicate for catalog entries. Dispatch follows the represented kind and its lifecycle.

- Policy packages activate through decisions and issue constituent commands.
- Facilities are authorized, prepared, opened, used by counterparties, and wound down.
- Legal clauses become effective through dated legal transitions.
- Markets run when orders or scheduled clearing require them.
- Mechanical systems process queues.
- Actors reconsider on declared evidence, thresholds, plan decision points, or scheduled reviews.

A catalog entry defines something that may be represented. It is not itself a runtime job.

## Economic fidelity

### Opening conditions

Use anchored stress regimes.

- Each opening state has a historical or institutional reference regime.
- Deliberately stressed parameters are identified.
- Starts may be bad and difficult.
- Expertise should improve the outcome distribution without guaranteeing success.
- Randomness should matter without overwhelming expertise.
- A fixture must not be automatically recoverable or secretly unwinnable.

Historical plausibility constrains the opening state and causal mechanisms. It does not force the historical outcome.

### Approximation tolerances

Treat the simulation as a decision-shaped, turn-based abstraction.

- Within-turn mechanics may compress long real-world processes.
- Preserve evidence-backed directional effects, regime transitions, ordering, and pivotal tails.
- Do not require a universal numeric epsilon or exact reconstruction of unknowable prices.
- Exact thresholds are required where an institution or contract actually defines one.
- Approximation error must not be disguised as economic randomness.
- Different credible decisions must materially alter important outcome distributions.

Channel-specific calibration budgets remain future mechanism and research work.

### Transferable learning

Use sibling fixtures and holdout playtests where a real mechanism has materially different regimes.

- Sibling fixtures may share superficial cues while changing a consequential underlying condition, such as liquidity stress versus insolvency.
- The distinguishing evidence must be institutionally obtainable rather than arbitrarily hidden.
- Practical cards explain what an instrument does, not whether it is the correct choice.
- Holdout observation tests whether players inspect and use the relevant distinction instead of repeating a memorized package.
- Do not require artificial inversions, confounders, or extra branches where the underlying mechanism is genuinely simple.

## Catalog, scenarios, and legal state

### Catalog and scenario distinctions

Keep three separate concepts.

1. Catalog eligibility asks whether an entry is validly represented.
2. Scenario composition selects instances, providers, period variants, residuals, and fidelity tiers for the initialized world.
3. Runtime action eligibility follows effective law, authority, capability, access, commitments, and current state.

A scenario manifest still selects a representation slice as required by the Representation Bible. It must not arbitrarily whitelist or delete legal actions. The global catalog is the full represented vocabulary; scenario composition decides which subjects require instantiated state.

### Identity aliases

Do not build a general alias or automatic entity-reconciliation system in pass one.

- Catalogs and scenarios are authored collaboratively with language models and reviewed against stable IDs.
- Authoring output must reference canonical catalog IDs directly.
- Presentation labels, localization, and satire remain in the presentation register.
- A source-specific alternate label may be added later only when a real import or search workflow requires it.
- Mergers, successions, reorganizations, and changed legal subjects remain explicit identity transitions, never aliases.

### Validation and fallback

- Invalid catalog structure fails catalog validation.
- Missing or incompatible frozen bindings prevent the scenario from starting or resuming.
- Runtime handlers are total over valid frozen inputs.
- Modeled inability, refusal, rationing, failed clearing, and failed settlement are typed outcomes.
- Do not use gameplay fallback to hide broken content or engine defects.

### Legal regime

Adhere to the Representation Bible's dynamic legal model.

- The Chair advises, testifies, coordinates, and applies institutional pressure but is not a parliamentary whip.
- Congress and other authorities independently own legislation, oversight, appointments, and legal transitions.
- Formal authority comes from effective legal instruments or valid delegations.
- Informal pressure cannot manufacture authority.
- Enactment, amendment, delegated rulemaking, effective dates, compliance, enforcement, supersession, and repeal may occur during play.
- Legal staff may misunderstand a rule. Their interpretation is epistemic state and does not rewrite canonical law.

### Historical temporality

Every scenario has three temporal layers.

- `HistoricalPrefix`: immutable events and provenance through scenario start.
- `OpeningSnapshot`: canonical state effective at scenario start.
- `FutureQueue`: obligations caused by prior history plus contingent future events and decisions.

The past is immutable. The future inside the campaign bounds is contingent. A past enactment or appointment may create a future effective date or term boundary, but a valid future amendment, repeal, removal, or succession can alter the prospective path without erasing the historical record.

## Runtime and repository

### Runtime selection

Use Godot with a pure Rust simulation core and a thin `godot-rust` GDExtension bridge.

- `reservist-core` owns canonical state, scheduling, markets, accounting, cognition, evidence, witnesses, saves, and replay.
- `reservist-content` owns catalog and scenario compilation.
- `reservist-godot` exposes a small Godot-facing bridge.
- Godot owns windows, scenes, `Control` layout, input, audio, assets, animation, and presentation.
- Use declarative Godot scenes and themes with minimal GDScript.
- GDScript may contain only local presentation behavior. It may not contain causal state, persistence, mechanics, cognition, economic decisions, or consequential scheduling.

### Rewrite and proof

The first runtime implementation task is a full one-way rewrite of the existing Python program.

- Freeze representative Python fixtures as a temporary behavioral oracle.
- Reimplement the complete executable program in Rust and Godot.
- Port the behavioral tests.
- Compare exact conserved state, ownership, authorization, event order, and categorical outcomes.
- Compare calibrated outputs under declared channel tolerances.
- Prove deterministic behavior and checkpoint/resume equivalence inside the new runtime.
- Remove Python from the production path after parity.

Cross-language canonical byte identity is not required. Semantic invariants, deterministic ordering, and exact checkpoint/resume behavior are required.

### Client boundary

Godot and Rust share one process but retain a strict command/projection boundary.

- Godot submits typed player commands.
- Rust validates and executes them.
- Rust returns receipts and immutable player-safe projections.
- Godot never holds canonical-state references.
- Commands and receipts may remain serializable for replay and debugging without introducing an external IPC protocol.

### Repository

Keep the authoritative catalog, scenario sources, Rust engine, Godot project, tests, and assets in one version-controlled repository. The catalog's current external location is accidental.

The runtime reads compiled frozen scenario data, not authoring CSV files during play.

## Representation fidelity and actors

### Fidelity traits

Represent closed fidelity tiers as sealed Rust model traits.

- A `FidelityModel` implementation defines complete required state and behavior.
- Rust defines which identity clades are type-compatible with each fidelity model.
- A catalog entry declares a narrower subset of models permitted for that subject.
- A scenario selects exactly one permitted model and may not omit required state.
- The content compiler generates the complete machine-readable permission matrix and validates all three levels.

Do not introduce a universal `Agent`, `Entity`, or per-entity update trait.

### Actor growth

Use typed fidelity tiers plus an event-driven `Reconsiders` capability.

- Only representations that make attributable decisions implement `Reconsiders`.
- Each implementation declares wake triggers and bounded candidate-query budgets.
- The scheduler indexes interested representations by event type instead of scanning all actors.
- Markets, facilities, legal instruments, and unrelated actors do not wake for every event.
- This implements the existing Clausewitz pulse, scheduled-review, Clowder persistence, and Representation Bible fidelity decisions.

### Chief of staff

The chief of staff is always a persistent `Person` at `NAMED_COGNITION`, bound through a dated `OfficeHolding` to the persistent chief-of-staff office.

- The person owns private beliefs, memory, judgment, relationships, dispositions, and attributable recommendations.
- The office grants access, agenda responsibility, confidentiality, and institutional duties, but not monetary-policy authority.
- Succession transfers office duties and institutional records, not private cognition or personal relationships.
- The current office-only catalog representation is incomplete and needs a bound person entry.

### Staff variation

Keep four systems separate.

- `Capability`: bounded professional expertise, often an unambiguous domain advantage.
- `Disposition`: judgment and salience tendencies with context-dependent benefits and costs.
- `Condition`: temporary workload, fatigue, exclusion, stale inputs, scrutiny, or similar effects.
- `Presentation`: the satirical label and rendering of the above mechanics.

Federal Reserve staff are institutionally competent. Variation affects speed, stopping thresholds, attention, confidence calibration, source weighting, and judgment under uncertainty. It does not grant hidden truth, legal authority, or inaccessible evidence.

## Institutional and campaign continuity

### Identity and presentation

Stable historical identity, institutional continuity, officeholding, succession, relationships, and legal temporality are canonical.

Presentation owns species, portrait, costume, voice, localization, and satirical rendering. A presentation change does not replace the represented person or institution.

A persistent institution or ruling house may span scenarios while different people hold its offices during different effective periods. The simulation reasons over stable IDs and dated relationships rather than inferring identity or authority from displayed text.

### Chair succession and campaign bounds

Campaign play continues through every valid Chair succession until the campaign's own bounded endpoint.

- Removal, resignation, incapacity, death, failed renomination, and ordinary term expiry all use the same succession path.
- Legal succession selects the incoming Chair.
- Player control binds to the incoming Chair.
- Institutional records, facilities, official commitments, unresolved proceedings, staff, and office-level consequences persist.
- Private beliefs, personal memory, person-owned commitments, and personal relationships do not transfer.
- Each chairmanship receives its own dossier; the campaign receives a final institutional dossier.
- Succession must not become a soft reset for damaged credibility, independence, legitimacy, capacity, or inherited conditions.

The prior Bible decision that final removal or term expiry ends play is superseded by this decision.

The campaign-level analogue for CK prestige is the Stewardship Score. It remains separate from credibility, legitimacy, independence, personal reputation, and every causal world-state variable. Its evaluative purpose is whether the player leaves the office capable, lawful, credible in its commitments, and able to meet future shocks and obligations.

### Stewardship score

The campaign has one explicit headline number that answers whether the player did a good job.
- The headline is an unbounded campaign total, not a normalized percentage or capped rating.
- Longer stewardship may create more opportunities to earn points; duration is part of the campaign record rather than normalized away.
- The total is signed and may rise or fall below zero.
- Awards and deductions require witnessed accomplishments, failures, incurred costs, or evaluated handoff conditions.
- Severe failures can erase earlier gains; accumulated tenure does not shield the score from later institutional damage.
- Removal from office remains a canonical legal and political event rather than a score threshold, and the campaign continues through succession.
- Being removed is not automatically a deduction: the score evaluates the stewardship facts that led to and followed the removal, because a Chair may be dismissed for a lawful but politically intolerable decision.

- The score is extradiegetic, non-spendable, and non-causal.
- It evaluates witnessed institutional stewardship rather than becoming reputation, credibility, legitimacy, independence, authority, or an actor belief.
- In-world actors never observe or react to the score.
- The underlying dossier preserves mandate outcomes, crisis performance, institutional capacity, legal and procedural integrity, future optionality, inherited conditions, incurred costs, and outstanding obligations.
- Activity alone earns nothing. Evaluation follows witnessed outcomes, proportionality, preparation, lawful execution, exit, and the condition handed onward.
- The number remains inspectable through its component evidence even though it provides one deliberately reductive headline verdict.

- Score deltas are presented when the player accepts a completed postmortem or comparable institutional review, not immediately after the underlying action.
- Each accepted review pairs an authored verdict such as `Awesome job, team` or `We have to tighten up` with the signed score delta.
- The review shows the headline delta first and retains an inspectable evidence ledger underneath.
- Later evidence does not rewrite an accepted delta silently; a supplemental or revised review records an explicit adjustment.
- Every delta is the sum of explicit authored postmortem findings evaluated against witnessed facts.
- Findings declare their evidence requirements and signed point values.
- Actions do not earn points by name; their witnessed performance, costs, proportionality, legality, exit, and resulting condition may satisfy findings.
- Do not run a continuous hidden utility formula behind the score.
- Standing institutional criteria and exact point values are visible from the beginning.
- A contingent criterion and its point value become visible when its subject is institutionally known; the scorecard never reveals an undiscovered subject.
- Score entries form an immutable ledger keyed to the accepted postmortem and its version.
- A later downward revision creates a new negative adjustment and removes points from the current total; it never rewrites the earlier entry.
- Upward revisions use the same compensating-entry rule.
- Accepting a postmortem means dispositioning the institutional record, not agreeing with every conclusion.
- The player may accept, attach a Chair response, request a specific evidence-backed revision, or accept and commission a supplemental review.
- An adverse score cannot be suppressed by refusing to close the review.
- Returning a report or commissioning more work consumes real capacity and may delay other packages, reports, or decisions.
- Any further score loss comes from witnessed obstruction, missed deadlines, displaced work, or degraded outcomes—not from a recursive penalty for disliking the score.
- Stewardship uses one global point denomination across every scenario and campaign.
- Authors select applicable findings, but a shared magnitude rubric constrains their signed awards and deductions.
- Scenario conditions may qualify the magnitude of an accomplishment or failure; scenarios may not invent incompatible scoring economies.
- Totals are broadly comparable without normalizing away campaign duration.
- Exact point-band values remain balancing work against representative postmortems.

## External providers

Reject a universal `ExternalProvider` interface.

- Define narrow channel-specific provider traits such as dollar funding, foreign financial stress, external demand, energy supply, or freight capacity.
- One provider may implement multiple channel traits.
- Each trait declares owned state, input and output schemas, units, timing, residuals, witnesses, consumers, and replacement contract.
- External providers emit boundary quantities, distributions, constraints, or observations. They do not directly set domestic prices or macroeconomic outcomes.
- Each provider declares either `RECORDED` or `RESPONSIVE` execution.
- A recorded provider consumes only scheduled external tape or incident inputs and is valid only when no player-reachable action feeds its channel.
- A responsive provider owns every behaviorally relevant state value and consumes declared interventions that can alter future outputs.
- Provider registrations declare their execution phase, reads, writes, and downstream invalidation edges through the same `WorkHandler` contract.
- Feedback follows the fixed forward-phase rule. A response may affect a later phase in the same period; feedback toward an earlier phase enters the next period unless the mechanism explicitly owns a bounded clearing loop.

Concrete period state, parameters, accounts, counterparties, and calibration remain content work for each selected provider.

## Information routing and authored content

### Evidence routing

Evidence routing uses institutional information, never omniscient significance.

- Canonical events become scoped observations through owned measurement and access paths.
- Routing sees only delivered artifacts, visible metadata, asserted urgency, deadlines, active commitments, case files, doctrine, and staff judgments.
- It does not receive true crisis identity, future damage, or canonical importance scores.
- Staff may miss connections, disagree, delay uncertain material, or prioritize the wrong accessible evidence without becoming generally incompetent.
- The player may change doctrine, monitoring, staff work, and priorities. These alter future routing without revealing hidden truth.

### Evidence access and reporting friction

Access controls always apply mechanically, but they become player-facing only when competent routine workflow cannot resolve a consequential conflict.

- Routine authorized task assignment creates any permitted purpose-bound access automatically.
- Forwarding a task or evidence reference does not broaden the source's access scope.
- Broader distribution requires an authorized grant, sanitization, aggregation, publication, or other witnessed transition.
- Nonroutine conflicts may require reassignment, a sanitized summary, restricted joint work, additional authorization, delay, or proceeding with incomplete evidence.
- Reporting difficulty ordinarily manifests as delay, uncertainty, reduced staff capacity, narrower evidence, stale inputs, or weaker confidence.
- Do not turn ordinary access administration into repeated player approval prompts.
- Surface the issue when the choice materially changes timing, evidence quality, awareness, confidentiality exposure, staff load, or institutional relationships.
- A hard decision deadline always produces multiple institutionally feasible options rather than a contentless refusal.
- Typical options include delivering a preliminary assessment now, narrowing the question, reallocating staff at an explicit opportunity cost, requesting a restricted or sanitized source, presenting conditional branches, or delaying only when procedure permits.
- Every option states its delivery time, uncertainty, missing evidence, capacity cost, displaced work, and known decision risk.
- When evidence remains insufficient, competent staff still provide bounded conditional judgments and explain what would distinguish the branches.

### Retrospective conclusions

In-world postmortems remain institutionally sourced rather than omniscient.

- Separate evidence known at decision time, accessible but missed, unavailable at the time, learned later, and still inaccessible or disputed.
- Every retrospective conclusion names supporting and contrary evidence, confidence, dissent, author, and evidence timing.
- Later evidence may revise the assessment but must be labeled as hindsight.
- Developer traces may inspect canonical causality; staff reports and player-facing postmortems may not.

### Primary player presentation

Decision packets, slide decks, and bounded conversation options in calls and interviews are the primary player surfaces.

- Each consequential decision receives a complete finite packet rather than competing for equal weight on a global dashboard.
- Packets foreground required decisions, material changes, staff recommendations and disagreements, affected commitments, evidence gaps, and inspectable source records.
- Slide decks present sequenced institutional argument and evidence rather than independent metric cards.
- Phone calls and interviews expose authored, context-specific options backed by typed commands or communication acts.
- Progressive depth runs from headline to brief, assessment and dissent, source record, then provenance and revision history.
- Routing controls attention, access controls what can be known, and the archive preserves every delivered artifact.
- Do not impose a universal numeric inbox cap. Control load through decision-centered bundling, institutional routing, and optional depth.

### Commit and submit

Effectful player choices use a working-folder and handoff workflow.

- While a meeting, call, interview, or decision folder remains open, the player may revise penciled choices freely.
- Penciled choices do not mutate canonical simulation state.
- Exception: an authored inquiry, question, or disclosure marked **commits on speaking** takes effect when selected and the turn passes to the counterpart. Its practical card is available before selection; its admission and subsequent delivery are separately witnessed. It cannot be penciled, secretly executed for a preview, or undone by closing the folder.
- The open folder shows the exact practical card for each current choice and for the combined slate.
- Closing and handing off the folder commits the complete slate atomically.
- Once handed off, the decision is canonical and its history is never deleted.
- A later cancellation, correction, amendment, or reversal must use the real lifecycle of the relevant instrument.
- A pending proposal may be amended prospectively through a new folder handoff. The original folder and witnesses remain, with a linked correction record; an already resolved institutional decision cannot be replaced by a Chair proposal.
- Navigation, inspection, filtering, and other non-effectful UI actions remain immediate.
- Do not introduce optimistic-revision conflict as an ordinary player-facing concern; deliberation occurs against the meeting's settled decision context.
- Do not add confirmation layers beyond the folder handoff unless an action has an unusual destructive consequence.

### Authored options and mechanical previews

- Conversation choices use complete authored lines, not runtime generation or sentence-fragment assembly.
- Each line binds to reusable typed actions, claims, commitments, and eligibility rules.
- Godot renders the authored line and sends only its selected option ID; Rust resolves the bound command.
- Hovering an effectful option shows a practical card naming the instrument or communication act that selection will invoke.
- The card must distinguish exact immediate mechanics from uncertain downstream effects.
- The exact section names the command, authority, recipients, timing, commitments, disclosures, capacity cost, and explicit non-effects.
- The assessment section presents expected downstream effects with institutional sources, confidence, uncertainty, and dissent; it never exposes canonical future outcomes.
- Language models may assist the offline authoring process, but committed lines and bindings pass ordinary review and catalog validation.

### Language models

Do not add runtime language-model integration or a speculative runtime renderer seam.

- Use structured simulation outputs plus reviewed authored templates and conditional passages.
- No runtime model chooses actions, updates beliefs, resolves mechanics, or writes player-facing prose.
- No model or network dependency belongs inside the executable game bounds currently envisioned.
- Offline language-model collaboration may help author catalogs and prose; the reviewed committed content remains authoritative.
- Reconsider runtime integration only if a concrete authored-content failure appears later.

### Asset pipeline

Use Godot's importer as the initial cooker.

- Track approved source assets and presentation references.
- Track provenance, usage rights, stable asset ID, dimensions, crop, focal point, and role.
- Validate references and asset properties.
- Pin Godot and import settings.
- Keep generated Godot import caches out of source control.
- Keep a presentation-build identity separate from simulation identity.
- Do not build a custom atlas or content-addressed cooker until measured loading, memory, or packaging needs justify it.

## Still open or deferred

- Numerical calibration budgets for each economic channel and scenario regime.
- Concrete state schemas and parameters for each selected provider.
- Period-specific counterparties, accounts, effective-period contracts, residual quantities, and opening-state research.
- Player-observation evidence showing that lessons transfer beyond memorizing a small fixture.
- Period-specific legal, institutional, and financial research required by selected mechanisms.
