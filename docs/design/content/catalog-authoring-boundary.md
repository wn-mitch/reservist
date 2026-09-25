# Catalog authoring boundary

**ID:** `design.content.catalog-authoring-boundary`
**Status:** `canonical`
**Depends on:** `design.content.validation-and-fallbacks`, `design.content.stable-identifiers-and-aliases`

`catalog/*.csv` owns the concrete identity roster, type and instance rows, stable IDs, research status, readiness, relationships, transmissions, families, providers, period variants, and provenance. Canonical design owns the schemas and invariants those rows must satisfy. Prose must not duplicate catalog rows or claim readiness that generated validation does not prove.

Inventory tables are the authored inputs. Each row lives in the partition its schema placement declares: the owning entry's domain, a dated world profile, or a scenario. Root catalog tables are the authoritative normalized import that validation and freezing read; they never keep rows absent from inventory. Generated tables are disposable evidence rebuilt from authored inputs and canonical design. Their contents never become a second authoring source. A catalog ID cited in a canonical leaf must exist in the catalog.

Catalog provenance is hash-bearing scenario input. Existing legacy locator strings remain byte-for-byte stable and resolve through `docs/source-map.toml` at the pinned revision. New active documentation references use stable `doc:` IDs or ordinary links, never path-and-line locators. Source mapping preserves provenance identity without retaining obsolete prose files.

Authoring changes run catalog validation and tests before generation. Scenario-selected changes use the established freeze workflow and produce a reviewed new frozen identity. M1 remains preserved; M2 or later fixtures change only through explicit source edits and resealing, never incidental documentation migration.
