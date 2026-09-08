# Instrument and product families

**ID:** `design.content.instrument-and-product-families`
**Status:** `canonical`
**Depends on:** `design.content.catalog-entry-contract`, `design.economy.accounting-and-settlement`

Instrument families are closed type-level definitions referenced by positions. Positions live in accounts; buckets aggregate fungible risk positions; a specific security exists only through the declared pre-run extension and residual-reconciliation path. Families encode duration, collateral role, settlement, demandability, credit state, currency, quantity and value measures, contingency, liquidity, seniority, priority, rollover, rates, conversion, margin, and counterparty exposure where applicable.

Physical product and commodity families are also closed type-level definitions. They declare stage, grade, unit, storage, substitution, transformation, transport, loss, custody, title, pricing, and settlement semantics. Umbrella labels may organize presentation, but canonical inventories and transmissions use stage-specific codes.

A physical distinction becomes a family dimension only when it changes
fungibility, acceptance, substitution, transformation yield, storage, route,
destination eligibility, loss, payment, or settlement. Joint outputs and
quality changes require witnessed transformations. Concrete crude grades,
refined products, gas and LNG forms, water qualities, power products,
petrochemical inputs, fertilizers, and period availability remain catalog data,
not prose enums.

Neither family kind is a represented subject, owner, Agent, market, or facility. Innovation and arbitrage combine, repackage, substitute, transform, or migrate exposure among known families rather than inventing unrestricted runtime types.

A bucket key contains only dimensions that alter fungibility or modeled risk. A transformation names owner, input and output accounts, capacity, yields or loss, required authority, and witness. Family additions require schema review, dependent catalog closure, generated outputs, and a new frozen content identity where selected.
