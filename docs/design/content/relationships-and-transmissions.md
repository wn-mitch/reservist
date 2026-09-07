# Relationships and transmissions

**ID:** `design.content.relationships-and-transmissions`
**Status:** `canonical`
**Depends on:** `design.representation.relationships-and-affiliations`, `design.economy.opening-conditions-and-transmission`

Relationship records describe durable composition, scope, affiliation, ownership, contract, access, eligibility, control, or other structure. Each has a stable type, typed endpoints, effective period, provenance, observability, lifecycle, and declared ownership effect. A relationship does not propagate a value merely because two subjects are connected.

Transmission records describe values crossing canonical-owner boundaries. Each names producer owner and transition, stable output ID, consumer owner and input ID, schema, unit, direction, effective delay, capacity or saturation, persistence, provenance, witness, selected provider, and fallback behavior. Conserved states cross through balanced transactions; conditions and observations still require owner-specific transitions.

The manifest closes both registries for every selected entry. Missing endpoints, units, owners, witnesses, or provider bindings are structural failures. Generic `affects` edges and prose-only causal descriptions are invalid substitutes.

A transmission may be one-to-one, distributed, aggregated, delayed, or conditional, but its semantics are typed and reviewable. Fallback must preserve the declared receiving contract or state an honest loss. Relationship changes and transmission effects retain separate histories so structure cannot be mistaken for causation.
