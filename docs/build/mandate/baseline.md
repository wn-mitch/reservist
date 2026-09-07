# Baseline

**ID:** `mandate.baseline`
**Status:** `mandate`
**Depends on:** `design.runtime.replay-and-determinism`, `design.mvp.early-2006-opening-state`

The historical oracle is pinned to repository revision `97fc926a` (jj `oyrstukq`), Python 3.14.3 with no third-party dependencies, and the original early-2006 scenario input. The oracle remains test tooling only; it is not a production sidecar, fallback, or causal authority.

The preserved M1 fixture is `scenarios/mvp_2006_cycle_m1` with scenario hash `sha256:493ca02478ebd045bc2ab5720ef3ef65410f4557f97410570a4576a39578d066`. The adopted M2 fixture is `scenarios/mvp_2006_cycle` with hash `sha256:d9f745b948d7cf2d76e364ff08664a0fcf3347a41485e107a18b02297bca2ce4`.

M1 oracle replay identities are:

- `WAIT_AND_WARN`: state `sha256:424bdafb55ed42ca9c06b3a41634926351d5d26d529552bcc061f6efb626337b`.
- `MEASURED_FIRMING`: state `sha256:58738b57d2e8bf3d4a53c11c90d0b4730aa67bef6c9338c0024f2670be6cdd63`.
- `FIRMING_BIAS`: state `sha256:af3ee7c770d23610ea4cb26fb61faab35ab7fc904d08d4bc9cf3c610c0d11cb2`.

Frozen catalog and scenario inputs may change only through the existing reviewed freeze workflow. Documentation migration must preserve both hashes.
