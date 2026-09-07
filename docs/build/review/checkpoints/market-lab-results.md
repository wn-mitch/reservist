# Market laboratory results

**ID:** `evidence.checkpoints.market-lab-results`
**Status:** `evidence`
**Depends on:** `evidence.checkpoints.market-lab-architecture`
**Verifies:** `design.runtime.defect-vs-economic-failure`, `design.economy.accounting-and-settlement`

**Observed on the market-laboratory working change:** the stable baseline ran six periods with stable convergence and no forced sale, residual imbalance, settlement failure, or reconciliation failure. The forced-unwind fixture contracted both cash Treasury and matching short futures positions; participant orders reduced the cash price.

An executed Desk purchase filled one Treasury unit with balanced Fed cash and security entries. Rejected and authorized-but-unexecuted cases matched the no-execution control market state in every period while retaining proposal, authorization, execution, and reason records. Insufficient cash produced failed preparation with no partial mutation.

Every committed fixture reconciled cash, Treasuries, zero-sum futures exposure, repo claims, collateral control, failed-settlement atomicity, and executed-policy entries. Identical fixture and seed replayed identically. The first 81-case principal sweep reconciled every case, but later precondition-aware analysis showed 63 of those starts were initially infeasible rather than valid amplification evidence.

Robust software findings are conservation, atomicity, execution gating, and deterministic traces. Price impact, margin and repo coefficients, thresholds, yield conversion, participant distributions, and policy magnitudes remain qualitative and provisional.
