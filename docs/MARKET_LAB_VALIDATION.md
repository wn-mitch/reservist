# Progressive Market-Simulation Validation

Suite: `progressive_market_validation`. Thresholds are provisional.

## 1. Experimental hierarchy implemented

- Tier 0: 1 experiments; gate passed: `true`.
- Tier 1: 5 experiments; gate passed: `true`.
- Tier 2: 4 experiments; gate passed: `true`.
- Tier 3: 1 experiments; gate passed: `true`.
- Tier 4: 2 experiments; gate passed: `true`.
- Tier 5: 1 experiments; gate passed: `true`.
- Tier 6: 1 experiments; gate passed: `true`.
- Tier 7: 1 experiments; gate passed: `true`.

## 2. Tier-by-tier commands and results

Run `reservist market-lab suite experiments/market_lab/validation_suite.toml` or `reservist market-lab tier <tier> experiments/market_lab/validation_suite.toml`.

- `tier0.deterministic_control`: passed `true`; control `STABLE_CONVERGENCE`; treatment `STABLE_CONVERGENCE`.
- `tier1.cash_treasury_absorption`: passed `true`; control `STABLE_CONVERGENCE`; treatment `ONE_TIME_ADJUSTMENT`.
- `tier1.futures_variation_margin`: passed `true`; control `STABLE_CONVERGENCE`; treatment `ONE_TIME_ADJUSTMENT`.
- `tier1.variation_margin_atomic_failure`: passed `true`; control `STABLE_CONVERGENCE`; treatment `SETTLEMENT_FAILURE`.
- `tier1.repo_capacity`: passed `true`; control `STABLE_CONVERGENCE`; treatment `STABLE_CONVERGENCE`.
- `tier1.settlement_atomic_failure`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `SETTLEMENT_FAILURE`.
- `tier2.cash_market_plus_repo`: passed `true`; control `STABLE_CONVERGENCE`; treatment `FIRE_SALE_AMPLIFICATION`.
- `tier2.futures_plus_fund_liquidity`: passed `true`; control `STABLE_CONVERGENCE`; treatment `ONE_TIME_ADJUSTMENT`.
- `tier2.market_plus_dealer_constraint`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `FIRE_SALE_AMPLIFICATION`.
- `tier2.funding_plus_settlement`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `SETTLEMENT_FAILURE`.
- `tier3.complete_basis_loop`: passed `true`; control `STABLE_CONVERGENCE`; treatment `FIRE_SALE_AMPLIFICATION`.
- `tier4.remove_leverage_constraint`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `FIRE_SALE_AMPLIFICATION`.
- `tier4.remove_variation_margin`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `FIRE_SALE_AMPLIFICATION`.
- `tier6.executed_policy`: passed `true`; control `FIRE_SALE_AMPLIFICATION`; treatment `FIRE_SALE_AMPLIFICATION`.

## 3. Initial-state and burn-in validity

Every passing experiment reports valid preconditions, shock-free burn-in, identical pre-treatment state, and exact reconciliation.

## 4. Primitive response findings

- `tier1.cash_treasury_absorption`: `STABLE_CONVERGENCE` → `ONE_TIME_ADJUSTMENT`; passed `true`.
  - `PriceDrawdown` `Greater`: 0 → 0.345; passed `true`.
  - `PeakResidualImbalance` `Greater`: 0 → 2; passed `true`.
- `tier1.futures_variation_margin`: `STABLE_CONVERGENCE` → `ONE_TIME_ADJUSTMENT`; passed `true`.
  - `FinalMarginHeadroom` `Less`: 60.58 → 58.5; passed `true`.
- `tier1.variation_margin_atomic_failure`: `STABLE_CONVERGENCE` → `SETTLEMENT_FAILURE`; passed `true`.
- `tier1.repo_capacity`: `STABLE_CONVERGENCE` → `STABLE_CONVERGENCE`; passed `true`.
  - `FinalRepoCapacity` `Less`: 966.28 → 838.1; passed `true`.
- `tier1.settlement_atomic_failure`: `FIRE_SALE_AMPLIFICATION` → `SETTLEMENT_FAILURE`; passed `true`.

## 5. Pairwise coupling findings

- `tier2.cash_market_plus_repo`: `STABLE_CONVERGENCE` → `FIRE_SALE_AMPLIFICATION`; passed `true`.
  - `TotalLiquidation` `Greater`: 0 → 3; passed `true`.
- `tier2.futures_plus_fund_liquidity`: `STABLE_CONVERGENCE` → `ONE_TIME_ADJUSTMENT`; passed `true`.
  - `FinalFuturesPositionAbs` `Less`: 10 → 8; passed `true`.
- `tier2.market_plus_dealer_constraint`: `FIRE_SALE_AMPLIFICATION` → `FIRE_SALE_AMPLIFICATION`; passed `true`.
  - `PeakRationing` `Greater`: 0 → 1; passed `true`.
- `tier2.funding_plus_settlement`: `FIRE_SALE_AMPLIFICATION` → `SETTLEMENT_FAILURE`; passed `true`.

## 6. Full basis-loop impulse responses

- `tier3.complete_basis_loop`: `STABLE_CONVERGENCE` → `FIRE_SALE_AMPLIFICATION`; passed `true`.
  - `EndogenousSales` `Greater`: 0 → 4; passed `true`.
  - `PriceDrawdown` `Greater`: 0 → 0.375; passed `true`.

## 7. Causal-ablation findings

- `tier4.remove_leverage_constraint`: `FIRE_SALE_AMPLIFICATION` → `FIRE_SALE_AMPLIFICATION`; passed `true`.
  - `TotalLiquidation` `LessOrEqual`: 5 → 5; passed `true`.
- `tier4.remove_variation_margin`: `FIRE_SALE_AMPLIFICATION` → `FIRE_SALE_AMPLIFICATION`; passed `true`.
  - `TotalLiquidation` `LessOrEqual`: 5 → 5; passed `true`.

Run `reservist market-lab ablate experiments/market_lab/tier3_full_stress.toml` for the eight-edge table.

## 8. Revised regime classifications

Classifications are trajectory-derived and emit predicates: invalid initial state, failed burn-in, failed reconciliation, stable convergence, one-time adjustment, damped recovery, persistent rationing, fire-sale amplification, funding-liquidity failure, settlement failure, and policy-assisted recovery.

## 9. Threshold and sensitivity results

- `amplification_ratio`: `0`.
- `basis_recovery_tolerance`: `0.01`.

Run `reservist market-lab sweep experiments/market_lab/dense_transition_sweep.toml` for the dense neighborhood.

## 10. Effects introduced by heterogeneity

Run `reservist market-lab ecology experiments/market_lab/ecology_concentrated.toml experiments/market_lab/ecology_distributed.toml`.

## 11. Policy timing and scale results

Run `reservist market-lab policy-ladder experiments/market_lab/tier3_full_stress.toml`.

## 12. Scenario-runtime integration results

Run `reservist market-lab runtime-compose experiments/market_lab/fed_purchase_executed.toml`.

## 13. Invariants and reconciliation totals

All passing manifest rows require both control and treatment reconciliation and complete causal parents.

## 14. Defects discovered and repaired

Invalid initial leverage and margin states, reference trades during burn-in, empty settlement envelopes, and missing margin-only futures adjustment were repaired in the owning mechanics.

## 15. Robust conclusions

The suite distinguishes execution correctness, isolated primitive coherence, and composed amplification evidence.

## 16. Provisional conclusions

All numerical coefficients and regime thresholds remain qualitative and provisional.

## 17. Failed or unvalidated tiers

Consult each tier gate and experiment failure list in the JSON result; a failed lower gate invalidates interpretation above it.

## 18. Next smallest justified mechanism

Calibrate heterogeneous fund and dealer distributions against an external empirical target before adding a default waterfall.
