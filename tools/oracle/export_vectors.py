#!/usr/bin/env python3
"""Export pinned Python-runtime oracle vectors as RFC 8785 canonical JSON.

This script deliberately drives ScenarioRuntime rather than only the CLI: parity
also needs state snapshots, receipts, and exact witness lines. It works before
and after the Python oracle moves under tools/oracle/.
"""

from __future__ import annotations

import argparse
import contextlib
import dataclasses
import io
import json
import shutil
import sys
import tempfile
from pathlib import Path
from typing import Any, Iterator
from unittest.mock import patch

ORACLE_ROOT = Path(__file__).resolve().parent
REPOSITORY_ROOT = ORACLE_ROOT.parents[1]
if str(ORACLE_ROOT) not in sys.path:
    sys.path.insert(0, str(ORACLE_ROOT))

from engine.canon import canonical_bytes, sha256  # noqa: E402
from engine.cli import play_command, replay_command, run_command  # noqa: E402
from engine.packages import PACKAGES  # noqa: E402
from engine.scenario import ScenarioRuntime, seal_scenario, validate_scenario  # noqa: E402
from engine.staff.analytical_task import RequestMode  # noqa: E402

REQUEST_MODES: tuple[RequestMode | None, ...] = (
    None,
    RequestMode.NORMAL,
    RequestMode.ACCELERATED,
    RequestMode.DECLINED,
    RequestMode.MISSED,
)
RESEED_VALUES = (20060328, 20060329, 20060330, 20060331)
PLAY_SCRIPT = (
    "propose MEASURED_FIRMING",
    "advance",
    "advance",
    "advance",
    "advance",
    "review",
    "quit",
)


def main() -> int:
    args = parse_args()
    scenario = resolve_from_root(args.scenario)
    output = resolve_from_root(args.output)
    output.mkdir(parents=True, exist_ok=True)

    for package_id in PACKAGES:
        for request_mode in REQUEST_MODES:
            vector = capture_run(scenario, package_id, request_mode)
            write_vector(output / vector_name(package_id, request_mode), vector)

    for seed in RESEED_VALUES:
        with copied_reseeded_scenario(scenario, seed) as reseeded_scenario:
            vector = capture_run(reseeded_scenario, "MEASURED_FIRMING", None)
        vector["vector_kind"] = "reseed"
        write_vector(output / f"reseed__{seed}.json", vector)

    write_vector(output / "play__scripted.json", capture_play(scenario))
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--scenario", default="scenarios/mvp_2006_cycle_m1")
    parser.add_argument("--output", default="tests/oracle/vectors")
    return parser.parse_args()


def resolve_from_root(value: str) -> Path:
    path = Path(value)
    return path if path.is_absolute() else REPOSITORY_ROOT / path


def capture_run(
    scenario_dir: Path, package_id: str, request_mode: RequestMode | None
) -> dict[str, Any]:
    scenario = validate_scenario(scenario_dir)
    runtime = ScenarioRuntime(scenario, package_id=package_id, request_mode=request_mode)
    result = runtime.run_all()
    transcript = [event.to_dict() for event in runtime.ledger.events]
    transcript_lines = result.transcript.decode("utf-8").splitlines()
    assert transcript_lines == [
        canonical_bytes(event).decode("utf-8") for event in transcript
    ], "WitnessLedger transcript drifted from engine.canon"

    request = request_mode.value if request_mode is not None else None
    seed = scenario.initialization.value["seed"]
    state = state_snapshot(runtime)
    material = material_snapshot(runtime)
    assert sha256(state) == result.state_hash, "exported state snapshot does not match runtime identity"
    assert sha256(material) == runtime._material_state_hash(), "exported material snapshot drifted"
    return {
        "format_version": 1,
        "vector_kind": "run",
        "source_identity": source_identity(scenario_dir, package_id, request, seed),
        "scenario_hash": result.scenario_hash,
        "state_hash": result.state_hash,
        "request": request,
        "package": package_id,
        "seed": seed,
        "result": result_dict(result, transcript_lines),
        "transcript": transcript,
        "transcript_canonical_lines": transcript_lines,
        "state_snapshot": state,
        "material_snapshot": material,
        "receipts": [receipt.to_dict() for receipt in runtime.receipts],
        "canonical_registry": {
            owner_id: runtime.registry._owners[owner_id].snapshot_for_hash()
            for owner_id in sorted(runtime.registry._owners)
        },
        "cli_stdout": {
            "run": capture_run_stdout(scenario_dir, package_id, request),
            "replay_check": capture_replay_stdout(scenario_dir),
        },
    }


def source_identity(
    scenario_dir: Path, package_id: str, request: str | None, seed: int
) -> dict[str, Any]:
    """Location-independent identity, stable after engine moves under tools/oracle."""
    return {
        "engine_module": "engine.scenario",
        "baseline_revision": "97fc926a",
        "python_version": ".".join(map(str, sys.version_info[:3])),
        "oracle_format": "reservist-python-runtime-v1",
        "scenario": scenario_dir.name,
        "package": package_id,
        "request": request,
        "seed": seed,
    }


def result_dict(result: Any, transcript_lines: list[str]) -> dict[str, Any]:
    value = {
        field.name: getattr(result, field.name)
        for field in dataclasses.fields(result)
        if field.name != "transcript"
    }
    # RunResult.transcript is bytes. Its lossless canonical-line representation
    # is JSON-safe and is also exported at the vector's top level.
    value["transcript"] = {"canonical_lines": transcript_lines}
    return value


def state_snapshot(runtime: ScenarioRuntime) -> dict[str, Any]:
    """The exact value passed to ScenarioRuntime._state_hash before hashing."""
    return {
        "accounting": runtime.accounting.snapshot_for_hash(),
        "canonical_registry_hash": runtime.registry.state_hash(),
        "dealer_cohort": runtime.dealers.snapshot_for_hash(),
        "external_buyer": runtime.external_buyer.snapshot_for_hash(),
        "leveraged_funds": runtime.leveraged_funds.snapshot_for_hash(),
        "participants": [participant.snapshot_for_hash() for participant in runtime.participants],
        "audience_delivery": runtime.audience_router.snapshot_for_hash(),
        "communication_acts": [row.to_dict() for row in runtime.communication_acts],
        "commitments": runtime.commitments.snapshot_for_hash(),
        "compression": runtime.compression.snapshot_for_hash(),
        "households": runtime.households.snapshot_for_hash(),
        "population": runtime.population.snapshot_for_hash(),
        "population_views": [row.to_dict() for row in runtime.population_views],
        "monitoring": runtime.monitoring.snapshot_for_hash(),
        "next_morning_book": runtime.next_morning_book.to_dict() if runtime.next_morning_book else None,
        "reports": [row.to_dict() for row in runtime.reports],
        "repo_agreement": runtime.repo.snapshot_for_hash(),
        "staff": runtime.staff.snapshot_for_hash(),
        "staff_assessments": {
            key: runtime.assessments[key].to_dict() for key in sorted(runtime.assessments)
        },
        "staff_tasks": {key: runtime.tasks[key].to_dict() for key in sorted(runtime.tasks)},
        "staff_review": runtime.staff_review.to_dict() if runtime.staff_review else None,
        "treasury_market": runtime.market.snapshot_for_hash(),
    }


def material_snapshot(runtime: ScenarioRuntime) -> dict[str, Any]:
    """The exact value passed to ScenarioRuntime._material_state_hash before hashing."""
    return {
        "accounting": runtime.accounting.snapshot_for_hash(),
        "canonical_registry": runtime.registry.state_hash(),
        "households": runtime.households.snapshot_for_hash(),
        "population": runtime.population.snapshot_for_hash(),
        "treasury_market": runtime.market.snapshot_for_hash(),
    }


def capture_run_stdout(scenario_dir: Path, package_id: str, request: str | None) -> str:
    args = argparse.Namespace(
        scenario=str(scenario_dir),
        package=package_id,
        request=request,
        transcript=None,
        report_endogeneity=False,
    )
    return capture_stdout(run_command, args)


def capture_replay_stdout(scenario_dir: Path) -> str:
    return capture_stdout(replay_command, argparse.Namespace(scenario=str(scenario_dir)))


def capture_play(scenario_dir: Path) -> dict[str, Any]:
    output = io.StringIO()
    args = argparse.Namespace(scenario=str(scenario_dir), package="MEASURED_FIRMING")
    with (
        patch("sys.stdin.isatty", return_value=True),
        patch("builtins.input", side_effect=PLAY_SCRIPT),
        contextlib.redirect_stdout(output),
    ):
        exit_code = play_command(args)
    scenario = validate_scenario(scenario_dir)
    return {
        "format_version": 1,
        "vector_kind": "play_script",
        "source_identity": source_identity(
            scenario_dir, "MEASURED_FIRMING", None, scenario.initialization.value["seed"]
        ),
        "scenario_hash": scenario.scenario_hash,
        "package": "MEASURED_FIRMING",
        "request": None,
        "seed": scenario.initialization.value["seed"],
        "script": list(PLAY_SCRIPT),
        "stdout": output.getvalue(),
        "exit_code": exit_code,
    }


def capture_stdout(command: Any, args: argparse.Namespace) -> str:
    output = io.StringIO()
    with contextlib.redirect_stdout(output):
        exit_code = command(args)
    if exit_code != 0:
        raise RuntimeError(f"oracle CLI command failed with exit code {exit_code}")
    return output.getvalue()


@contextlib.contextmanager
def copied_reseeded_scenario(source: Path, seed: int) -> Iterator[Path]:
    """Reseal a disposable copy; authored scenario inputs are never mutated."""
    with tempfile.TemporaryDirectory() as temporary:
        scenario = Path(temporary) / source.name
        shutil.copytree(source, scenario)
        initialization = scenario / "initialization.json"
        value = json.loads(initialization.read_text(encoding="utf-8"))
        value["seed"] = seed
        initialization.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
        seal_scenario(scenario)
        yield scenario


def vector_name(package_id: str, request_mode: RequestMode | None) -> str:
    request = request_mode.value if request_mode is not None else "none"
    return f"{package_id}__{request}.json"


def write_vector(path: Path, vector: dict[str, Any]) -> None:
    path.write_bytes(canonical_bytes(vector) + b"\n")


if __name__ == "__main__":
    raise SystemExit(main())
