# Volcker November checkpoint

**ID:** `evidence.checkpoints.volcker-november`
**Status:** `evidence`
**Depends on:** `mandate.m4-proving-lens`, `evidence.checkpoints.volcker-opening`
**Verifies:** `design.economy.external-provider-contracts`, `design.campaign.scenario-and-campaign-composition`, `mandate.m4-proving-lens`

**Inspected:** a dated instance names a template and source cutoff; sealing
freezes the template hash, and validation rejects a changed template or an
instance that no longer meets it. Channels declare RECORDED or RESPONSIVE
modes. A channel that consumes Chair-reachable interventions cannot be
RECORDED, and a RESPONSIVE channel needs its state owner selected. Iran
sanctions implementation is RESPONSIVE: the blocking order is recorded, the
New York Fed executes on the accounts it holds, and the Chair coordinates
foreign-branch compliance. Iran withdraws a daily share of unblocked official
deposits once it announces intent.

**Observed on `scenarios/volcker_1979_11`:** the instance of
`template.volcker_lens` seals as `sha256:8373bde3…f4e9` and replays
identically for all five packages. It opens November 5 under the reserves
path with a 12% discount rate. Weekly funds clear at 13.53–13.65% against
recorded weekly averages of 13.1–13.77%.

| Package | Committee | Iranian withdrawal |
|---|---|---|
| Continue path, execute and coordinate at 08:30 | 12–0 | none |
| Continue path, no coordination | 12–0 | $5.6 billion from foreign branches |
| Continue path, both on November 16 | 12–0 | $6.0 billion |
| Slower path | 9–3, Partee, Teeters, Rice dissenting | none; funds rise to about 14.1% |
| Return to a funds band | 1–11, rejected | none |

Moving execution before the 08:10 order leaves both actions unauthorized.

Feasible-project envelopes are catalog rows naming a registered site, a
registered technology, eligible owners, and capacity, cost, and lead-time
ranges. The slice freezes them onto their owners. The Abqaiq–Yanbu crude line
is the one 1979 envelope, pinned to its observed 1.85 mb/d, $1.6 billion, and
4.5 years. A recorded project inside it builds; an unregistered envelope or an
ineligible owner fails the run, and catalog validation rejects unregistered
sites and technologies. Completed capacity commissions into operations at the
next weekly realization.
Every committed fixture reseals byte-for-byte and parity passes.

**Not established:** as-published November SOMA and M-1 levels, Iranian
production after August, evidence for the withdrawal rate, the East-West
line's construction dates, the 1979
calendar-and-folders interaction, and events after November.
