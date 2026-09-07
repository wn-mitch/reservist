# Catalog authoring

This folder owns authored catalog tables and the generator entry point. Concrete identities, contracts, readiness, and provenance live in CSV. Canonical schema and invariants are routed from [catalog authoring design](../docs/design/content/catalog-authoring-boundary.md).

Catalog provenance is hash-bearing scenario input. Preserve existing legacy locator strings byte-for-byte; they resolve through [the source map](../docs/source-map.toml) at its pinned revision. New documentation references use stable `doc:` IDs. Root CSV is regenerated from [inventory](inventory/AGENTS.md); derived evidence lives under [generated](generated/AGENTS.md). Run `just catalog-test` after authored changes and `just docs-check` after guidance changes.
