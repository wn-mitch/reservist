# Accounting and settlement

**ID:** `design.economy.accounting-and-settlement`
**Status:** `canonical`
**Depends on:** `design.state.canonical-state-and-ownership`, `design.state.causal-and-stage-contracts`

Accounts and canonical owners carry cash, deposits, reserves, securities, loans, repo, equity, inventories, claims, collateral, obligations, and other conserved positions. Every movement uses balanced typed entries with currency, unit, amount, source and destination accounts, effective time, authorization, and witness. Consolidated balance sheets and exposure views remain derived.

Preparation, authorization, execution, clearing, and settlement are separate stages. A prepared transfer may fail without partial mutation. Market fills create obligations but do not prove settlement. Collateral eligibility, title, encumbrance, margin, haircut, maturity, priority, and counterparty exposure remain explicit contract dimensions rather than labels inferred from an instrument name.

Physical and financial state are coequal canonical truth. Production, consumption, loss, transformation, transport, and title transfer reconcile product-specific quantities. A relationship or region never owns inventory by implication. Corrections use compensating entries and preserve the original record.

Hard conservation, legal ownership, and exact institutional thresholds are invariant. Economic inability such as insufficient available cash or collateral is a typed result. Unbalanced entries, double ownership, silent title transfer, and partial failed settlement are defects.
