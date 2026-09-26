# Reserves and operating regimes

**ID:** `design.economy.reserves-and-operating-regimes`
**Status:** `canonical`
**Depends on:** `design.economy.markets-clearing-and-prices`, `design.institutions.policy-portfolios`, `design.institutions.legal-regime`

The market for bank reserves is an explicit market with its own owners. The
Desk supplies nonborrowed reserves through open market operations. Depository
institutions subject to reserve requirements demand reserves against their
reservable liabilities at ratios that effective law sets. Banks may borrow at
the discount window on terms the lending Reserve Bank administers. The
overnight interbank rate clears that market; no owner writes it directly.

The Desk executes an operating regime that the authorized directive names.
A rate regime instructs the Desk to supply whatever nonborrowed reserves keep
the interbank rate inside a band. A reserves regime instructs the Desk to
supply a nonborrowed path consistent with target money growth and lets the
rate move within a wider tolerance band. Both are dated directive state with
explicit parameters, and changing regime is an ordinary directive decision,
never a scripted historical event.

Constituent actions keep their owners. The FOMC directs open market
operations. Reserve Banks propose discount rates, and the Board reviews and
determines them. The Board sets reserve requirements within statutory limits.
A package that combines these actions admits them together, but each is
decided, authorized, and executed by its own owner, and a refusal by one
owner leaves the others' results standing.

Coverage is legal state. Requirements bind only the institutions that
effective law covers, so movement of deposits or members outside coverage
changes reservable liabilities and weakens control of money growth without
any owner choosing that outcome.

Borrowing responds to the spread between the interbank rate and the discount
rate, bounded by administration of the window. Required reserves follow
reservable liabilities with a reporting lag. Money and reserve measures
reach the Chair only as dated, revisable releases, so the Chair observes the
effect of a regime through evidence, not through the market's internal state.

Exact historical rates and aggregates are not acceptance targets. Acceptance
requires that a rate regime smooths the interbank rate while letting money
growth drift, that a reserves regime tightens money control while increasing
rate variability, and that discount-rate and requirement actions change
borrowing and required reserves through their own channels.
