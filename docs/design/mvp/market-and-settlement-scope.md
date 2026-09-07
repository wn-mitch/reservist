# Market and settlement scope

**ID:** `design.mvp.market-and-settlement-scope`
**Status:** `canonical`
**Depends on:** `design.economy.markets-clearing-and-prices`, `design.economy.accounting-and-settlement`

The MVP contains one bounded Treasury maturity-bucket market and one bilateral secured-funding relationship. A primary-dealer cohort, leveraged-fund cohort, and external-buyer residual submit constrained demand or orders. The market owns matching, price, fills, and residual quantities. Participant accounts own cash, Treasuries, financing, collateral, margin, and resulting obligations.

Repo state records counterparties, collateral, cash, haircut, maturity, capacity, rollover, and settlement. Authorization, market fill, funding availability, collateral control, and final settlement remain separate. Failed preparation or settlement is atomic and leaves accounts unchanged except for an explicit failure record.

The market is sufficient to transmit mild intermediation pressure and policy execution evidence through the selected cycle. It is not the complete Treasury curve, dealer network, derivatives ecology, banking system, or real economy. Omitted detail enters only through declared residuals and adapters compatible with the selected contracts.

Accounting conservation, legal ownership, deterministic order, and witnesses are exact. Coefficients and prices are scenario parameters under declared tolerances; the fixture does not claim historical calibration beyond its sourced opening regime and directional mechanism.
