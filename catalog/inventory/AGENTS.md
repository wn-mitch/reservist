# Catalog inventory

Inventory CSV files are the partitioned authored source for catalog import. Keep schema-exact headers, stable IDs, provenance, and table lifecycle semantics. Import is authoritative for root catalog CSV and must not preserve stale root-only rows.

Rows live in exactly three folder shapes, and `table_placement` in `../schema.json` decides which one:

- `<domain>/` holds rows that belong to one catalog entry. The folder name equals that entry's `entities.csv` domain. `shared/` holds catalog-wide tables such as types, probes, instruments, the research backlog, the registered project sites, technologies, and feasible-project envelopes, and presentation asset metadata.
- `profiles/<profile_id>/` holds rows keyed by a dated world profile.
- `scenarios/<scenario_id>/` holds rows keyed by a scenario or template, plus the campaign tables of the fixture that uses them.

Validation rejects a row outside its declared folder, nested folders, and files that are not schema tables.

Legacy Markdown locators and retired inventory paths are hash-bearing provenance and resolve through [the source map](../../docs/source-map.toml); do not rewrite them. M1 source data remains frozen. Changes intended for M2 or later scenarios require catalog validation, generation, the established freeze workflow, replay, and reviewed new identity. Run `just catalog-generate` and `just catalog-test`.
