# Actions, results, and events

**ID:** `design.state.actions-results-and-events`
**Status:** `canonical`
**Depends on:** `design.state.causal-and-stage-contracts`, `design.state.cognition-beliefs-and-plans`

A command is a typed request from an identified requester. An action is an attributable institutional choice or commitment with typed targets, eligibility, authority requirements, considerations, and expected effects. An action result answers the requester: accepted, rejected, unavailable, rationed, failed, or otherwise resolved. A domain event records a completed state transition. These are distinct records linked by stable identifiers and witnesses.

Role capabilities expose possible actions. Effective authorities determine permission. Responsible owners execute. Transactions and settlement entries prove conserved effects. A proposal identifier, label, or player intention cannot substitute for authorization or execution evidence.

`ScheduledEvent` identifies queued work becoming due. `ScenarioIncident` supplies a declared initiating fact or keyed draw. `DomainEvent` records something that happened through an owner transition. None is interchangeable with a narrative event, case file, or crisis flag.

Actions use a closed engine-owned vocabulary. Content may select and parameterize typed actions, predicates, and distributions, but cannot inspect arbitrary state or introduce executable behavior. Player-facing strategies and historical labels compose from primitives. Exact intermediate values remain inspectable to developer tooling without making canonical state visible to the player.
