# Stable identifiers and aliases

**ID:** `design.content.stable-identifiers-and-aliases`
**Status:** `canonical`
**Depends on:** `design.content.catalog-entry-contract`, `design.representation.identity-presentation-and-persistence`

Canonical catalog IDs are stable, explicit, and reviewed. Authoring output, scenario manifests, relationships, transmissions, contracts, witnesses, records, and frozen data reference those IDs directly. Display names, satire, species, localization, and other labels live in a separate presentation register excluded from simulation identity.

There is no general alias or automatic entity-reconciliation system. Source-specific alternate labels may be added only when a real import, lookup, or search workflow requires them, and they resolve to one reviewed canonical ID without changing provenance. Fuzzy matching cannot decide identity for causal loading.

Mergers, successions, reorganizations, transformed legal subjects, and changed ownership are explicit dated transitions. They are not aliases. A renamed display label does not create a new subject; a legal successor does not silently inherit all private or institutional state.

A subject that persists as the same legal person keeps its ID and gains a dated
period variant when its name, territory, constitution, or membership changes.
Dissolution, partition, or a continuator claim creates a new identity linked by
a dated `SUCCEEDED_BY` relationship. The successor receives only what that
transition explicitly transfers.

Stable contract, handler, state, input, output, relationship, transmission, family, bucket, and option IDs follow the same principle. Frozen data stores resolved IDs and definition versions. If an ID changes meaning, create a new version or identity and migrate every caller; never reuse it for a different contract.
