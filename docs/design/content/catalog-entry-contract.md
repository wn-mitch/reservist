# Catalog entry contract

**ID:** `design.content.catalog-entry-contract`
**Status:** `canonical`
**Depends on:** `design.representation.representation-axes`, `design.runtime.catalog-binding-and-composition`

The catalog is the only structural input to a scenario manifest. Each entry has a stable `catalog_id`, `entry_class`, identity clade, type linkage where applicable, definition version, cognition class, permitted fidelity, selectable flag, fallback and residual references, period variants, research and completeness state, uncertainty, provenance, and required contracts.

Type entries define reusable structure, own no runtime state, and are not selectable. Instance entries reference their type and supply subject-specific identity, relationships, ownership, behavior, observations, and readiness. A manifest selects an instance and model and may override declared parameters, never structure.

Primary indexing follows identity clade because it exposes ontology mismatch. Domain, scenario, channel, readiness, and presentation are derived views. A `fit` or research label records current evidence; it does not waive a missing required field.

Catalog validity proves structural consistency, not scenario initialization or runtime action eligibility. A selectable entry must close every referenced relationship, transmission, owner, schema, handler, witness, period variant, fallback, and selected-model requirement. The concrete roster and its readiness are owned by CSV and generated evidence, not this prose contract.
