# Manifests and supported transitions

**ID:** `design.content.manifests-and-supported-transitions`
**Status:** `canonical`
**Depends on:** `design.content.catalog-entry-contract`, `design.content.relationships-and-transmissions`

A scenario manifest is the machine-readable selection record. It chooses catalog instances, fidelity models, period variants, providers, residuals, schemas, and parameters for the initialized world. The opening-state bundle separately supplies canonical values. Keeping them separate allows the same structural slice to support multiple researched openings.

A non-selectable scenario candidate or reusable template records planning
requirements and catalog relevance only. It cannot supply defaults to a
manifest. A dated playable instance must bind every selected template
requirement to concrete period content and a frozen source cutoff.

The manifest closes every selected reference and every transition the scenario claims to support: commands, authorities, handlers, reads and writes, witnesses, relationships, transmissions, accounts, legal changes, facilities, provider feedback, failure paths, and succession. It may not omit a lawful supported action merely because its implementation is inconvenient. Unsupported actions are excluded explicitly by the selected content contract, not hidden at runtime.

Selection fixes causal resolution for the run. Runtime salience cannot add a rich actor, relationship, product, provider, or mechanism. Frozen output records resolved identifiers, versions, schemas, and content identity and is validated before start and resume. A frozen slice carries only the period variants effective on the scenario's opening date, so content authored for another era never alters an existing scenario's identity.

Catalog eligibility, manifest selection, and runtime action eligibility remain distinct. A valid manifest can initialize an action that current law or state makes unavailable; the runtime returns that modeled result without treating the manifest as broken.
