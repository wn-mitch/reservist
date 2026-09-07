# Validation and fallbacks

**ID:** `design.content.validation-and-fallbacks`
**Status:** `canonical`
**Depends on:** `design.content.manifests-and-supported-transitions`, `design.runtime.defect-vs-economic-failure`

Catalog validation rejects duplicate IDs, incompatible clades and fidelity, unresolved references, missing owners, malformed schemas, invalid period variants, incomplete relationships or transmissions, unit mismatches, absent witnesses, and dishonest fallback contracts. Scenario compilation additionally rejects unregistered handlers, unsupported transitions, ambiguous writes, illegal phase feedback, missing opening state, and incomplete manifest closure.

Missing or incompatible frozen bindings prevent start or resume. Validation does not invent defaults, downgrade fidelity, drop a selected entry, or translate a defect into an economic failure. Failed compilation leaves prior frozen outputs unchanged.

Every fallback names the alternate entry or adapter, the stable interface it preserves, information or behavior honestly lost, residual treatment, disappearance behavior, and conditions under which selection is valid. A fallback cannot mask incomplete required content. When no compatible representation exists, selection fails.

Runtime handlers are total over valid frozen input. Modeled refusal, rationing, failed clearing, or failed settlement remains a typed result. Structural impossibility and accounting violations remain defects. Generated readiness reports evidence these checks but do not redefine them.
