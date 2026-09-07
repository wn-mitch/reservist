# Markets, clearing, and prices

**ID:** `design.economy.markets-clearing-and-prices`
**Status:** `canonical`
**Depends on:** `design.economy.accounting-and-settlement`, `design.representation.firms-markets-and-mechanisms`

Participants propose desired orders or demand schedules from their own beliefs, constraints, plans, accounts, and authority. A market owns the order book, matching or clearing procedure, prices, fills, residual quantities, timing, and market-specific witnesses. It does not deliberate, read private motives, or let one participant directly set the price unless the mechanism is explicitly administered rather than a market.

A clearing result is not settlement. Fills produce typed obligations for account owners and settlement systems. Failed clearing may return residual quantities, rationing, or no price as a modeled output. Failed settlement preserves the prepared obligation and reason without partial account mutation.

Market scope is explicit by family, maturity or product bucket, venue, currency, eligibility, and period. Aggregation must preserve the risks and discontinuities relevant to the selected mechanism. Prices emerge from bounded participant behavior and constraints; content cannot assign a desired final domestic price through a provider or narrative event.

Exact historical prices are not an acceptance target. Directional effects, regime changes, ordering, bottlenecks, pivotal tails, accounting, and declared institutional thresholds are. Approximation error remains separate from keyed economic randomness.
