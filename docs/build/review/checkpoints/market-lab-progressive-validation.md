# Progressive market validation

**ID:** `evidence.checkpoints.market-lab-progressive-validation`
**Status:** `evidence`
**Depends on:** `evidence.checkpoints.market-lab-results`
**Verifies:** `design.economy.economic-fidelity-and-tolerances`, `mandate.evidence-obligations-m3-m4`

**Observed on the current working change:** `cargo test -p reservist-core --locked` reported 95 passing tests, including precondition classification, the 81-case invalid-state regression, isolated cash, futures, repo, and settlement behavior, heterogeneous distribution effects, and eight deterministic runtime slices.

The tiered validation suite passed declared gates from deterministic control through isolated primitives, paired mechanisms, the complete basis loop, ablation, heterogeneous ecology, policy timing, and runtime composition. Every interpreted treatment had valid preconditions, shock-free burn-in, identical pre-treatment state, complete causal parents, and exact reconciliation.

The dense sweep found initial validity beginning at leverage 13.51 and persisting through initial haircut 0.087; 0.088 and above was initially repo-infeasible. Every valid tested point amplified after the common 0.15 treatment haircut. In the valid stress fixture, the haircut increase caused one initial and four endogenous sales; removing haircut response removed all five. Other single-edge removals did not eliminate liquidation in that fixture.

Equal aggregate portfolios with concentrated terms produced four forced-sale units versus two under distributed terms. Eight runtime slices were deterministic, preserved evidence-before-decision order, hid canonical fund state, and retained causal reconstruction. All coefficients and thresholds remain provisional, not historical calibration.
