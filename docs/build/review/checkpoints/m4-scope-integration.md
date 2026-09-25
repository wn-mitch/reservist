# M4 scope integration checkpoint

**ID:** `evidence.checkpoints.m4-scope-integration`
**Status:** `evidence`
**Depends on:** `mandate.m4-proving-lens`, `mandate.evidence-obligations-m3-m4`
**Verifies:** `design.representation.sovereign-necessity-systems`, `design.interface.art-direction-and-character-production`, `design.content.catalog-authoring-boundary`, `mandate.m4-proving-lens`

**Inspected on the change after M3 (`twnyzxpu`) and its docs successor:** the
settled M4 scope now lives in canonical owners and the M4 proving-lens mandate.
Catalog inventory is partitioned into entity-domain, dated-profile, and
scenario folders, and import rejects rows outside their declared partition.
Sovereign, federated, and region entries own no state or relationship records.
The Python catalog validator is retired; its uncovered tests and the
design-citation comparison run natively. Chair art lives in per-character
folders with a closed four-role asset vocabulary.

**Observed:** `just check` passed. Resealing M1 and M3 reproduced
`sha256:493ca024…d066` and `sha256:095a1bbd…305a` byte-for-byte, and replay
checks passed for M1, M2, and M3.

**Observed defect:** `reservist freeze scenarios/mvp_2006_cycle` fails with
`missing required stewardship finding column receipt_status`. The M3 change
replaced the stewardship-finding schema, while the M2 interaction contract
still requires the earlier columns. The committed M2 slice still validates
and replays; only resealing M2 from the current catalog fails.

**Not established:** no M4 runtime, sovereign bundle, roster, provider,
calibration, or art beyond the existing Chair portraits.
