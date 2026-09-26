# Scenario fixtures

Scenario directories own frozen manifest, opening state, catalog slice, and replay identity for each executable fixture. Catalog provenance is hash-bearing and legacy locators resolve through [the source map](../docs/source-map.toml).

The M1 fixture is preserved. M2 changes only through the established authored-input and freeze workflow. Documentation reorganization must not rewrite catalog slices or frozen JSON. After any scenario-source change, run the applicable `just freeze`, replay, parity, validation, and hash checks; never reseal an incidental difference.

`volcker_1979` is the first M4 opening: the Miller-Volcker handover through the August 14, 1979 FOMC meeting and its discount-rate and weekly money consequences. It selects the domestic policy cast, the reserves market, petroleum systems, and the calendar-and-folders interaction; its staff request is authored in `staff/work_1979.json` as `request_task`, replacing the built-in 2006 dealer task.

`templates/` holds reusable scenario templates. A dated instance names its template and source cutoff in the manifest; `just freeze` records the template hash, and validation fails if the template changes afterward or the instance stops meeting it.

`volcker_1979_11` is a second instance of the Volcker lens: November 5 through the November 20 FOMC meeting, with Iran sanctions implementation as a RESPONSIVE channel and the oil import ban, Iranian occurrences, and blocking order recorded.
