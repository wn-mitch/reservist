# Documentation governance

This tree is the active documentation boundary. Non-`AGENTS.md` Markdown starts with one H1, then `ID`, `Status`, and `Depends on` metadata. Evidence also declares `Verifies`; proposals declare `Proposes changes to`. IDs are stable lowercase dot-separated owners. Status is one of `canonical`, `mandate`, `evidence`, `proposal`, or `index`.

Canonical leaves own one design contract. Mandates define authorized work. Evidence records inspected or observed implementation without changing design. Proposals are nonbinding. Indexes route and summarize but own no rules. Use `none` for empty metadata. Edit an existing owner rather than restating it. Accept a proposal by moving its accepted clauses into canonical owners and deleting it.

Every documentation directory has a local `AGENTS.md` linking every immediate child document and documentation folder. Preserve IDs across moves and update dependencies in the same change. Active Markdown uses stable `doc:` IDs or ordinary links, never path-and-line locators. Target 400–500 words; 601 words fails. Run `just docs-check` after every documentation change.

- [Current design](CURRENT_DESIGN_BIBLE.md)
- [Build mandate](BUILD_MANDATE.md)
- [Build review](BUILD_REVIEW.md)
- [Canonical design tree](design/AGENTS.md)
- [Build tree](build/AGENTS.md)
