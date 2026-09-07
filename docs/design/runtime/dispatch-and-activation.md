# Dispatch and activation

**ID:** `design.runtime.dispatch-and-activation`
**Status:** `canonical`
**Depends on:** `design.runtime.ordering-and-temporal-cadence`, `design.state.actions-results-and-events`

Executable mechanics use a trait-driven static Rust registry. Each `WorkHandler` declares associated input and output types, a stable handler key, phase, reads, writes, and witnesses. Registration produces a monomorphized erased function pointer for heterogeneous lookup. The registry does not store heap-allocated entity trait objects, runtime scripts, general plugins, or linker-discovered registration magic. Adding mechanics requires compiled code.

Activation follows represented kind and lifecycle; a catalog entry is not a generic runtime job. Policy packages activate through decisions and emit constituent commands. Facilities pass through authorization, preparation, opening, counterparty use, and wind-down. Legal clauses change through dated legal transitions. Markets run when orders or scheduled clearing require them. Mechanical systems process owned queues. Deliberating representations reconsider only on declared evidence, thresholds, plan decision points, or scheduled reviews.

Handlers must be total over valid frozen inputs. Event subscriptions index interested representations rather than scanning every entity. Markets, facilities, legal instruments, and unrelated actors do not wake for every observation. Generated registration metadata may render flows, but executable declarations and boundary validation govern dispatch.
