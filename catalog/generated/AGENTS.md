# Generated catalog evidence

This folder contains disposable derived views produced from authored inventory and root catalog data. It owns no source identity, contract, readiness decision, or provenance. Regenerate outputs through the existing catalog generation flow; never hand-author a generated CSV or treat it as an input.

`source_comparison.csv` compares stable IDs cited in canonical design leaves with catalog data; generation fails when a leaf cites an absent ID. Documentation migration may change that derived file without changing hash-bearing catalog input. Run `just catalog-generate` and `just catalog-test` after source changes.
