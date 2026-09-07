# Replay and determinism

**ID:** `design.runtime.replay-and-determinism`
**Status:** `canonical`
**Depends on:** `design.state.persistence-and-identity`, `design.runtime.ordering-and-temporal-cadence`

A run is deterministic for the same frozen scenario identity and command sequence. Queued events carry stable sequence numbers. Randomness comes from persisted deterministic subsystem streams or one explicitly ordered stream; every draw is keyed or consumed in a stable order. Registration, iteration, commit, and output ordering are stable.

Replay proves commands, receipts, authorization, event order, witnesses, transactions, categorical outcomes, exact conserved state, and final state identity. Calibrated numeric outputs compare under declared channel tolerances, while accounting and contract thresholds remain exact. A presentation build or label change cannot alter simulation replay identity.

Checkpoint and resume equivalence is stronger than merely loading. A checkpoint taken at a supported commit barrier and continued with the same commands must produce the same later trace and state as uninterrupted execution. Corrupt, incompatible, or incomplete saves fail closed.

Historical oracle fixtures may remain as test evidence after a one-way rewrite, but they are not production authority or fallback. Cross-language byte equality is unnecessary. Semantic invariants, deterministic production ordering, and exact continuation are mandatory.
