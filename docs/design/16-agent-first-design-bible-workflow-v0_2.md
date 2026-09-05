# Reservist: Agent-First Design Bible and Build Workflow

**Document ID:** `RDW-2026-09-05-01`  
**Version:** `0.2-agent-first-proposal`  
**Prepared:** September 5, 2026  
**Replaces:** The workflow recommendation in `Reservist_Living_Design_Bible_Workflow.md`, version 0.1. Preserve that version as history, not a competing current instruction.  
**Companion:** `Reservist_Current_Design_Amendment_Review_Draft_v0_2.md`.  
**Status:** The agent-first operating premise below is user-supplied. The concrete workflow is a proposal for testing that premise, not an established method or an amendment to the game's mechanics. No repository reorganization, agent execution, or Rust/Godot implementation is claimed here.

> Repository note (added at import, 2026-09-05): this file is the owner-supplied v0.2 text checked in verbatim as history. The `Reservist_*` filenames it references were never created on disk; the companion amendment is `17-current-design-amendment-review-draft-v0_2.md` and its corrected successor is `18-current-design-amendment-v0_3.md`. The front doors it proposes exist as `docs/CURRENT_DESIGN_BIBLE.md`, `docs/BUILD_MANDATE.md`, and `docs/BUILD_REVIEW.md`.

## 1. Operating premise

Reservist is a **vibe-code-first project**. The human design owner primarily reads design documents, examines implementation shape, and plays the result. Routine source-code inspection, test implementation, repository maintenance, and document reconciliation belong to development agents. A workflow that ultimately requires the owner to audit ordinary source diffs or maintain many synchronization tables has failed this operating premise.

The intended interaction is: finish the design review, disposition material choices, then authorize a coding agent to build the accepted bible. That authorization should support substantial implementation work without requiring the human to issue every task or approve every internal milestone.

**Source review is delegated, not eliminated.** Implementation and verification agents must inspect actual code and execution evidence. Their human-facing report must make the important conclusions inspectable without requiring the owner to repeat their source investigation.

This is an experimental division of labor. Documentation, automated checks, and additional model review can all be wrong. The workflow should expose disagreements and unproved claims and should itself be evaluated during the build. It is not a guarantee that an arbitrarily large bible can be implemented correctly in one model invocation.

## 2. Three front doors, not a documentation job for the owner

Give the human and a newly arriving development agent three stable entry points. These can be three files or clearly separated sections; the responsibilities matter more than the filenames.

| Surface | Question answered | Maintenance responsibility |
|---|---|---|
| **Current Design Bible** | What are we building, why, and which constraints must survive implementation? | Agents integrate accepted decisions. The human reviews material design changes. |
| **Build Mandate** | What work is authorized now, what may the agent decide, and what establishes completion? | Agents prepare and maintain it under the owner's scope authorization. |
| **Build Review** | What does the current executable actually do, how does its shape compare with the bible, and what remains unproved? | Implementation agents supply evidence; a verification pass inspects the code and relevant runs. The human reviews the conclusions and playable result. |

Do not make the human navigate decision registers, source inventories, requirement matrices, task ledgers, and historical exports to establish the current design. Those remain available to agents and reviewers behind the front doors.

Keep **desired design** and **observed implementation** distinguishable even when presented side by side. The bible governs intent. The executable establishes current behavior. A report links them; it cannot make them agree by rewriting either side's history.

### The bible's reading shape

Begin with a coherent explanation of the game and its causal architecture. Follow it with full current contracts for the runtime, representation, economy, institutions, evidence, interface, campaign, and selected acceptance scope. Preserve required exceptions, negative rules, failure modes, and worked examples; a polished summary is not a replacement for those details.

A reader should be able to move from an overview of a system to its ownership and state transitions, then to exact contracts and provenance. The human can stop at the depth needed to judge shape. The implementing agent must follow the relevant contracts to their dependencies.

Existing owner chapters can supply these sections. The user should experience one logical bible, not a collection of unresolved amendments. Agents maintain the source chapters and generate the reading package. A single physical source file is also acceptable while manageable; splitting it is a maintenance choice, not another required architecture project.

The historical omnibus remains an audit archive. It is not a second instruction stream with equal authority to the current bible.

## 3. Give the bible enough structure to authorize work

For each consequential system, the current text should answer the following questions. Use ordinary prose and compact tables; do not require a separate database record for every sentence.

| Contract facet | What an implementing or verifying agent needs |
|---|---|
| Purpose and player effect | What the mechanic contributes and how its consequences can become legible. |
| Ownership and boundaries | Canonical owners, non-owners, information access, and producer/consumer responsibilities. |
| Lifecycle | Inputs, commands, transitions, delayed work, outputs, failure classes, and release conditions. |
| Invariants and forbidden shortcuts | What must remain exact or separate, including plausible but false implementations. |
| Acceptance | Representative successful and adverse cases, required traces, persistence obligations, and limits of the proof. |
| Latitude and maturity | What is adopted, what the agent may choose, what is a working mechanism, and what requires evidence or a decision. |

Retain stable references for load-bearing contracts. Agents maintain their mappings to code, schemas, fixtures, and evidence. The human should not have to assign or reconcile these IDs manually.

Machine-readable contracts supplement the bible; they do not silently replace a normative distinction with whatever the current schema happens to permit. Conversely, the implementation need not mirror the prose's heading structure or create a Rust type for every noun.

### Example: capacity commitments

The specification can require that a staff assignment occupies dated capacity until its modeled release, can displace other work, and survives save/resume. Its shape description identifies the commitment owner, scheduler, resource owner, deliverable, and player projection.

A useful implementation demonstration would show an assignment being accepted, a competing request being delayed or narrowed, the reservation persisting through a checkpoint, and completion releasing capacity once. A negative case would show that reopening a folder or replaying a delivery does not reserve or release the resource twice.

A sentence saying “capacity commitments implemented,” a matching type name, or a diagram copied from the specification would not establish those facts. This is an example of the required evidence form, not a claim that the demonstration currently exists.

## 4. Put the maintenance and execution burden on agents

| Participant | Responsibilities |
|---|---|
| **Human design owner** | Sets the intended game, approves material design and scope changes, resolves consequential tradeoffs, examines shape reports, and evaluates the playable experience. Can respond in ordinary language rather than editing registers. |
| **Integration/build agent** | Retrieves the accepted sources, checks the actual baseline, integrates decisions, plans dependencies, implements code and content, creates and runs tests, fixes defects, updates documentation, and preserves continuation state. |
| **Verification role** | Reads the requirements and actual repository, checks changed and neighboring boundaries, runs relevant cases where available, challenges assertions and test adequacy, and reports discrepancies with evidence. |
| **Mechanical tooling** | Produces reproducible compilations, registry views, schema checks, traces, test output, and exports within its actual capabilities. It does not supply unearned semantic approval. |

These are responsibilities, not a requirement for a separate agent or artifact at every step. A distinct verification context is useful, but two models agreeing is not independent proof. Verification should examine source and runs, not merely assess whether the builder's summary sounds consistent.

Agents perform the routine bookkeeping: update the owning sections, record actual dispositions, maintain references, retire stale operative wording, regenerate exports, and report remaining contradictions. Do not return “remember to update the bible” as homework after completing the code.

## 5. Bounded autonomy: enough freedom to finish the build

The Build Mandate pins the accepted design revision or source snapshot and states the authorized executable target. It is an execution brief, not a second specification. Internal checkpoints may organize a large task without becoming requests for permission to continue.

| Issue encountered | Expected behavior under the proposed mandate |
|---|---|
| Local coding, module organization, helper types, or test organization consistent with accepted boundaries | Decide, implement, test, and report material implementation choices without interrupting the owner. |
| Existing design question already answered elsewhere | Retrieve the controlling clause, apply it, and repair stale documentation. Do not ask the owner again. |
| Working coefficient or algorithm behind a settled contract | Research or exercise it as appropriate. Use provisional values only where the mandate permits, identify their scope, and do not claim calibration. |
| A proposed change to gameplay, ownership, legal authority, evidence access, persistence, approved technology, or completion scope | Describe the exact conflict and proposed design delta. Obtain owner disposition unless a specific choice has already been delegated. |
| Missing input, unavailable tool, or genuine unresolved dependency | Record the precise limitation and affected work. Continue independent work without fabricating evidence or replacing the missing behavior with a misleading stub. |
| A failing test that enforces accepted behavior | Fix the implementation. Changing the requirement, deleting the case, or weakening the assertion needs a substantive justification and the relevant authorization. |

Collect consequential questions into a short decision packet where dependencies permit. Do not make the human answer a stream of routine engineering questions. Do not silently choose economic, legal, or product facts merely to avoid all questions.

“Build the bible” means execute the authorized program of work against the bible. It does not mean stop after a scaffold or a plan. It also does not make every speculative inventory entry an immediate implementation requirement. The mandate must distinguish the complete Python executable replacement, applicable adopted changes, and later campaign/world work without allowing the builder to silently narrow or enlarge that scope.

## 6. Return a reviewable implementation, not just a completion narrative

At a coherent integration checkpoint or final handoff, the Build Review should make the result understandable in design language. Reuse actual evidence rather than creating a new essay after every small edit.

| Review component | Required content |
|---|---|
| **Result and runnable path** | What now works end to end, how to launch it, which fixture or scenario is demonstrated, and any actual build/environment limits. |
| **Shape comparison** | Intended versus implemented ownership, module boundaries, event/data flow, authority, information access, and persistence. Show meaningful changes from the last reviewed shape. |
| **Behavioral evidence** | Identified runs, command sequences, stage receipts, conservation checks, adverse paths, and checkpoint/resume comparisons appropriate to the work. |
| **Deviations and uncertainty** | Missing behavior, provisional mechanisms, failed or unrun checks, limited fixture coverage, stale evidence, and unauthorized design changes requiring disposition. |
| **Decision request, when necessary** | The exact choice, evidence, alternatives, consequences, and blocked work. Not an undifferentiated backlog dump. |

Source paths and symbols belong underneath the report for the verifier and for optional inspection. They should not be the explanation the owner must decipher. Technical review at the design level can still be precise: state schemas, ownership tables, transition sequences, practical-card examples, and trace excerpts are appropriate.

### Where shape evidence comes from

Label a diagram or table according to its basis: intended design, registered metadata, inspected implementation, or observed execution. A view generated from `WorkHandler` declarations establishes what was registered; it does not by itself establish that every code path obeys those declarations. Pair it with enforcement checks, relevant source review, and exercised paths.

Screenshots and playthroughs establish visible behavior, not hidden accounting or access correctness. Unit tests establish their actual assertions, not every clause in the bible. Preserve these limits without requiring the human to review every low-level test.

The verification role should derive cases from the accepted requirements, including forbidden shortcuts, rather than accepting the implementation's current outputs as correct by default. Where useful, perturb inputs or deliberately violate a boundary to check that the expected mechanism and diagnostic actually operate.

### Completion is relative to evidence and scope

An integrated playable path, valid frozen content, exercised failures, and the required deterministic continuation evidence matter more than file counts or a large green test total. A scripted happy path cannot substitute for a promised live mechanism. An untriggered contract cannot be reported as exercised.

Keep static conformance, executed behavior, economic calibration, and human-observed comprehension distinct. No documentation generator or model sign-off can prove all four at once.

## 7. Keep intended design stable while reporting as-built facts honestly

The builder must not silently update normative prose to match an accidental implementation. Implementation discoveries enter the Build Review as deviations or proposed changes. Once a change is accepted, the integration agent patches the authoritative chapter and all dependent contracts as part of the same coherent change set.

The agent records the actual human disposition. “Proceed with your recommendation on these three points” can be sufficient explicit authorization; the owner need not edit each register row. Review recommendations that were not adopted stay proposed. An unresolved design choice cannot disappear merely because the next agent did not retrieve it.

Reports identify the source, content, executable, and fixture revision they actually concern. A change to an affected contract makes its old proof stale until checked again. Unaffected evidence can remain useful with its scope stated. Do not refresh a timestamp and treat an old run as proof of new behavior.

Generate the human reading bible from current owned text, not by repeatedly asking a model to resummarize the preceding summary. Agents may revise the source text, but consequential omissions and semantic changes must remain reviewable. Preserve the historical corpus and its original identities separately.

This arrangement has one editorial authority, one observed implementation, and evidence connecting them. It does not require pretending that a prose bible is an executable specification or that a compiled schema proves the intended experience.

## 8. Make a fresh agent able to continue without the human reconstructing the project

The agent entry point should route to the current bible, the active mandate, and the latest Build Review. A short repository instruction file can carry that routing; do not duplicate the game's full rules there.

The agent-maintained continuation record needs the actual baseline and working-tree state, completed and remaining obligations, current commands and results, pending owner decisions, known failed approaches, and the next concrete executable step. Keep it compact and linked to evidence. Record uncommitted work rather than inventing a clean revision.

A fresh agent should verify those pointers and continue the active work, not restart architecture discovery from all historical artifacts. A handoff should identify unfinished work honestly and preserve it; it is not a promise that a stopped agent will continue running elsewhere.

For model-sized bundles, use a stable index and explicit cross-references. The global architecture, invariants, active scope, and affected producer/consumer contracts must travel with a local implementation task. Do not omit a critical exception to save context or require the owner to curate a new bundle for each module.

## 9. Apply this after the Fable review

The review should return material findings, proposed replacement clauses, and a concise list of decisions that genuinely require the owner. It should also identify what an implementing agent can safely decide without further design review. Do not classify every unwritten function or parameter as a blocking design question.

After the owner's disposition, an integration agent should produce the consolidated current bible and a Build Mandate. It should perform the source reconciliation and export work, not return instructions for the owner to perform it manually. Keep the original amendment and review as history.

The Build Mandate should contain, in a compact form, the accepted baseline, executable completion target, binding constraints, permitted implementation latitude, known blockers, required evidence, and continuation policy. It should directly authorize implementation, testing, repair, and document maintenance once launched. No special documentation platform is a prerequisite: existing Markdown, repository history, a small index, and available test/build tools are enough to start this experiment.

### Build authorization template

Use this only after the relevant design decisions and execution scope have actually been dispositioned. The referenced files are proposed repository entry points, not files claimed to exist now.

```text
Build Reservist according to the accepted Current Design Bible and active Build
Mandate. Treat their pinned source snapshot and scoped authority as controlling.
Read the relevant full contracts, then inspect the actual repository baseline.

Execute the authorized implementation program, not just a plan or scaffold.
Own internal decomposition, code and content changes, tests, debugging, source
reconciliation, and documentation maintenance. Do not require me to author each
task or review routine source diffs. Continue through internal milestones within
the authorized scope without requesting permission at every step.

Choose ordinary engineering details within the accepted contracts. Do not silently
change gameplay, causal ownership, information boundaries, legal authority,
persistence, approved technology, or the agreed completion scope. Batch genuine
design decisions with evidence and alternatives; continue unaffected work.

Maintain a readable intended-versus-as-built shape report backed by the actual
implementation and identified executions. Preserve the distinction between
passing checks, exercised behavior, calibration, and player comprehension.
Do not weaken tests, fabricate results, or replace live mechanics with scripted
outcomes to claim completion.

Perform a verification pass against the source and runs, not merely your summary.
Return the runnable result, a concise design-level change report, evidence and
limits, and any real decision required from me. Maintain the current bible and
continuation state so another agent can resume without reconstructing the project
from our conversation. If blocked or stopped, identify precisely what is done and
what remains; do not label partial completion as completion of the mandate.
```

This template specifies responsibilities. It does not establish any model's ability to finish the entire task in one session.

## 10. Evaluate the workflow as part of the experiment

The experiment is whether this documentation and evidence boundary lets the owner direct a complex build without becoming its routine source reviewer. Use existing review records to observe a few things rather than building a new metrics system first.

Did human interventions concern real design choices, or missing bookkeeping? Could the owner spot a material shape or behavior discrepancy from the review and demonstration? Did a fresh agent continue from the recorded state without reopening settled choices? Did the verification pass find problems that the builder's tests or summary missed? Did maintenance cost grow mainly with meaningful changes or with the number of accumulated documents?

A useful stress test is a deliberately introduced representative defect: wrong-owner mutation, evidence leaking into a projection, or a duplicated capacity release. Check whether the technical verification catches it and whether the human-facing report explains its consequence. Passing that test establishes only the tested cases, not universal reliability.

If the human repeatedly has to read source to discover material deviations absent from the report, improve the report, enforcement, or verification method. If the process demands continual permission for ordinary coding, improve the delegation boundary. If all reports look green while the game violates the intended behavior, treat that as workflow failure rather than proof that another summary is needed.

## 11. Source basis and governing rule

**Accepted operating premise:** the user's clarification that Reservist is vibe-code-first, that the human will primarily inspect design documents and implementation shape, and that the intended next step is a broadly authorized agent build after review.

**Accepted game/technology constraints:** `Open Questions Decision Handoff`, reproduced in Section 3 of `Reservist_Current_Design_Amendment_Review_Draft_v0_2.md`.

**Previous workflow being revised:** `Reservist_Living_Design_Bible_Workflow.md`, version 0.1. Its separation of current intent, history, and revision-scoped evidence is retained. Its implicit allocation of integration and routine maintenance is replaced with explicit agent responsibilities and a small human-facing surface.

The specific operating mechanism in this document remains proposed. It must not be added to the game's canonical simulation rules as though documentation responsibilities were in-world mechanics.

**The human directs the design and judges the experience. Agents own implementation, source inspection, reconciliation, and proof production. The bible and Build Review make that delegation inspectable without pretending it is infallible.**
