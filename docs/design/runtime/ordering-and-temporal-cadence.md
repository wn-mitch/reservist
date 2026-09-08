# Ordering and temporal cadence

**ID:** `design.runtime.ordering-and-temporal-cadence`
**Status:** `canonical`
**Depends on:** `design.state.correction-and-invalidation`

Simulation work moves through a fixed forward phase roster. Each handler declares reads and writes; those declarations determine local ordering and deterministic commit barriers. Concurrent calculations consume versioned snapshots and write private result buffers. Canonical owners commit stable-sorted batches. Two handlers may not ambiguously write the same state. Feedback toward an earlier completed phase enters a later tick or period unless one owner explicitly defines a bounded clearing loop.

Player time uses a visible calendar board, never a real-time clock or speed controls. Dates, deadlines, institutional anchors, future obligations, and interruptions remain visible. The player authorizes discrete advancement. A span between anchors offers finitely many discretionary activity periods, not one required turn per date. Uneventful dates may resolve without empty turns.

Activities reserve concrete dated capacity and may complete now or schedule later artifacts, meetings, and consequences. Reading delivered material does not consume capacity or advance time. Scheduled briefing cycles are the ordinary cadence. Observed calls, emergencies, or deadlines may interrupt before the next briefing, but hidden canonical significance never triggers a stop.

Briefings summarize trends and accumulated changes. Genuine surprise remains possible when evidence was inaccessible, misleading, unmonitored, or not institutionally escalated.

Canonical event time is one UTC instant. Institutions, venues, assets, and
player offices interpret it through versioned local calendars, holidays,
sessions, information cutoffs, and settlement rules. Daily accounting and
physical settlement may coexist with timestamped intraday announcements,
clearings, incidents, and interruptions. Long-horizon processes update at
their declared natural cadence rather than whenever the global clock advances.
A later player role changes its local view and briefing cutoff, never the
world's clock or causal order.
