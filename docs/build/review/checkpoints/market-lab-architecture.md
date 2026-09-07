# Market laboratory architecture

**ID:** `evidence.checkpoints.market-lab-architecture`
**Status:** `evidence`
**Depends on:** `evidence.current-runtime`
**Verifies:** `design.economy.accounting-and-settlement`, `design.economy.markets-clearing-and-prices`

**Inspected on the working change after M2:** `reservist-core::market_lab` is tooling-only composition over the production Treasury order book, accounting ledger, and atomic settlement envelope. Ordinary sessions and Godot projections cannot access laboratory state.

Each period applies scheduled shocks, observes prior prices and balance sheets, clears matching Treasury futures, settles symmetric variation margin, reprices bilateral repo capacity, applies margin and leverage constraints, generates forced paired cash and futures closeout, clears cash Treasuries with dealer, residual, and eligible Desk orders, settles cash and securities atomically, repays repo from realized proceeds, releases collateral control, and reconciles cash, Treasuries, futures, repo claims, and collateral.

Prices come from participant limit orders and fills. Shocks change terms, constraints, or order pressure rather than setting prices. Desk purchases enter only when both authorization and execution are recorded. Intermeeting macro input now derives from the witnessed executed package; rejected and authorized-but-unexecuted proposals retain records but have no policy-caused macro effect.

The composition includes one leveraged fund, dealer, residual buyer, repo lender, futures clearing account, and Desk. Parameters and participant breadth remain provisional.
