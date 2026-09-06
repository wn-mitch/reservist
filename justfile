python := "python3"
scenario := "scenarios/mvp_2006_cycle"
catalog_dir := "catalog"
godot := "godot"
reservist := "cargo run --quiet --locked -p reservist-cli --"

test:
    cargo test --workspace --locked

gates:
    cargo test --workspace --locked

catalog-test:
    cargo test -p reservist-content --locked
    {{python}} -m unittest discover -s catalog -p test_catalog.py -v

catalog-generate:
    {{reservist}} catalog-import {{catalog_dir}}
    {{reservist}} catalog-generate {{catalog_dir}}

validate:
    {{reservist}} validate {{scenario}}

freeze:
    {{reservist}} freeze {{scenario}}

run *args:
    {{reservist}} run {{scenario}} {{args}}


market-lab fixture="experiments/market_lab/stable_baseline.toml" *args:
    {{reservist}} market-lab run {{fixture}} {{args}}

market-lab-sweep experiment="experiments/market_lab/principal_sweep.toml" *args:
    {{reservist}} market-lab sweep {{experiment}} {{args}}

market-lab-compare control treatment *args:
    {{reservist}} market-lab compare {{control}} {{treatment}} {{args}}
replay:
    {{reservist}} replay-check {{scenario}}


save *args:
    {{reservist}} save {{scenario}} {{args}}

resume *args:
    {{reservist}} resume {{args}}
play: godot-import
    {{godot}} --path godot

cli-play *args:
    {{reservist}} play {{scenario}} {{args}}

rust-build:
    cargo build --workspace --locked

rust-test:
    cargo test --workspace --locked

fmt:
    cargo fmt --all --check

parity:
    {{reservist}} parity

godot-test: godot-import
    {{reservist}} boundaries godot-runtime --godot {{godot}}

lint:
    cargo clippy --workspace --all-targets --locked -- -D warnings
    cargo clippy -p reservist-godot --all-targets --locked --no-default-features -- -D warnings

deps-check:
    {{reservist}} boundaries dependencies

godot-lint:
    {{reservist}} boundaries godot

check: rust-build rust-test fmt lint deps-check parity catalog-test oracle-test validate godot-lint godot-test

godot-import: rust-build
    {{godot}} --headless --editor --path godot --import --quit

oracle-test:
    PYTHONPATH=tools/oracle {{python}} -m unittest discover -s tools/oracle/tests -t tools/oracle -v

oracle-vectors:
    {{python}} tools/oracle/export_vectors.py
