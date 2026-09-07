# Asset and language-model boundary

**ID:** `design.interface.asset-and-language-model-boundary`
**Status:** `canonical`
**Depends on:** `design.runtime.client-and-repository-boundary`, `design.representation.identity-presentation-and-persistence`

Godot imports approved presentation assets. Each source asset has a stable asset ID, provenance, usage rights, dimensions, crop, focal point, and declared role. Import settings and Godot versions are pinned. Validation rejects missing references and incompatible properties. Generated import caches remain untracked, and presentation build identity stays separate from frozen simulation identity.

Runtime language-model integration is prohibited. No model chooses actions, updates beliefs, resolves mechanics, writes player-facing prose, interprets canonical state, or sits behind a speculative renderer seam. No model or network dependency belongs inside current executable game bounds.

Player-facing text uses structured simulation output plus reviewed authored lines, templates, and conditional passages. The committed content determines claims and mechanical bindings. Offline language-model collaboration may help draft catalogs, dialogue, or prose, but ordinary human review, source control, schema validation, and content checks remain authoritative.

A runtime model may be reconsidered only after a concrete authored-content failure is demonstrated and the design owner accepts a new bounded contract. Convenience or expected future scale is not sufficient.
