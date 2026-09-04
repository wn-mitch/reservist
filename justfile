python := "python3"
scenario := "scenarios/mvp_2006_cycle"
catalog_dir := ".humanlayer/tasks/federal-reserve-chair-crisis-management-simulator"

test:
    {{python}} -m unittest discover -s tests -v

catalog-test:
    {{python}} {{catalog_dir}}/catalog/test_catalog.py

catalog-generate:
    {{python}} {{catalog_dir}}/catalog/catalog.py import
    {{python}} {{catalog_dir}}/catalog/catalog.py generate

validate:
    {{python}} -m engine.cli validate {{scenario}}

freeze:
    {{python}} -m engine.cli freeze {{scenario}}

run *args:
    {{python}} -m engine.cli run {{scenario}} {{args}}

replay:
    {{python}} -m engine.cli replay-check {{scenario}}

play:
    {{python}} -m engine.cli play {{scenario}}
