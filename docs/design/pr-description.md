[HumanLayer Task](https://cloud.humanlayer.com/artifacts/01a06425-b741-7618-9c5f-f5342373bfc4)

## Why the change

Reservist needs a playable proof that one bounded Federal Reserve policy cycle can preserve institutional authority, partial information, endogenous market response, and deterministic replay without building the full campaign.

## Special things to note

- The runtime is standard-library Python and intentionally uses direct audience delivery plus one bilateral repo relationship rather than a general network model.
- `just test`, `just gates`, and `just replay` pass, but the catalog commands and `tests/test_phase6_catalog_slice.py` depend on the ignored `.humanlayer/tasks/federal-reserve-chair-crisis-management-simulator/catalog` artifact, and the global catalog still has 214 pre-existing validation issues outside this slice.

## Change outline

The new codebase separates scenario data, canonical owners, player-safe projections, and executable acceptance checks.

```text
reservist/
├── engine/
│   ├── state/ + accounting/ + markets/   # canonical ownership and material transitions
│   ├── staff/ + cognition/ + bodies/     # evidence, beliefs, FOMC procedure, authority
│   ├── communication.py + delivery.py    # structured claims and bounded audiences
│   ├── commitments.py + postmortem.py    # persistent obligations and player-safe review
│   ├── harness/                          # Office, FOMC, operations, wire, and review screens
│   └── scenario.py                       # deterministic cycle orchestration
├── scenarios/mvp_2006_cycle/             # frozen manifest, opening state, cast, law, and tape
└── tests/                                # invariants, replay checks, negative paths, ten gates
```

The cycle carries one decision from scoped evidence into institutional memory without giving the Chair direct control over later stages.

```text
scheduled release
  -> scoped observation -> Morning Book
  -> bounded staff request -> sourced assessment -> belief revision
  -> Chair package -> FOMC vote -> certified directive
  -> New York Desk order -> Treasury clearing -> two-phase settlement
  -> structured statement -> Loonberg/direct audiences -> participant orders
  -> commitments + intermeeting realization -> next Morning Book -> staff review
```

Hard boundaries keep authority, causality, and player knowledge distinct.

```diff
- policy package -> scripted yield and public response
+ package -> FOMC authorization -> Desk execution
+         -> participant-owned orders + dealer capacity -> market clearing
+         -> structured claims -> per-recipient delivery -> optional belief revision

- harness reads canonical scenario state
+ canonical owner -> witnessed transition -> scoped observation/delivery -> player record

- outcome text decides what happened
+ witnessed events and receipts determine state; text renders player-safe records
```
