# Persistence and identity

**ID:** `design.state.persistence-and-identity`
**Status:** `canonical`
**Depends on:** `design.state.canonical-state-and-ownership`, `design.state.correction-and-invalidation`

A save contains every behavior-affecting fact required for exact continuation: canonical owner state, effective identity and relationship records, beliefs and plans, commitments, queues with stable sequence, deterministic random state, pending work versions, command idempotency, witnesses, historical entries, and the frozen content identity. Derived caches may be rebuilt only when rebuilding is deterministic from those owners.

Resume must be behaviorally identical to uninterrupted execution. The same valid commands after a checkpoint produce the same ordering, categorical outcomes, conserved state, and final state identity. Normal saves occur only at completed deterministic commit barriers. The runtime refuses a save whose in-flight presentation context cannot be represented without discarding effectful state.

Frozen scenario identity records resolved stable IDs, versions, period variants, fidelity selections, content hashes, and compatible schemas. It never stores function pointers. Cross-language byte identity with a retired oracle is unnecessary; semantic invariants, stable ordering, and exact continuation inside the production runtime are mandatory.

Historical identities and transitions are immutable. Mergers, reorganizations, office succession, and renamed legal subjects use explicit dated records rather than aliases or destructive replacement.
