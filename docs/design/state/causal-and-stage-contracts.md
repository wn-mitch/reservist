# Causal and stage contracts

**ID:** `design.state.causal-and-stage-contracts`
**Status:** `canonical`
**Depends on:** `design.state.canonical-state-and-ownership`

Reservist separates information and action stages so no stage implies the next. A canonical event may produce a scoped observation. Delivery makes evidence available to a recipient. A recipient may revise a belief, reconsider a plan, choose an action, obtain authority, attempt execution, create transactions, settle them, and emit domain events. Each transition names its owner, inputs, result, timing, and witness.

Authorization is not execution. Execution is not take-up. Take-up is not clearing. Clearing is not settlement. A submitted package may be admitted atomically while its constituent actions later succeed, fail, ration, expire, or settle independently under their real lifecycles. Package atomicity prevents a partial submission mutation; it does not promise downstream success.

Causal modules communicate through typed stocks, flows, commands, results, events, observations, claims, commitments, and transmissions. They do not reach into another owner's private state. A crisis is an attractor arising from ordinary material state and mechanisms, not a canonical crisis-pressure meter. Case files and situation views may organize evidence but do not create the condition they describe.

Developer traces may retain complete causal lineage. Player-facing artifacts remain bounded by observation, access, and institutional interpretation.
