# Historical temporality

**ID:** `design.campaign.historical-temporality`
**Status:** `canonical`
**Depends on:** `design.runtime.scenario-selection`, `design.state.correction-and-invalidation`

Every scenario separates three temporal layers. `HistoricalPrefix` records
immutable events and provenance through scenario start. `OpeningSnapshot`
contains canonical state effective at the start. `FutureQueue` contains
obligations caused by prior history, recorded occurrences on no-feedback
external channels, and contingent events and decisions that may occur during
play.

History constrains identity, law, officeholding, institutions, accounts,
relationships, public claims, schedules, inherited projects, and known
conditions. It does not prescribe player-reachable decisions or downstream
outcomes. A historical episode supplies a researched starting regime, external
channel classifications, and mechanism hypotheses, not a script.

A past enactment, appointment, contract, or commitment may schedule a future effective date, term boundary, payment, review, or delivery. Valid future amendment, repeal, removal, succession, cancellation, performance, or breach can alter the prospective path. None erases the historical record or changes what was known earlier.

Scenario and campaign time remains deterministic and institutionally dated. Presentation may compress uneventful periods, but every causal transition retains its effective time, owner, and witness. Retrospective evidence learned later is marked as hindsight rather than inserted into the historical information set.

A reusable crisis template owns no rolling current truth. Each playable
instance freezes an opening instant, source cutoff, dated roster, provider
modes, event vocabulary, and content identity. Later real-world developments
produce a new dated instance or revision rather than mutating an existing run.
