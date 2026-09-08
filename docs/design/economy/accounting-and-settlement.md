# Accounting and settlement

**ID:** `design.economy.accounting-and-settlement`
**Status:** `canonical`
**Depends on:** `design.state.canonical-state-and-ownership`, `design.state.causal-and-stage-contracts`

Accounts and canonical owners carry cash, deposits, reserves, securities, loans, repo, equity, inventories, claims, collateral, obligations, and other conserved positions. Every movement uses balanced typed entries with currency, unit, amount, source and destination accounts, effective time, authorization, and witness. Consolidated balance sheets and exposure views remain derived.

Preparation, authorization, execution, clearing, and settlement are separate stages. A prepared transfer may fail without partial mutation. Market fills create obligations but do not prove settlement. Collateral eligibility, title, encumbrance, margin, haircut, maturity, priority, and counterparty exposure remain explicit contract dimensions rather than labels inferred from an instrument name.

Physical title, custody, transformation, delivery, invoicing, payment, public
claims, reserve availability, and portfolio use are distinct transitions. A
carrier, terminal, warehouse, processor, or transformation mechanism does not
acquire inventory by custody alone. Contracts name transfer points and may
allocate output among several title-bearing accounts. Prepayment, credit,
escrow, netting, blockage, default, taxes, royalties, dividends, and retained
earnings follow their own balanced entries.

Physical and financial state are coequal canonical truth. Production, consumption, loss, transformation, transport, and title transfer reconcile product-specific quantities. A relationship or region never owns inventory by implication. Corrections use compensating entries and preserve the original record.

Period accounting reconciles physical output and contract prices through
operator accounts, government claims, balance-of-payments entries, central-bank
reserves, bank settlement, portfolio flows, and declared statistical
discrepancies. Consolidating public entities never grants one owner authority to
spend another owner's assets.

Hard conservation, legal ownership, and exact institutional thresholds are invariant. Economic inability such as insufficient available cash or collateral is a typed result. Unbalanced entries, double ownership, silent title transfer, and partial failed settlement are defects.
