# Catalog inventory

Inventory CSV files are the partitioned authored source for catalog import. Keep schema-exact headers, stable IDs, provenance, and table lifecycle semantics. Import is authoritative for root catalog CSV and must not preserve stale root-only rows.

Legacy Markdown locators are hash-bearing provenance and resolve through [the source map](../../docs/source-map.toml); do not rewrite them during documentation migration. M1 source data remains frozen. Changes intended for M2 or later scenarios require catalog validation, generation, the established freeze workflow, replay, and reviewed new identity. Run `just catalog-test`.
