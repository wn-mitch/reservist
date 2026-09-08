# External provider contracts

**ID:** `design.economy.external-provider-contracts`
**Status:** `canonical`
**Depends on:** `design.runtime.dispatch-and-activation`, `design.economy.opening-conditions-and-transmission`

External boundaries use narrow channel-specific provider traits such as dollar funding, foreign financial stress, external demand, energy supply, import supply, or freight capacity. There is no universal external-provider interface. One provider may implement several traits, but each trait separately declares owned state, input and output schemas, units, timing, residuals, witnesses, consumers, replacement behavior, phase, reads, writes, and invalidation edges.

Providers emit boundary quantities, distributions, constraints, or observations. They do not directly set domestic prices, actor beliefs, or macroeconomic outcomes. Receiving owners consume outputs through typed transmissions and ordinary mechanisms.

A `RECORDED` provider consumes only a scheduled external tape or declared incidents. It is valid only when no player-reachable action can feed back into that channel. A `RESPONSIVE` provider owns every behaviorally relevant state value and consumes every declared intervention capable of changing later output. Rich providers replace their declared aggregate share and reconcile residuals; they never layer on top.

For a Federal Reserve scenario, major foreign political exigencies may terminate at an authored boundary. A war, revolution, embargo, blockade, regime change, alliance action, or diplomatic rupture may be fixed in the historical prefix, opening snapshot, or future queue rather than produced by a simulation of the foreign political system. Its material, financial, institutional, and informational consequences still enter through ordinary owners and typed transmissions. A responsive provider may vary only within its declared posture and transition vocabulary. The runtime cannot invent a new conflict, regime, authority, or geopolitical relationship because omitted politics drifted offscreen.

The Chair affects an external political path only through an explicitly selected and researched feedback channel. Advice, coordination, swap arrangements, sanctions implementation, public claims, or other U.S. actions do not move foreign politics merely because they are salient. When no player-reachable action can alter the exigency, recording it is an honest aggregation boundary rather than a loss of required agency.

Feedback follows fixed forward phases. A response may enter a later phase in the same period; feedback to an earlier phase enters the next period unless one mechanism owns a bounded loop. Concrete period state, accounts, counterparties, and calibration remain provider content.
