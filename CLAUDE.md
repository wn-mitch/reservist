# Reservist

Start here, in order: `docs/CURRENT_DESIGN_BIBLE.md` (what we are building and which text governs), `docs/BUILD_MANDATE.md` (what work is authorized and how completion is judged), `docs/BUILD_REVIEW.md` (what the executable actually does and what is unproved). Design chapters and the adopted decision handoff live in `docs/design/`; the authoring catalog in `catalog/`; the Python prototype in `engine/`, `scenarios/`, `tests/`.

Commands: `just test`, `just gates`, `just validate`, `just freeze`, `just replay`, `just run`, `just play`, `just catalog-test`, `just catalog-generate`. Python 3.14, no third-party dependencies.

Version control is jj. Do not restate game rules here; edit the owning chapter and record the change in the Build Review.
