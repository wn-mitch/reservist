# Runtime design

Authority class: canonical. This folder owns compiled content binding, phase order, dispatch, scenario selection, client boundaries, replay, and defect classification. It does not own domain state or concrete catalog rows. Edit the existing owner and use stable IDs for cross-domain dependencies. Run `just docs-check`.

- [Catalog binding and composition](catalog-binding-and-composition.md)
- [Ordering and temporal cadence](ordering-and-temporal-cadence.md)
- [Dispatch and activation](dispatch-and-activation.md)
- [Scenario selection](scenario-selection.md)
- [Client and repository boundary](client-and-repository-boundary.md)
- [Replay and determinism](replay-and-determinism.md)
- [Defect versus economic failure](defect-vs-economic-failure.md)
