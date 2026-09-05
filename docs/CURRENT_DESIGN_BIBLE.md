# Reservist: Current Design Bible (front door)

**Status:** Routing index for adopted H and accepted decisions D1–D6. Not a consolidated bible. The governing design is H, its accepted dispositions, and compatible inherited chapters under the precedence rules below. `docs/BUILD_REVIEW.md` records executable evidence; `docs/FABLE_IMPLEMENTATION_PLAN.md` allocates the M1/M2 work packages.

## What Reservist is

A turn-based, satirical institutional crisis simulator centered on the Federal Reserve Chair's office. The player holds one bounded node in an adaptive system of markets, institutions, political actors, population cohorts, media, and external shocks. Difficulty comes from judgment under partial information and institutional capacity, never wall-clock pressure. The world is a pure Rust core behind a thin Godot client; authored data configures compiled mechanics; the campaign continues through Chair successions and is scored by an extradiegetic Stewardship ledger. The working design spine is `docs/design/task.md`.

## Precedence

1. **H** — `docs/design/15-open-questions-decision-handoff.md`, as resolved by accepted D1–D6 in review Part G and their adopted amendment clauses. Governs every subject it explicitly decides or supersedes.
2. **Compatible inherited chapters**, in their own domains: D04 causal kernel, D05 representation, D06 catalog contracts, D09 media, D10 interface, D11 MVP cut. Where a chapter conflicts with H, H wins for that proposition only; the rest of the chapter stands.
3. **Reports** (D12, R13) describe what the Python prototype did. They never override design and never prove the new runtime.
4. **Unaccepted proposals** bind nothing. Accepted D1–D6 dispositions are binding; remaining reviewer suggestions and historical draft questions do not override them.

## Where each system is owned

| System | Owning text | Superseding or amending H clause |
|---|---|---|
| Causal contracts, owned state, stage witnesses, accounting | D04 `docs/design/04-design-discussion-minimum-simulation-kernel.md` (Foundational causal contracts) | none |
| Scheduling and time | D04 Hybrid task graph and cadence (line 267), Q4 (line 3062) | H Ordering and invalidation; Temporal cadence; Runtime dispatch (supersede the task graph, the elastic-calendar live clock, and weekly agenda framing) |
| Persistence and replay identity | D04 line 655; interchange line 2725 | H Rewrite and proof (relaxes cross-language byte parity only) |
| Campaign end and evaluation | D04 Term, failure, and legacy (line 2160); Q16 (line 3167) | H Chair succession and campaign bounds; Stewardship score (supersede "ends the playable role" and "no single victory score") |
| Representation kinds, fidelity tiers, invariants | D05 lines 158, 935, 1076 | H Fidelity traits; Actor growth; Identity and presentation |
| Chief of staff | D11 Q6 (line 507) | H Chief of staff (overrides: persistent Person at NAMED_COGNITION) |
| Catalog contracts, eligibility, manifest | D06; `catalog/schema.json`; `crates/reservist-content/src/manifest.rs` | H Catalog contract binding; Catalog and scenario distinctions; Validation and fallback; Identity aliases |
| Legal regime and temporality | D05 legal ownership; D04 | H Legal regime; Historical temporality |
| External channels and providers | D06; `catalog/external_channel*.csv` | H External providers |
| Media, claims, publication | D09 | H Language models (no runtime prose generation); Authored options |
| Interface, evidence, routing, postmortems | D10 | H Evidence routing; Evidence access; Retrospective conclusions; Primary player presentation; Commit and submit; Authored options |
| Runtime, client, repository | D14 (recommendations only; none resolved) | H Runtime selection; Client boundary; Repository; Asset pipeline (decide what D14 left open) |
| Economic fidelity | D04, D07 | H Opening conditions; Approximation tolerances; Transferable learning |

## Superseded clauses (historical, retained in place)

| Clause | Location | Superseded by |
|---|---|---|
| "Resignation, death, final removal, or term expiry ends the playable role." | D04 Q16, `04…kernel.md:3167-3172` | H Chair succession and campaign bounds |
| "It should not add these axes into a single victory score." / "never emits an overall grade" | D04 `04…kernel.md:2160-2180` | H Stewardship score (in-world review still has no verdict; see F-06) |
| Chief of staff at limited role-holder cognition | D11 Q6, `11…mvp-slice.md:507-510` | H Chief of staff |
| Deterministic event queue plus dependency-ordered task graph | D04 `04…kernel.md:267-334` | H Ordering and invalidation; Runtime dispatch |
| Elastic calendar over a live intermeeting clock | D04 `04…kernel.md:1996-2006`, `:3062-3068` | H Temporal cadence |
| Display-only runtime LLM rendering permitted | D04 `04…kernel.md:1783`; D14 Q9 | H Language models |
| D14 recommendations Q1, Q2, Q3, Q5, Q7, Q9 | `14…architecture.md:372-611` | H Runtime selection; Client boundary; Runtime dispatch; Repository; Asset pipeline; Language models |

## Owner dispositions

D1–D6 were accepted as recommended on 2026-09-05. Their binding text and consequences are in the review, Part G. The Build Mandate is authorized.

## Amendment history

- `docs/design/17-current-design-amendment-review-draft-v0_2.md` — owner's v0.2 draft (history).
- `docs/design/18-current-design-amendment-v0_3.md` — v0.3 with review corrections and reviewer proposals marked as such (current).
- `docs/design/16-agent-first-design-bible-workflow-v0_2.md` — the workflow this front door implements.
