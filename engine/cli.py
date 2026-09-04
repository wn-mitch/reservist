from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path

from engine.harness.fomc_room import FomcRoomHarness
from engine.harness.office import OfficeHarness
from engine.harness.operations_room import OperationsRoomHarness
from engine.harness.request import RequestHarness
from engine.harness.review import ReviewHarness
from engine.harness.statement import StatementHarness
from engine.harness.wire import WorldWireHarness
from engine.manifest import ManifestValidationError
from engine.packages import PACKAGES
from engine.scenario import ScenarioRuntime, seal_scenario, validate_scenario
from engine.staff.analytical_task import RequestMode


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
    runtime = ScenarioRuntime(
        validate_scenario(_scenario(args.scenario)),
        package_id=args.package,
        request_mode=args.request,
    )
    result = runtime.run_all()
    if args.transcript:
        Path(args.transcript).write_bytes(result.transcript)
    print(f"scenario_hash={result.scenario_hash}")
    print(f"state_hash={result.state_hash}")
    print(f"events={len(runtime.ledger.events)}")
    print(f"package={result.package_id}")
    for receipt in result.receipts:
        print(
            f"receipt={receipt['stage']} status={receipt['status']} "
            f"owner={receipt['owner_id']}"
        )
        clearing = receipt["details"].get("clearing_result")
        if clearing is not None:
            print(
                f"market_price={clearing['price']} filled={clearing['filled_quantity']} "
                f"residual_buy={clearing['residual']['BUY']} "
                f"residual_sell={clearing['residual']['SELL']} "
                f"source={clearing['source_kind']}"
            )
    if args.report_endogeneity:
        for row in result.endogeneity_report:
            print(
                f"endogeneity={row['proposition']} source_kind={row['source_kind']} "
                f"source={row['source']}"
            )
    return 0


def replay_command(args: argparse.Namespace) -> int:
    scenario = validate_scenario(_scenario(args.scenario))
    replay_rows = []
    for package_id in PACKAGES:
        first = ScenarioRuntime(
            scenario, package_id=package_id, request_mode=RequestMode.NORMAL
        ).run_all()
        second = ScenarioRuntime(
            scenario, package_id=package_id, request_mode=RequestMode.NORMAL
        ).run_all()
        if first != second:
            print(f"replay mismatch: {package_id}", file=sys.stderr)
            return 1
        with tempfile.TemporaryDirectory() as directory:
            first_path = Path(directory) / "first.jsonl"
            second_path = Path(directory) / "second.jsonl"
            first_path.write_bytes(first.transcript)
            second_path.write_bytes(second.transcript)
            if first_path.read_bytes() != second_path.read_bytes():
                print(f"transcript byte mismatch: {package_id}", file=sys.stderr)
                return 1
        replay_rows.append(
            f"{package_id}: state={first.state_hash}; "
            f"transcript_bytes={len(first.transcript)}"
        )
    print(f"replay passed: {scenario.scenario_hash}")
    for row in replay_rows:
        print(row)
    return 0


def play_command(args: argparse.Namespace) -> int:
    runtime = ScenarioRuntime(
        validate_scenario(_scenario(args.scenario)), package_id=args.package
    )
    runtime.run_until_first_delivery()
    harness = OfficeHarness(
        runtime.player_records,
        runtime.advance_to_next_consequential_event,
        runtime.routing_account,
    )
    print(harness.render_morning_book())
    if not sys.stdin.isatty():
        return 0
    print(
        "\nCommands: inspect <number> | ask markets [accelerated] | advance | book | verbs | fomc | "
        "propose <package> | operations | statement | wire | review | quit"
    )
    while True:
        try:
            command = input("reservist> ").strip()
        except EOFError:
            break
        if command == "quit":
            break
        if command == "book":
            print(harness.render_morning_book())
        elif command == "verbs":
            print("Available: " + " | ".join(runtime.available_verbs()))
        elif command == "fomc":
            print(
                FomcRoomHarness(
                    runtime.participants,
                    runtime.fomc_decision,
                    runtime.participant_labels,
                ).render()
            )
        elif command == "operations":
            print(OperationsRoomHarness(tuple(runtime.receipts)).render())
        elif command == "statement":
            statement = StatementHarness()
            if runtime.communication_acts:
                print(statement.render_published(runtime.communication_acts[-1]))
            else:
                print(statement.render_preview(runtime.authorized_statement_claims()))
        elif command == "wire":
            print(
                WorldWireHarness(
                    tuple(runtime.reports), runtime.population_views
                ).render()
            )
        elif command == "review":
            review = ReviewHarness(runtime.next_morning_book, runtime.staff_review)
            print(review.render_next_morning_book())
            print()
            print(review.render_staff_review())
        elif command in {"ask markets", "ask markets accelerated"}:
            mode = (
                RequestMode.ACCELERATED
                if command.endswith("accelerated")
                else RequestMode.NORMAL
            )
            try:
                task = runtime.request_follow_up(mode)
            except ValueError as exc:
                print(f"request rejected: {exc}")
                continue
            unit = runtime.staff.unit(task.assigned_unit_id)
            print(RequestHarness().render(task, unit.display_name))
        elif command.startswith("propose "):
            package = command.split(maxsplit=1)[1]
            if package not in PACKAGES:
                print(f"unknown package: {package}")
                continue
            if runtime.fomc_decision is not None:
                print("The Committee has already recorded its decision.")
                continue
            runtime.package_id = package
            while runtime.fomc_decision is None and runtime.advance_next():
                pass
            print(
                FomcRoomHarness(
                    runtime.participants,
                    runtime.fomc_decision,
                    runtime.participant_labels,
                ).render()
            )
        elif command == "advance":
            print(harness.advance())
        elif command.startswith("inspect "):
            try:
                print(harness.inspect(int(command.split(maxsplit=1)[1])))
            except (ValueError, IndexError) as exc:
                print(f"invalid inspect command: {exc}")
        else:
            print(
                "Commands: inspect <number> | ask markets [accelerated] | advance | book | verbs | fomc | "
                "propose <package> | operations | statement | wire | review | quit"
            )
    return 0


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(prog="reservist")
    subcommands = result.add_subparsers(dest="command", required=True)
    for name, handler in (
        ("validate", validate_command),
        ("freeze", freeze_command),
        ("replay-check", replay_command),
    ):
        command = subcommands.add_parser(name)
        command.add_argument("scenario", nargs="?")
        command.set_defaults(handler=handler)
    play = subcommands.add_parser("play")
    play.add_argument("scenario", nargs="?")
    play.add_argument(
        "--package", choices=tuple(PACKAGES), default="MEASURED_FIRMING"
    )
    play.set_defaults(handler=play_command)
    run = subcommands.add_parser("run")
    run.add_argument("scenario", nargs="?")
    run.add_argument("--transcript")
    run.add_argument("--report-endogeneity", action="store_true")
    run.add_argument("--package", choices=tuple(PACKAGES), default="MEASURED_FIRMING")
    run.add_argument(
        "--request",
        type=str.upper,
        choices=tuple(mode.value for mode in RequestMode),
        default=RequestMode.NORMAL.value,
    )
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
