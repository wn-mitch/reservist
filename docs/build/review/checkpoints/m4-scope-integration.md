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

**Observed defect, then repair:** resealing M2 failed with `missing required
stewardship finding column receipt_status` because the M3 change replaced the
finding schema that M2's interaction contract reads. M2's original receipt
rules now live in their own scenario table, and a test reseals M1, M2, and M3
byte-for-byte from the current catalog. M2 reseals as `sha256:d9f745b9…2ce4`.

**Inspected after the roster change:** `profile.volcker_1979` carries a
complete thin roster of 169 entries (161 roots), cited to a 1979 UN
membership list plus per-row sources for non-members and claimants. Nine
deep-set sovereigns name researched role holders, and the seven external
channels follow a researched 1979 composition. The catalog validates and the
2006 fixtures reseal unchanged.

**Not established:** no M4 runtime, opening state, calibration, or art beyond
the existing Chair portraits.
