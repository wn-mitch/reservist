# Generators, processes, and references

**ID:** `design.representation.generators-processes-and-references`
**Status:** `canonical`
**Depends on:** `design.representation.representation-axes`

A generator owns keyed initiating draws and emits typed initiating facts or shocks. It does not own the evolving world condition or assign final economic outcomes. A stateful external process owns evolving physical or external state such as weather, disease, conflict conditions, or resource pressure. People and institutions own attributable interventions and incidents. Causal references connect initiation, process evolution, and actor response without combining them into one actor.

A `ScheduledProcess` owns a published occurrence calendar, revision history, and derived windows that may gate behavior. It is distinct from a person's private agenda and from a queued runtime event. A `PublishedReference` owns a released measurement or benchmark, its vintages, methodology, revision policy, and publication provenance. The institution and measurement mechanism remain separate owners.

A `Record` is a persistent institution-owned artifact with identity, custody, lifecycle, awareness, status, and provenance. Assessments, analytical tasks, case files, policy packages, proceedings, and programs may specialize it. Records can organize evidence and authorize lifecycle steps only through declared clauses; they cannot absorb the canonical state they describe.

Narrative templates and hooks remain downstream interpretations of owned events and records. They never mutate state directly.
