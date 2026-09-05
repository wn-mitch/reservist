from __future__ import annotations

import json
import shutil
import tempfile
from contextlib import contextmanager
from pathlib import Path
from typing import Iterator


PROJECT_ROOT = Path(__file__).resolve().parents[3]
ORACLE_ROOT = Path(__file__).resolve().parents[1]
SCENARIO = PROJECT_ROOT / "scenarios/mvp_2006_cycle_m1"


@contextmanager
def copied_scenario() -> Iterator[Path]:
    with tempfile.TemporaryDirectory() as directory:
        target = Path(directory) / "scenario"
        shutil.copytree(SCENARIO, target)
        yield target


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value: dict) -> None:
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
