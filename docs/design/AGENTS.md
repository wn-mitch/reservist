# Canonical design

This folder owns binding game and architecture contracts. Each immediate domain folder has one local routing file and narrowly scoped canonical owners. Edit the existing owner for a concept; never restate its rules in another leaf. Cross-domain references use stable `doc:` IDs. Moving a contract preserves its ID and updates every dependency in the same change. Run `just docs-check` after any edit.

- [Gulf reference world](gulf-web.md)

- [Product](product/AGENTS.md)
- [State](state/AGENTS.md)
- [Runtime](runtime/AGENTS.md)
- [Representation](representation/AGENTS.md)
- [Institutions](institutions/AGENTS.md)
- [Economy](economy/AGENTS.md)
- [Information](information/AGENTS.md)
- [Interface](interface/AGENTS.md)
- [Campaign](campaign/AGENTS.md)
- [Content](content/AGENTS.md)
- [MVP](mvp/AGENTS.md)
