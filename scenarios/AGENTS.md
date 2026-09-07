# Scenario fixtures

Scenario directories own frozen manifest, opening state, catalog slice, and replay identity for each executable fixture. Catalog provenance is hash-bearing and legacy locators resolve through [the source map](../docs/source-map.toml).

The M1 fixture is preserved. M2 changes only through the established authored-input and freeze workflow. Documentation reorganization must not rewrite catalog slices or frozen JSON. After any scenario-source change, run the applicable `just freeze`, replay, parity, validation, and hash checks; never reseal an incidental difference.
