# Volcker opening checkpoint

**ID:** `evidence.checkpoints.volcker-opening`
**Status:** `evidence`
**Depends on:** `mandate.m4-proving-lens`, `design.economy.reserves-and-operating-regimes`
**Verifies:** `design.economy.reserves-and-operating-regimes`, `design.institutions.policy-portfolios`, `mandate.m4-proving-lens`

**Inspected:** the runtime builds only the subsystems a scenario selects, so
the 2006 Treasury, repo, and population cast is absent from 1979. Scenarios
author their own policy packages. A package may carry constituent actions
owned outside the FOMC, which each owner's members decide from authored
belief tests. The Desk executes a funds-rate band or a nonborrowed reserves
path in a reserves market whose requirements bind only covered member banks.
Meetings without a published statement time release no statement.

**Observed on `scenarios/volcker_1979`:** the fixture seals as
`sha256:0c94f427…7a89` and replays identically for all five packages. All
161 roster roots are selected as identity-only composition roots; they own no
state or scheduled work, and adding them left every transcript byte unchanged.
August 1979 magnitudes open the market: M-1 of $373.3 billion, required
reserves near $40.1 billion, borrowing near $0.8 billion, and a 10% discount
rate. With twelve voters and researched leanings:

| Package | Committee | Consequence |
|---|---|---|
| Firm band with discount increase | 10–2, Rice and Black dissenting | Board adopts 10.5% on August 16 |
| Hold prevailing band | 8–4 | none beyond the directive |
| Larger firming | 9–3 | Board adopts 10.5% |
| Ease | 3–9, rejected | none |
| Reserves path | 4–8, rejected | none |

The first simulated week produced M-1 of $373.1 billion against a recorded
$373.3 billion. M1, M2, and M3 still reseal byte-for-byte and parity passes.

**Calendar and folders:** the opening adopts `calendar_folders_v1` with the
chief of staff, three staff units, and folders for the five packages. A folder
proposal of the firm band reaches the Committee and carries 10–2. The staff
request is authored content, not the built-in 2006 dealer task: International
Finance assesses how much of the announced Saudi increase is lifting. Its file
separates announcement, capacity, production, delivery, and access, and Monetary
Policy must route it through a restricted, sanitized, or granted channel.

**Not established:** macro releases beyond weekly money and funds data,
calibration of money demand and borrowing, the September and October cycles,
the post-meeting staff review and scorecard, which still assume 2006 settlement
witnesses, and statement claims, which 1979 does not publish.
