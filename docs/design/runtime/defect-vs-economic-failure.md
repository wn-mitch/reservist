# Defect versus economic failure

**ID:** `design.runtime.defect-vs-economic-failure`
**Status:** `canonical`
**Depends on:** `design.runtime.catalog-binding-and-composition`, `design.state.causal-and-stage-contracts`

Modeled inability is data. Engine or content incoherence is a defect. A market may fail to clear with residual quantities; a borrower may refuse; a facility may ration; an action may lack take-up; a prepared settlement may fail for insufficient resources. Each is a typed, witnessed domain outcome that preserves the attempted action and resulting state.

Malformed schemas, duplicate owners, missing handlers, incompatible bindings, illegal transitions, ambiguous writes, broken accounting, impossible initialization, and violated conservation are defects. Validation rejects them before play when possible. Runtime detection stops the affected operation without partial mutation. The engine must not translate a defect into a plausible gameplay failure, substitute a fallback silently, or continue with guessed state.

Fallback is permitted only when the catalog contract explicitly selects a valid alternate representation and names what it preserves, loses, and reconciles. It cannot hide missing required content. A scenario that cannot close its supported action and transition set cannot start or resume.

Player-facing explanation may describe modeled failure without exposing developer internals. Developer diagnostics retain the exact violated invariant, owner, inputs, and witness chain.
