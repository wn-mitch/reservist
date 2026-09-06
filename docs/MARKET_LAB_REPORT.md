# Treasury Basis Market Laboratory Report

Implemented and committed initially as `e239bd1f` — `feat: Add Treasury basis market laboratory`.

## 1. What changed

- Added the tooling-only laboratory domain in [`crates/reservist-core/src/market_lab.rs`](../crates/reservist-core/src/market_lab.rs).
- Added CLI commands in [`crates/reservist-cli/src/market_lab.rs`](../crates/reservist-cli/src/market_lab.rs):
  - `reservist market-lab run <fixture>`
  - `reservist market-lab sweep <experiment>`
  - `reservist market-lab compare <control> <treatment>`
- Added `just market-lab`, `just market-lab-sweep`, and `just market-lab-compare` recipes in [`justfile`](../justfile).
- Added eleven committed fixtures and one 81-case sweep under [`experiments/market_lab/`](../experiments/market_lab/).
- Kept the laboratory behind `test`/`tooling`; ordinary game and Godot projections cannot access laboratory state.
- Updated executable evidence and calibration limits in [`docs/BUILD_REVIEW.md`](BUILD_REVIEW.md).

## 2. Precise causal defect repaired

[`crates/reservist-core/src/scenario/handlers/staff_continuity.rs`](../crates/reservist-core/src/scenario/handlers/staff_continuity.rs) previously passed `self.package_id`—the proposed package—to intermeeting compression. Consequently, rejected `MEASURED_FIRMING` could receive the same inflation adjustment as an executed firming operation.

The release now derives its policy input from:

1. The witnessed execution receipt.
2. An `EXECUTED` status.
3. The corresponding authorized package.

[`crates/reservist-core/src/compression.rs`](../crates/reservist-core/src/compression.rs) treats absent execution as `NO_EXECUTED_POLICY`. Rejected and authorized-but-unexecuted cases retain proposal, authorization, execution, and reason records without receiving policy-caused macro effects.

The retained Python oracle received the same correction in:

- [`tools/oracle/engine/compression.py`](../tools/oracle/engine/compression.py)
- [`tools/oracle/engine/scenario.py`](../tools/oracle/engine/scenario.py)

Oracle vectors were regenerated; native/oracle parity remains exact across all 20 vectors.

## 3. Implemented market loop

Each laboratory period performs:

1. Apply scheduled, pre-existing shocks.
2. Observe prior cash and futures prices, balance sheets, funding, and margin.
3. Generate futures orders and clear through the production order-book algorithm.
4. Mark futures positions and settle symmetric variation margin.
5. Reprice repo capacity from collateral value and haircut.
6. Apply margin, repo, and leverage constraints.
7. Generate paired cash-Treasury and futures closeout orders when deleveraging binds.
8. Generate dealer, residual-buyer, and executed Fed orders.
9. Clear the cash Treasury market through `TreasurySecondaryMarket`.
10. Atomically settle cash and securities through `SettlementEnvelope`.
11. Repay repo principal and release collateral control from realized sale proceeds.
12. Reconcile cash, Treasuries, futures, repo claims, and collateral.
13. Emit period metrics, accounting entries, failures, shocks, and causal parents.

Prices come from participant limit orders and fills. Shocks alter funding terms, constraints, or order pressure; no package-to-price or shock-to-price table was introduced.

## 4. Participant and balance-sheet rules

The composition includes:

- Leveraged relative-value fund:
  - Long cash Treasury.
  - Short matching Treasury future.
  - Bilateral repo obligation.
  - Cash variation margin.
  - Haircut, margin, cash, and leverage constraints.
- Primary dealer:
  - Treasury inventory.
  - Explicit intermediation capacity.
- Residual cash buyer:
  - Explicit demand capacity.
- Repo lender:
  - Cash, repo claim, and collateral control.
- Futures clearing account:
  - Equal-and-opposite futures exposure.
  - Symmetric variation-margin cash flows.
- Fed Desk:
  - Treasury purchases only when both authorization and execution equal the required states.

The production accounting ledger and settlement envelope enforce balanced entries, available-balance checks, version checks, reservation rollback, and conservation.

## 5. Tests and fixtures

Regression and invariant coverage is in:

- [`crates/reservist-core/src/conformance_tests.rs`](../crates/reservist-core/src/conformance_tests.rs)
- [`crates/reservist-core/src/compression.rs`](../crates/reservist-core/src/compression.rs)
- [`crates/reservist-core/src/market_lab.rs`](../crates/reservist-core/src/market_lab.rs)

Committed scenarios include:

1. [`stable_baseline.toml`](../experiments/market_lab/stable_baseline.toml)
2. [`basis_widening.toml`](../experiments/market_lab/basis_widening.toml)
3. [`repo_haircut_increase.toml`](../experiments/market_lab/repo_haircut_increase.toml)
4. [`dealer_constraint.toml`](../experiments/market_lab/dealer_constraint.toml)
5. [`residual_buyer_retreat.toml`](../experiments/market_lab/residual_buyer_retreat.toml)
6. [`forced_fund_unwind.toml`](../experiments/market_lab/forced_fund_unwind.toml)
7. [`fed_purchase_executed.toml`](../experiments/market_lab/fed_purchase_executed.toml)
8. [`fed_purchase_rejected.toml`](../experiments/market_lab/fed_purchase_rejected.toml)
9. [`fed_purchase_unexecuted.toml`](../experiments/market_lab/fed_purchase_unexecuted.toml)
10. [`fed_purchase_control.toml`](../experiments/market_lab/fed_purchase_control.toml)
11. [`settlement_failure.toml`](../experiments/market_lab/settlement_failure.toml)

Observed:

- Stable baseline: six periods, stable convergence, no forced sales, residual imbalance, settlement failure, or reconciliation failure.
- Forced unwind: cash Treasury and short futures positions both contract; the cash price falls through sell orders and constrained demand.
- Executed Fed purchase: one unit fills, with balanced Fed cash and Treasury entries.
- Rejected and unexecuted comparisons: market state is identical to the matched control in every period.
- Settlement failure: insufficient available cash produces `FAILED_PREPARE`; no partial mutation occurs.

## 6. Reconciliation results

Every committed fixture passed final reconciliation:

- Cash conserved.
- Treasuries conserved.
- Futures exposure sums to zero.
- Repo claims reconcile.
- Collateral control sums to zero.
- Failed settlement remains atomic.
- Executed policy entries balance.
- Identical fixture and seed replay identically.

The 81-case sweep had **81/81 reconciliation passes**.

## 7. Sweep regimes

[`principal_sweep.toml`](../experiments/market_lab/principal_sweep.toml) spans:

- Leverage limits: 12, 18, 24.
- Haircuts: 5%, 10%, 15%.
- Dealer capacities: 1, 4, 8.
- Residual demand: 0, 2, 6.

Observed classifications:

- **18 stable-convergence cases**
- **63 fire-sale-amplification cases**

Threshold evidence:

- At a 5% haircut, both stable and amplification regimes occur.
- At 10% and 15%, every tested combination amplifies.
- A leverage limit of 12 amplifies throughout the neighborhood.
- Limits of 18 and 24 each retain nine stable cases.
- Lower dealer and residual capacity increases unresolved sell pressure or rationing.

## 8. Robust versus provisional

Robust within the tested neighborhood:

- Higher haircuts weakly increase required balance-sheet adjustment.
- Tighter leverage limits lower the threshold for forced closeout.
- Lower dealer or residual capacity weakly increases rationing or residual pressure.
- Higher margin requirements reduce available margin headroom.
- Unauthorized or unexecuted policy has no market or macro effect.
- Executed purchases change Fed holdings only through market fills and settlement.
- Conservation and atomicity hold across every case.

Provisional:

- Exact regime thresholds.
- Price-impact coefficient.
- Margin and repo calibration.
- Yield conversion.
- Dealer and residual demand magnitudes.
- The relative scale of variation margin versus repo-driven deleveraging.

These parameters are explicitly qualitative, not historical estimates.

## 9. Remaining architecture risks

- Futures liquidity currently uses one clearing-side counterparty rather than heterogeneous futures participants.
- No initial-margin default waterfall or mutualized clearing loss allocation exists.
- Repo is one bilateral channel; lender heterogeneity and maturity ladders remain absent.
- Futures closeout uses the shared matching and ledger mechanics but does not yet model exchange-specific execution rules.
- Regime names are deterministic classifications of observed mechanics, not externally calibrated financial-stability labels.
- The laboratory has no polished plotting layer by design.

## 10. Next smallest useful work package

Add explicit heterogeneous futures participants and an initial-margin/default-waterfall mechanism while retaining:

- The current fixture schema.
- The production accounting and settlement primitives.
- Paired control semantics.
- Existing reconciliation requirements.
- The period trace and sweep output.

## Commands executed

```text
cargo test -p reservist-core --locked
just oracle-test
just oracle-vectors
just parity
just market-lab experiments/market_lab/stable_baseline.toml
just market-lab experiments/market_lab/forced_fund_unwind.toml --format json --output target/market-lab-unwind.json
just market-lab experiments/market_lab/fed_purchase_executed.toml --format json --output target/market-lab-fed-executed.json
just market-lab experiments/market_lab/settlement_failure.toml --format json --output target/market-lab-settlement-failure.json
just market-lab-compare experiments/market_lab/fed_purchase_control.toml experiments/market_lab/fed_purchase_rejected.toml
just market-lab-compare experiments/market_lab/fed_purchase_control.toml experiments/market_lab/fed_purchase_unexecuted.toml
just market-lab-sweep experiments/market_lab/principal_sweep.toml --output target/market-lab-principal-sweep.json
just check
```

`just check` passed workspace build/tests, formatting, strict Clippy, dependency boundaries, 20-vector parity, 18 catalog tests, 86 retained oracle tests, scenario validation, and Godot boundary/runtime tests.
