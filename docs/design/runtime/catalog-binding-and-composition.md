# Catalog binding and composition

**ID:** `design.runtime.catalog-binding-and-composition`
**Status:** `canonical`
**Depends on:** `design.content.catalog-entry-contract`, `design.content.manifests-and-supported-transitions`

Author catalog and manifest data in reviewed structured files. Every selected executable contract names a stable contract ID, owner ID, handler key, input and output schema references, witness requirements, period variant, and fidelity selection. A compiler validates uniqueness, referential integrity, schema compatibility, registered-handler availability, legal transition closure, and complete bindings before play.

The runtime registry maps stable handler keys to compiled Rust implementations. Frozen scenario data records resolved identifiers, definition versions, content identity, and selected parameters. It does not contain function pointers or executable scripts. The runtime reads the compiled frozen slice rather than authoring CSV during play.

The global catalog describes what may be represented. Scenario composition chooses the initialized subset and its approved models. Runtime state and current law determine which actions are eligible. These boundaries must not be collapsed: catalog validity does not initialize an entry, scenario inclusion does not authorize an action, and a valid action may still fail economically.

Machine validation is authoritative. Generated diagrams and indexes explain compiled registration but do not define it.
