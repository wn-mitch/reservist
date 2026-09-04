from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path

from engine.harness.office import OfficeHarness
from engine.manifest import ManifestValidationError
from engine.scenario import ScenarioRuntime, seal_scenario, validate_scenario


DEFAULT_SCENARIO = Path("scenarios/mvp_2006_cycle")


def _scenario(value: str | None) -> Path:
    return Path(value) if value else DEFAULT_SCENARIO


def validate_command(args: argparse.Namespace) -> int:
    scenario = validate_scenario(_scenario(args.scenario))
    print(f"validation passed: {scenario.scenario_hash}")
    return 0


def freeze_command(args: argparse.Namespace) -> int:
    replay_hash = seal_scenario(_scenario(args.scenario))
    print(f"scenario sealed: {replay_hash}")
    return 0


def run_command(args: argparse.Namespace) -> int:
    runtime = ScenarioRuntime(validate_scenario(_scenario(args.scenario)))
    result = runtime.run_all()
    if args.transcript:
        Path(args.transcript).write_bytes(result.transcript)
    print(f"scenario_hash={result.scenario_hash}")
    print(f"state_hash={result.state_hash}")
    print(f"events={len(runtime.ledger.events)}")
    return 0


def replay_command(args: argparse.Namespace) -> int:
    scenario = validate_scenario(_scenario(args.scenario))
    first = ScenarioRuntime(scenario).run_all()
    second = ScenarioRuntime(scenario).run_all()
    if first != second:
        print("replay mismatch", file=sys.stderr)
        return 1
    with tempfile.TemporaryDirectory() as directory:
        first_path = Path(directory) / "first.jsonl"
        second_path = Path(directory) / "second.jsonl"
        first_path.write_bytes(first.transcript)
        second_path.write_bytes(second.transcript)
        if first_path.read_bytes() != second_path.read_bytes():
            print("transcript byte mismatch", file=sys.stderr)
            return 1
    print(
        f"replay passed: {first.scenario_hash}; "
        f"state={first.state_hash}; transcript_bytes={len(first.transcript)}"
    )
    return 0


def play_command(args: argparse.Namespace) -> int:
    runtime = ScenarioRuntime(validate_scenario(_scenario(args.scenario)))
    runtime.run_until_first_delivery()
    harness = OfficeHarness(runtime.player_records, runtime.advance_next)
    print(harness.render_morning_book())
    if not sys.stdin.isatty():
        return 0
    print("\nCommands: inspect <number> | advance | book | quit")
    while True:
        try:
            command = input("reservist> ").strip()
        except EOFError:
            break
        if command == "quit":
            break
        if command == "book":
            print(harness.render_morning_book())
        elif command == "advance":
            print(harness.advance())
        elif command.startswith("inspect "):
            try:
                print(harness.inspect(int(command.split(maxsplit=1)[1])))
            except (ValueError, IndexError) as exc:
                print(f"invalid inspect command: {exc}")
        else:
            print("Commands: inspect <number> | advance | book | quit")
    return 0


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(prog="reservist")
    subcommands = result.add_subparsers(dest="command", required=True)
    for name, handler in (
        ("validate", validate_command),
        ("freeze", freeze_command),
        ("replay-check", replay_command),
        ("play", play_command),
    ):
        command = subcommands.add_parser(name)
        command.add_argument("scenario", nargs="?")
        command.set_defaults(handler=handler)
    run = subcommands.add_parser("run")
    run.add_argument("scenario", nargs="?")
    run.add_argument("--transcript")
    run.set_defaults(handler=run_command)
    return result


def main() -> int:
    args = parser().parse_args()
    try:
        return args.handler(args)
    except ManifestValidationError as exc:
        print(f"ERROR [{exc.category}] {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
