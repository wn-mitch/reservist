#!/usr/bin/env python3
from __future__ import annotations

import argparse
import csv
import json
import re
import shutil
import sys
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).absolute().parent
REPO_ROOT = ROOT.parent
SCHEMA = json.loads((ROOT / "schema.json").read_text())
UNKNOWN = SCHEMA["null_token"]
NONE = "NONE"
PROFILE_ID = "profile.early_2006.bernankey"
EXPECTED_CHANNELS = {
    "channel.external_demand",
    "channel.import_supply",
    "channel.us_duration_demand",
    "channel.dollar_funding_fx",
    "channel.energy_supply",
    "channel.foreign_financial_stress",
    "channel.freight_shipping",
}
EXPECTED_PROBES = {
    "probe.composition.burrow_bank",
    "probe.composition.treasury_basis_trade",
    "probe.composition.amazon_region_replacement",
}
EXPECTED_RUNTIME_BLOCKERS = {
    "probe.composition.burrow_bank": {
        "period_content", "legal_content", "opening_accounts", "residual_values", "calibration"
    },
    "probe.composition.treasury_basis_trade": {
        "opening_positions", "counterparty_content", "market_parameters", "calibration"
    },
    "probe.composition.amazon_region_replacement": {
        "opening_physical_state", "regional_shares", "process_parameters", "calibration"
    },
}
EXPECTED_INSTRUMENTS = {
    "type.instrument_family.treasury_bill": ("Circuit", "Short-duration supply"),
    "type.instrument_family.treasury_note": ("Core", "Cash leg"),
    "type.instrument_family.treasury_bond": ("Core", "Long-duration cash leg"),
    "type.instrument_family.tips": ("Long tail", "Inflation-linked duration"),
    "type.instrument_family.reserves": ("Core", "Final cash settlement"),
    "type.instrument_family.deposits": ("Circuit", "Bank funding"),
    "type.instrument_family.money_fund_share": ("Circuit", "Investor redemption"),
    "type.instrument_family.repo": ("Core", "Leveraged funding"),
    "type.instrument_family.loans": ("Circuit", "Transmission from bank funding"),
    "type.instrument_family.agency_mbs": ("Long tail", "Convexity hedging"),
    "type.instrument_family.corporate_bonds": ("Long tail", "Credit-spread contagion"),
    "type.instrument_family.equity": ("Core", "Dealer, bank, fund"),
    "type.instrument_family.swaps": ("Long tail", "Alternative duration hedging"),
    "type.instrument_family.futures": ("Core", "Short futures leg"),
    "type.instrument_family.guarantees_credit_lines": ("Long tail", "Contingent liquidity calls"),
}
PROBE_PRODUCTS = {"TIMBER", "LUMBER", "GREEN_COFFEE", "ROASTED_COFFEE", "FEED_GRAIN", "LIVE_HOGS", "PORK"}
GAP_CATEGORIES = {
    "schema", "key", "vocabulary", "type", "endpoint", "unit", "state_transition",
    "relationship", "fallback_cycle", "catalog_eligibility", "profile", "external_channel",
    "market_interface", "composition_probe", "instrument", "product", "residual",
}


def read_table(name: str) -> list[dict[str, str]]:
    path = ROOT / name
    if not path.exists():
        return []
    with path.open(newline="") as handle:
        return list(csv.DictReader(handle))


def read_header(path: Path) -> list[str]:
    with path.open(newline="") as handle:
        return next(csv.reader(handle), [])


def write_table(path: Path, fields: list[str], rows: list[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields, extrasaction="ignore", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def table_sort_key(name: str, row: dict[str, str]) -> tuple[str, ...]:
    keys = SCHEMA["table_keys"][name]
    return tuple(row[key] for key in keys) + tuple(row[field] for field in SCHEMA["tables"][name])


def issue(errors: list[dict[str, str]], category: str, record_id: str, message: str) -> None:
    if category not in GAP_CATEGORIES:
        raise ValueError(f"unregistered gap category {category}")
    errors.append({"severity": "error", "category": category, "record_id": record_id, "issue": message})


def inventory_paths(name: str) -> list[Path]:
    return sorted((ROOT / "inventory").glob(f"*/{name}"))


def inspect_inventory() -> tuple[dict[str, list[dict[str, str]]], list[dict[str, str]]]:
    staged: dict[str, list[dict[str, str]]] = {}
    errors: list[dict[str, str]] = []
    for name, fields in SCHEMA["tables"].items():
        candidates: list[tuple[Path, int, dict[str, str]]] = []
        for path in inventory_paths(name):
            header = read_header(path)
            duplicates = sorted(value for value, count in Counter(header).items() if count > 1)
            if duplicates:
                issue(errors, "schema", str(path.relative_to(ROOT)),
                      f"duplicate header field(s): {','.join(duplicates)}")
                continue
            if header != fields:
                missing = [field for field in fields if field not in header]
                extra = [field for field in header if field not in fields]
                issue(errors, "schema", str(path.relative_to(ROOT)),
                      f"header mismatch; missing={missing}; extra={extra}; expected={fields}")
                continue
            with path.open(newline="") as handle:
                reader = csv.DictReader(handle)
                for row_number, raw in enumerate(reader, start=2):
                    row = {field: (raw[field].strip() if raw[field] is not None else "") for field in fields}
                    for field, value in row.items():
                        if not value:
                            issue(errors, "schema", f"{path.relative_to(ROOT)}:{row_number}:{field}",
                                  "empty authored field; use NONE, null, or an allowed UNKNOWN explicitly")
                    candidates.append((path, row_number, row))
        key_fields = SCHEMA["table_keys"][name]
        grouped: dict[tuple[str, ...], list[tuple[Path, int, dict[str, str]]]] = defaultdict(list)
        for item in candidates:
            key = tuple(item[2][field] for field in key_fields)
            if any(value == UNKNOWN for value in key):
                issue(errors, "key", f"{item[0].relative_to(ROOT)}:{item[1]}",
                      f"UNKNOWN is not permitted in key {key_fields}")
            grouped[key].append(item)
        rows: list[dict[str, str]] = []
        for key, items in sorted(grouped.items()):
            normalized = {tuple(item[2][field] for field in fields) for item in items}
            if len(normalized) > 1:
                locations = ", ".join(f"{item[0].relative_to(ROOT)}:{item[1]}" for item in items)
                issue(errors, "key", "|".join(key), f"conflicting duplicate key in {locations}")
                continue
            rows.append(items[0][2])
        staged[name] = sorted(rows, key=lambda row, table=name: table_sort_key(table, row))
    return staged, errors


def init_tables(_: argparse.Namespace) -> int:
    for name, fields in SCHEMA["tables"].items():
        if not (ROOT / name).exists():
            write_table(ROOT / name, fields, [])
    return 0


def import_inventory(_: argparse.Namespace) -> int:
    staged, errors = inspect_inventory()
    if errors:
        emit_issues(errors)
        print(f"inventory import failed: {len(errors)} malformed source issue(s)", file=sys.stderr)
        return 1
    for name, fields in SCHEMA["tables"].items():
        write_table(ROOT / name, fields, staged[name])
    print(f"inventory imported authoritatively: {sum(len(rows) for rows in staged.values())} rows")
    return 0


def tables_from_root(errors: list[dict[str, str]]) -> dict[str, list[dict[str, str]]]:
    tables: dict[str, list[dict[str, str]]] = {}
    for name, fields in SCHEMA["tables"].items():
        path = ROOT / name
        if not path.exists():
            issue(errors, "schema", name, "missing normalized table")
            tables[name] = []
            continue
        header = read_header(path)
        if header != fields:
            issue(errors, "schema", name, f"normalized header mismatch; expected={fields}; actual={header}")
            tables[name] = []
            continue
        tables[name] = read_table(name)
    return tables


def eligibility_details(
    tables: dict[str, list[dict[str, str]]], *, ignore_declaration: bool = False,
) -> dict[str, tuple[bool, list[str], list[str]]]:
    entities = {row["catalog_id"]: row for row in tables["entities.csv"]}
    type_ids = {row["type_id"] for row in tables["types.csv"]} | {
        row["catalog_id"] for row in entities.values() if row["entry_class"] == "type"
    }
    rel_ids: set[str] = set()
    for row in tables["relationships.csv"]:
        rel_ids.update((row["subject_entry_id"], row["object_entry_id"]))
    tx_ids: set[str] = set()
    for row in tables["transmissions.csv"]:
        tx_ids.update((row["producing_entry_id"], row["consuming_entry_id"]))
    probes = {row["catalog_id"] for row in tables["probe_coverage.csv"] if row["coverage_status"] == "covered"}
    interfaces = {row["provider_entry_id"] for row in tables["external_market_channels.csv"]}
    residual_complete = {row["catalog_id"] for row in tables["residual_reconciliation.csv"] if row["status"] == "complete"}
    details: dict[str, tuple[bool, list[str], list[str]]] = {}
    for catalog_id, row in entities.items():
        if row["entry_class"] != "instance":
            continue
        checks = ["explicit_type"] if row["instance_of"] in type_ids else []
        blockers: list[str] = []
        if row["instance_of"] not in type_ids:
            blockers.append("type")
        fallback = row["fallback_entry_id"]
        fallback_ok = fallback == UNKNOWN or fallback in entities or (
            fallback == NONE and row["identity_clade"] == "BoundaryAdapter"
        )
        if fallback_ok:
            checks.append("fallback")
        else:
            blockers.append("fallback")
        if catalog_id in rel_ids:
            checks.append("relationship")
        else:
            blockers.append("relationship")
        if catalog_id in tx_ids:
            checks.append("transmission")
        else:
            blockers.append("transmission")
        if catalog_id in probes:
            checks.append("probe")
        else:
            blockers.append("probe")
        if row["identity_clade"] == "BoundaryAdapter":
            if catalog_id in interfaces:
                checks.append("market_interface")
            if row["residual_counterpart_id"] not in {NONE, UNKNOWN}:
                if catalog_id in residual_complete:
                    checks.append("residual")
                else:
                    blockers.append("residual")
        contract_eligible = not blockers
        eligible = contract_eligible if ignore_declaration else (
            row["selectable_in_manifest"] == "true" and contract_eligible
        )
        if not ignore_declaration and row["selectable_in_manifest"] != "true":
            blockers = ["not_catalog_eligible"]
        details[catalog_id] = (eligible, checks, sorted(set(blockers)))
    return details


def collect_errors() -> tuple[dict[str, list[dict[str, str]]], list[dict[str, str]]]:
    errors: list[dict[str, str]] = []
    tables = tables_from_root(errors)
    if errors:
        return tables, errors

    # Lifecycle, required values, vocabularies, and composite keys.
    for name, lifecycle in SCHEMA["table_lifecycle"].items():
        rows = tables[name]
        if lifecycle == "required" and not rows:
            issue(errors, "schema", name, "required table is empty")
        if lifecycle == "inapplicable" and rows:
            issue(errors, "schema", name, "inapplicable table must contain no rows")
    for name, rows in tables.items():
        allowed_unknown = set(SCHEMA.get("allow_unknown", {}).get(name, []))
        fields = SCHEMA["tables"][name]
        key_fields = SCHEMA["table_keys"][name]
        seen: set[tuple[str, ...]] = set()
        for number, row in enumerate(rows, start=2):
            for field in fields:
                value = row.get(field, "")
                if not value:
                    issue(errors, "schema", f"{name}:{number}:{field}", "empty normalized field")
                if value == UNKNOWN and field not in allowed_unknown:
                    issue(errors, "schema", f"{name}:{number}:{field}", "UNKNOWN is not allowed in this field")
            key = tuple(row[field] for field in key_fields)
            if key in seen:
                issue(errors, "key", "|".join(key), f"duplicate composite key in {name}")
            seen.add(key)
        vocab_fields = {
            key.removeprefix(name + "."): vocab
            for key, vocab in SCHEMA["field_vocabularies"].items() if key.startswith(name + ".")
        }
        for row in rows:
            record = "|".join(row[field] for field in key_fields)
            for field, vocab in vocab_fields.items():
                if row[field] not in SCHEMA["vocabularies"][vocab]:
                    issue(errors, "vocabulary", record,
                          f"{name}.{field}={row[field]!r} is outside {vocab}")

    entities = {row["catalog_id"]: row for row in tables["entities.csv"]}
    instances = {key: row for key, row in entities.items() if row["entry_class"] == "instance"}
    types = {row["type_id"]: row for row in tables["types.csv"]}
    types.update({key: row for key, row in entities.items() if row["entry_class"] == "type"})
    for catalog_id, row in entities.items():
        if row["identity_clade"] == "Instrument":
            issue(errors, "type", catalog_id, "Instrument is not an identity clade")
        if row["entry_class"] == "instance":
            type_row = types.get(row["instance_of"])
            if not type_row:
                issue(errors, "type", catalog_id, f"unresolved instance_of {row['instance_of']}")
            elif type_row["identity_clade"] != row["identity_clade"]:
                issue(errors, "type", catalog_id,
                      f"instance clade {row['identity_clade']} conflicts with type clade {type_row['identity_clade']}")

    state_by_id = {row["state_id"]: row for row in tables["owned_state.csv"]}
    transition_ids = {row["state_id"] for row in tables["owned_state_transitions.csv"]}
    for state_id, row in state_by_id.items():
        owner = entities.get(row["owner_id"])
        if not owner or owner["entry_class"] != "instance":
            issue(errors, "endpoint", state_id, f"state owner is not an instance: {row['owner_id']}")
        elif owner["completeness_state"] == "identity_only":
            issue(errors, "catalog_eligibility", row["owner_id"], "identity_only entry owns executable state")
        if state_id not in transition_ids:
            issue(errors, "state_transition", state_id, "owned state lacks an accepted transition")
        if row["conserved"] == "true" and row["unit"] in {NONE, UNKNOWN}:
            issue(errors, "unit", state_id, "conserved state lacks a unit")
    for row in tables["owned_state_transitions.csv"]:
        if row["state_id"] not in state_by_id:
            issue(errors, "endpoint", row["state_id"], "transition references an unknown state")

    for row in tables["relationships.csv"]:
        relationship_id = row["relationship_id"]
        for field in ("subject_entry_id", "object_entry_id", "canonical_owner_id"):
            if row[field] not in instances:
                issue(errors, "relationship", relationship_id, f"unresolved instance endpoint {field}={row[field]}")
        for field in ("effective_period", "observability", "lifecycle_and_exit", "witness_kind"):
            if row[field] in {UNKNOWN, NONE}:
                issue(errors, "relationship", relationship_id, f"missing {field}")

    transmission_ids = {row["transmission_id"] for row in tables["transmissions.csv"]}
    for row in tables["transmissions.csv"]:
        transmission_id = row["transmission_id"]
        for field in ("producing_entry_id", "consuming_entry_id", "transformation_owner_id"):
            if row[field] not in instances:
                issue(errors, "endpoint", transmission_id, f"unresolved instance endpoint {field}={row[field]}")
        if row["producing_state_or_output"].startswith("state.") and row["producing_state_or_output"] not in state_by_id:
            issue(errors, "endpoint", transmission_id,
                  f"unresolved producing state {row['producing_state_or_output']}")
        if row["unit"] == UNKNOWN:
            issue(errors, "unit", transmission_id, "transmission unit is unresolved")
        if row["fallback_behavior"] in {UNKNOWN, NONE}:
            issue(errors, "endpoint", transmission_id, "transmission fallback behavior is unresolved")
    for row in tables["transmission_probes.csv"]:
        if row["transmission_id"] not in transmission_ids:
            issue(errors, "endpoint", row["transmission_id"], "probe references an unknown transmission")

    # Fallback graph and terminal residual exception.
    provider_rows = tables["external_channel_providers.csv"]
    residual_providers = {row["provider_entry_id"] for row in provider_rows if row["coverage_role"] == "residual"}
    graph: dict[str, str] = {}
    for catalog_id, row in instances.items():
        fallback = row["fallback_entry_id"]
        if fallback in instances:
            graph[catalog_id] = fallback
        elif fallback == NONE:
            if catalog_id not in residual_providers and row["entry_class"] == "instance":
                issue(errors, "fallback_cycle", catalog_id, "NONE fallback is reserved for residual BoundaryAdapter providers")
        elif fallback != UNKNOWN:
            issue(errors, "endpoint", catalog_id, f"unresolved fallback {fallback}")
    for start in graph:
        order: list[str] = []
        current = start
        while current in graph:
            if current in order:
                cycle = order[order.index(current):] + [current]
                issue(errors, "fallback_cycle", start, f"fallback cycle: {' -> '.join(cycle)}")
                break
            order.append(current)
            current = graph[current]

    products = {row["product_code"]: row for row in tables["product_families.csv"]}
    for code, row in products.items():
        if row["unit"] in {NONE, UNKNOWN}:
            issue(errors, "unit", code, "product family lacks a canonical unit")
    for row in tables["transformations.csv"]:
        transformation_id = row["transformation_id"]
        for field in ("input_product_code", "output_product_code"):
            if row[field] not in products:
                issue(errors, "product", transformation_id, f"unresolved {field}={row[field]}")
        if row["owner_entry_id"] not in instances or row["fallback_entry_id"] not in instances:
            issue(errors, "endpoint", transformation_id, "transformation owner or fallback is unresolved")
        inp = products.get(row["input_product_code"])
        out = products.get(row["output_product_code"])
        if inp and out and inp["material_stage"] == out["material_stage"]:
            issue(errors, "product", transformation_id, "transformation does not change material stage")

    # Profile roles do not mutate eligibility.
    profiles = {row["profile_id"]: row for row in tables["world_profiles.csv"]}
    roles = tables["profile_catalog_roles.csv"]
    role_counts = Counter((row["profile_id"], row["catalog_id"]) for row in roles)
    for profile_id, profile_row in profiles.items():
        player = instances.get(profile_row["player_entry_id"])
        if not player or player["identity_clade"] != "Person":
            issue(errors, "profile", profile_id, "player_entry_id must resolve to one Person instance")
        for catalog_id in instances:
            if role_counts[(profile_id, catalog_id)] != 1:
                issue(errors, "profile", catalog_id,
                      f"instance must have exactly one {profile_id} planning role")
    for row in roles:
        if row["profile_id"] not in profiles or row["catalog_id"] not in instances:
            issue(errors, "profile", row["catalog_id"], "planning role has an unresolved profile or instance")
        provider = row["candidate_provider_entry_id"]
        if provider != NONE and provider not in instances:
            issue(errors, "profile", row["catalog_id"], f"unresolved candidate provider {provider}")
    relationship_triples = {
        (row["relationship_family"], row["subject_entry_id"], row["object_entry_id"])
        for row in tables["relationships.csv"]
    }
    for row in tables["profile_required_offices.csv"]:
        profile_row = profiles.get(row["profile_id"])
        office = instances.get(row["office_id"])
        if not profile_row or not office or office["identity_clade"] != "Office":
            issue(errors, "profile", row["office_id"], "required office must resolve to an Office instance")
        elif ("HOLDS", profile_row["player_entry_id"], row["office_id"]) not in relationship_triples:
            issue(errors, "profile", row["office_id"], "profile player lacks an effective HOLDS relationship")
    for row in tables["presentation_refs.csv"]:
        asset = REPO_ROOT / row["asset_path"]
        if Path(row["asset_path"]).is_absolute() or not asset.is_file():
            issue(errors, "profile", row["catalog_id"], f"asset_path is not an existing repository-relative file: {row['asset_path']}")

    # Channel-first external coverage and interface typing.
    channels = {row["channel_code"] for row in tables["external_channels.csv"]}
    if channels != EXPECTED_CHANNELS:
        issue(errors, "external_channel", "external_channels.csv",
              f"expected seven closed channels; missing={sorted(EXPECTED_CHANNELS - channels)}; extra={sorted(channels - EXPECTED_CHANNELS)}")
    providers_by_channel: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for row in provider_rows:
        providers_by_channel[(row["profile_id"], row["channel_code"])].append(row)
        if row["provider_entry_id"] not in instances or row["channel_code"] not in channels:
            issue(errors, "external_channel", row["provider_entry_id"], "provider row has an unresolved channel or instance")
    for profile_id in profiles:
        for channel_code in EXPECTED_CHANNELS:
            key = (profile_id, channel_code)
            rows = providers_by_channel[key]
            residuals = [row for row in rows if row["coverage_role"] == "residual"]
            if len(residuals) != 1:
                issue(errors, "external_channel", "|".join(key),
                      f"channel requires exactly one residual; found {len(residuals)}")
    scopes = {(row["profile_id"], row["provider_entry_id"]) for row in tables["provider_scopes.csv"]}
    interfaces = {row["interface_code"]: row for row in tables["market_interfaces.csv"]}
    instruments = {row["instrument_code"]: row for row in tables["instrument_families.csv"]}
    fallback_contracts = {(row["catalog_id"], row["boundary_item"]): row for row in tables["entity_fallback_contracts.csv"]}
    bindings_by_provider: dict[tuple[str, str, str], list[dict[str, str]]] = defaultdict(list)
    for row in tables["external_market_channels.csv"]:
        key = (row["profile_id"], row["channel_code"], row["provider_entry_id"])
        bindings_by_provider[key].append(row)
        interface = interfaces.get(row["interface_code"])
        if not interface:
            issue(errors, "market_interface", row["interface_code"], "unknown market interface")
            continue
        allowed = interface["allowed_direction"]
        if allowed != "bidirectional" and row["direction"] != allowed:
            issue(errors, "market_interface", row["interface_code"], "binding direction is not allowed")
        family = row["family_code"]
        if interface["unit_source"] == "product_family":
            if interface["family_kind"] != "product" or family not in products:
                issue(errors, "market_interface", row["interface_code"], "product-family unit requires a known product family")
        elif family != NONE:
            duration_exception = row["channel_code"] == "channel.us_duration_demand" and family in instruments
            if not duration_exception:
                issue(errors, "market_interface", row["interface_code"], "fixed-unit interface has an invalid family binding")
        contract = fallback_contracts.get((row["provider_entry_id"], row["interface_code"]))
        if not contract:
            issue(errors, "external_channel", row["provider_entry_id"], f"missing fallback contract for {row['interface_code']}")
        elif row["coverage_role"] == "residual" and contract["preserves_or_loses"] != "terminal_exogenous_input":
            issue(errors, "external_channel", row["provider_entry_id"], "residual interface is not terminal_exogenous_input")
    for row in provider_rows:
        key = (row["profile_id"], row["channel_code"], row["provider_entry_id"])
        if not bindings_by_provider[key]:
            issue(errors, "external_channel", row["provider_entry_id"], "provider lacks a market-interface binding")
        if (row["profile_id"], row["provider_entry_id"]) not in scopes:
            issue(errors, "external_channel", row["provider_entry_id"], "provider lacks a non-owning scope")
        if row["coverage_role"] == "segment":
            residual = next((item["provider_entry_id"] for item in providers_by_channel[(row["profile_id"], row["channel_code"])] if item["coverage_role"] == "residual"), None)
            if residual and instances[row["provider_entry_id"]]["fallback_entry_id"] != residual:
                issue(errors, "external_channel", row["provider_entry_id"], f"segment fallback must be the channel residual {residual}")

    # Instrument family and bucket closure.
    if set(instruments) != set(EXPECTED_INSTRUMENTS):
        issue(errors, "instrument", "instrument_families.csv",
              f"expected fifteen families; missing={sorted(set(EXPECTED_INSTRUMENTS) - set(instruments))}; extra={sorted(set(instruments) - set(EXPECTED_INSTRUMENTS))}")
    attribute_fields = [
        "duration", "collateral_role", "settlement_role", "demandability", "credit_state", "currency",
        "quantity_model", "contingency", "liquidity", "seniority", "priority", "rollover", "rate",
        "convertibility", "margin", "counterparty_exposure",
    ]
    for code, expected in EXPECTED_INSTRUMENTS.items():
        row = instruments.get(code)
        if not row:
            continue
        if row["dependency_cut"] != expected[0] or not row["required_channel"].startswith(expected[1]):
            issue(errors, "instrument", code, "dependency cut or required-channel text differs from the normative inventory")
        for field in attribute_fields:
            if row[field] in {"", UNKNOWN, NONE}:
                issue(errors, "instrument", code, f"missing normative attribute {field}; literal null is the inapplicable value")
    buckets = {row["bucket_id"]: row for row in tables["instrument_buckets.csv"]}
    dimensions: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in tables["instrument_bucket_dimensions.csv"]:
        dimensions[row["bucket_id"]].append(row)
        if row["bucket_id"] not in buckets:
            issue(errors, "key", row["bucket_id"], "instrument bucket dimension is orphaned")
        if row["bucket_value"] == UNKNOWN:
            issue(errors, "instrument", row["bucket_id"], "structural bucket value cannot be UNKNOWN")
    bucket_families = set()
    for bucket_id, row in buckets.items():
        if row["instrument_code"] not in instruments:
            issue(errors, "key", bucket_id, f"instrument bucket references unknown family {row['instrument_code']}")
        else:
            bucket_families.add(row["instrument_code"])
        if not dimensions[bucket_id]:
            issue(errors, "instrument", bucket_id, "instrument bucket has no dimensions")
    expected_active = {code for code, (cut, _) in EXPECTED_INSTRUMENTS.items() if cut in {"Core", "Circuit"}}
    if bucket_families != expected_active:
        issue(errors, "instrument", "instrument_buckets.csv",
              f"active buckets must cover exactly Core + Circuit families; missing={sorted(expected_active - bucket_families)}; extra={sorted(bucket_families - expected_active)}")
    for row in tables["composition_probe_instrument_buckets.csv"]:
        if row["bucket_id"] not in buckets or row["composition_probe_id"] not in EXPECTED_PROBES:
            issue(errors, "key", row["bucket_id"], "composition-probe bucket reference is unresolved")

    # Composition contract and runtime readiness are separate verdicts.
    composition_probes = {row["composition_probe_id"] for row in tables["composition_probes.csv"]}
    if composition_probes != EXPECTED_PROBES:
        issue(errors, "composition_probe", "composition_probes.csv", "the catalog must contain exactly the three declared composition probes")
    coverage = {(row["probe_id"], row["catalog_id"]): row["coverage_status"] for row in tables["probe_coverage.csv"]}
    members = {(row["composition_probe_id"], row["catalog_id"]) for row in tables["composition_probe_members.csv"]}
    runtime_seen: dict[str, set[str]] = defaultdict(set)
    for row in tables["composition_probe_members.csv"]:
        if row["catalog_id"] not in entities:
            issue(errors, "composition_probe", row["catalog_id"], "composition member is not a catalog entry")
        provider = row["candidate_provider_entry_id"]
        if provider != NONE and provider not in instances:
            issue(errors, "composition_probe", row["catalog_id"], f"unresolved composition provider {provider}")
    for row in tables["composition_probe_requirements.csv"]:
        key = (row["composition_probe_id"], row["catalog_id"])
        if key not in members:
            issue(errors, "composition_probe", row["catalog_id"], "requirement entry is not a member of its composition probe")
        if row["gate"] == "catalog_contract":
            actual = coverage.get((row["architecture_probe_id"], row["catalog_id"]))
            if row["required_status"] != "covered" or actual != "covered":
                issue(errors, "composition_probe", row["architecture_probe_id"], "required architecture probe is uncovered")
        else:
            if row["required_status"] != "blocked" or not row["architecture_probe_id"].startswith("init."):
                issue(errors, "composition_probe", row["architecture_probe_id"], "runtime requirement must carry a blocked init.<code> status")
            else:
                runtime_seen[row["composition_probe_id"]].add(row["architecture_probe_id"].removeprefix("init."))
    for probe_id, expected in EXPECTED_RUNTIME_BLOCKERS.items():
        if runtime_seen[probe_id] != expected:
            issue(errors, "composition_probe", probe_id,
                  f"runtime blockers differ; expected={sorted(expected)}; actual={sorted(runtime_seen[probe_id])}")

    # Product topology for the worked regional-process probe only.
    product_roles: dict[str, set[str]] = defaultdict(set)
    for row in tables["product_channel_roles.csv"]:
        if row["product_code"] not in products:
            issue(errors, "product", row["product_code"], "channel role references an unknown product")
        if row["participant_entry_id"] not in instances or row["scope_entry_id"] not in instances:
            issue(errors, "endpoint", row["product_code"], "product channel role has an unresolved participant or scope")
        product_roles[row["product_code"]].add(row["flow_role"])
    for product in PROBE_PRODUCTS:
        roles_for_product = product_roles[product]
        if "source" not in roles_for_product or not roles_for_product & {"destination", "market"}:
            issue(errors, "product", product, "probe-referenced product requires source and destination/market coverage")

    # Structural residuals are complete without claiming opening values.
    for row in tables["residual_reconciliation.csv"]:
        if row["catalog_id"] not in instances or row["residual_counterpart_id"] not in instances:
            issue(errors, "residual", row["catalog_id"], "residual reconciliation endpoint is unresolved")
        if row["status"] != "complete":
            issue(errors, "residual", row["catalog_id"], "executable residual row must be structurally complete")

    # A true eligibility declaration must pass the catalog contract; profile roles never affect it.
    details = eligibility_details(tables)
    for catalog_id, row in instances.items():
        eligible, _, blockers = details[catalog_id]
        if row["selectable_in_manifest"] == "true" and not eligible:
            issue(errors, "catalog_eligibility", catalog_id,
                  f"selectable entry has incomplete catalog contract: {'|'.join(blockers)}")
    return tables, errors


def emit_issues(errors: list[dict[str, str]]) -> None:
    unique = sorted({(row["severity"], row["category"], row["record_id"], row["issue"]) for row in errors})
    rows = [dict(zip(("severity", "category", "record_id", "issue"), values)) for values in unique]
    write_table(ROOT / "generated" / "gaps.csv", ["severity", "category", "record_id", "issue"], rows)
    for row in rows:
        print(f"ERROR [{row['category']}] {row['record_id']}: {row['issue']}", file=sys.stderr)


def validate(_: argparse.Namespace) -> int:
    tables, errors = collect_errors()
    emit_issues(errors)
    if errors:
        print(f"validation failed: {len(errors)} structural issue(s)", file=sys.stderr)
        return 1
    print(f"validation passed: {len(tables['entities.csv'])} entities; zero structural gaps; zero warnings")
    return 0


def audit_eligibility(_: argparse.Namespace) -> int:
    tables, errors = collect_errors()
    write_catalog_eligibility(tables)
    eligible = sum(value[0] for value in eligibility_details(tables).values())
    total = len(eligibility_details(tables))
    print(f"catalog eligibility audit: {eligible}/{total} instances eligible; authored data unchanged")
    if errors:
        emit_issues(errors)
        return 1
    return 0


def write_catalog_eligibility(tables: dict[str, list[dict[str, str]]]) -> None:
    entities = {row["catalog_id"]: row for row in tables["entities.csv"]}
    details = eligibility_details(tables)
    rows = []
    for catalog_id, (eligible, checks, blockers) in details.items():
        row = entities[catalog_id]
        rows.append({
            "catalog_id": catalog_id,
            "entry_class": row["entry_class"],
            "instance_of": row["instance_of"],
            "identity_clade": row["identity_clade"],
            "selectable_in_manifest": row["selectable_in_manifest"],
            "catalog_eligibility": str(eligible).lower(),
            "contract_checks": "|".join(checks) if checks else NONE,
            "blocker_codes": "|".join(blockers) if blockers else NONE,
        })
    write_table(ROOT / "generated" / "catalog_eligibility.csv",
                ["catalog_id", "entry_class", "instance_of", "identity_clade", "selectable_in_manifest",
                 "catalog_eligibility", "contract_checks", "blocker_codes"],
                sorted(rows, key=lambda row: row["catalog_id"]))


def composition_statuses(tables: dict[str, list[dict[str, str]]]) -> dict[str, tuple[str, str, list[str]]]:
    coverage = {(row["probe_id"], row["catalog_id"]): row["coverage_status"] for row in tables["probe_coverage.csv"]}
    grouped: dict[str, list[dict[str, str]]] = defaultdict(list)
    for row in tables["composition_probe_requirements.csv"]:
        grouped[row["composition_probe_id"]].append(row)
    result = {}
    for probe_id, rows in grouped.items():
        contract = all(
            row["required_status"] == "covered" and coverage.get((row["architecture_probe_id"], row["catalog_id"])) == "covered"
            for row in rows if row["gate"] == "catalog_contract"
        )
        blockers = sorted(row["architecture_probe_id"].removeprefix("init.") for row in rows if row["gate"] == "runtime_initialization" and row["required_status"] == "blocked")
        result[probe_id] = ("complete" if contract else "incomplete", "blocked" if blockers else "ready", blockers)
    return result


def generate(_: argparse.Namespace) -> int:
    tables, errors = collect_errors()
    generated = ROOT / "generated"
    if generated.exists():
        shutil.rmtree(generated)
    generated.mkdir(parents=True)
    emit_issues(errors)
    write_catalog_eligibility(tables)
    details = eligibility_details(tables)
    entities = {row["catalog_id"]: row for row in tables["entities.csv"]}

    profile_rows = []
    for row in tables["profile_catalog_roles.csv"]:
        eligible = details[row["catalog_id"]][0]
        runtime_blockers = []
        if row["profile_role"] in {"slice_candidate", "boundary_candidate", "reserve"}:
            runtime_blockers.append("manifest_selection")
        if row["profile_role"] == "reserve":
            runtime_blockers.append("promotion_requirements")
        profile_rows.append({
            **row,
            "catalog_eligibility": str(eligible).lower(),
            "runtime_blocker_codes": "|".join(runtime_blockers) if runtime_blockers else NONE,
        })
    profile_fields = SCHEMA["tables"]["profile_catalog_roles.csv"] + ["catalog_eligibility", "runtime_blocker_codes"]
    write_table(generated / "profile_planning.csv", profile_fields,
                sorted(profile_rows, key=lambda row: (row["profile_id"], row["catalog_id"])))

    scope_by_provider: dict[tuple[str, str], list[str]] = defaultdict(list)
    for row in tables["provider_scopes.csv"]:
        scope_by_provider[(row["profile_id"], row["provider_entry_id"])].append(row["scope_entry_id"])
    provider_role = {(row["profile_id"], row["channel_code"], row["provider_entry_id"]): row["coverage_role"] for row in tables["external_channel_providers.csv"]}
    fallback = {row["catalog_id"]: row["fallback_entry_id"] for row in tables["entities.csv"]}
    external_rows = []
    for row in tables["external_market_channels.csv"]:
        scopes = sorted(scope_by_provider[(row["profile_id"], row["provider_entry_id"])])
        external_rows.append({
            "profile_id": row["profile_id"], "channel_code": row["channel_code"],
            "provider_entry_id": row["provider_entry_id"], "interface_code": row["interface_code"],
            "family_code": row["family_code"], "coverage_role": row["coverage_role"],
            "membership_scope": "|".join(scopes),
            "provider_contract_status": "complete" if details[row["provider_entry_id"]][0] else "incomplete",
            "residual_termination": "terminal_exogenous_input" if row["coverage_role"] == "residual" else fallback[row["provider_entry_id"]],
        })
    external_fields = ["profile_id", "channel_code", "provider_entry_id", "interface_code", "family_code",
                       "coverage_role", "membership_scope", "provider_contract_status", "residual_termination"]
    write_table(generated / "external_channel_coverage.csv", external_fields,
                sorted(external_rows, key=lambda row: tuple(row[field] for field in external_fields[:5])))

    scope_market = []
    for row in tables["external_market_channels.csv"]:
        for scope in scope_by_provider[(row["profile_id"], row["provider_entry_id"])]:
            scope_market.append({"profile_id": row["profile_id"], "scope_entry_id": scope,
                                 "channel_code": row["channel_code"], "interface_code": row["interface_code"],
                                 "family_code": row["family_code"], "flow_role": "source",
                                 "participant_entry_id": row["provider_entry_id"]})
    for row in tables["product_channel_roles.csv"]:
        scope_market.append({"profile_id": row["profile_id"], "scope_entry_id": row["scope_entry_id"],
                             "channel_code": row["channel_code"], "interface_code": "interface.import_supply.product_schedule",
                             "family_code": row["product_code"], "flow_role": row["flow_role"],
                             "participant_entry_id": row["participant_entry_id"]})
    scope_fields = ["profile_id", "scope_entry_id", "channel_code", "interface_code", "family_code", "flow_role", "participant_entry_id"]
    write_table(generated / "scope_market_coverage.csv", scope_fields,
                sorted(scope_market, key=lambda row: tuple(row[field] for field in scope_fields)))

    statuses = composition_statuses(tables)
    family_by_probe: dict[str, list[str]] = defaultdict(list)
    bucket_map = {row["bucket_id"]: row["instrument_code"] for row in tables["instrument_buckets.csv"]}
    for row in tables["composition_probe_instrument_buckets.csv"]:
        family_by_probe[row["composition_probe_id"]].append(bucket_map[row["bucket_id"]])
    composition_rows = []
    for row in tables["composition_probe_requirements.csv"]:
        contract_status, runtime_status, blockers = statuses[row["composition_probe_id"]]
        composition_rows.append({
            "composition_probe_id": row["composition_probe_id"], "member": row["catalog_id"],
            "architecture_probe": row["architecture_probe_id"],
            "family": "|".join(sorted(set(family_by_probe[row["composition_probe_id"]]))) or NONE,
            "graph": "typed_member_and_requirement",
            "contract_status": contract_status, "runtime_status": runtime_status,
            "blocker_codes": "|".join(blockers) if blockers else NONE,
        })
    comp_fields = ["composition_probe_id", "member", "architecture_probe", "family", "graph", "contract_status", "runtime_status", "blocker_codes"]
    write_table(generated / "composition_probe_coverage.csv", comp_fields,
                sorted(composition_rows, key=lambda row: (row["composition_probe_id"], row["member"], row["architecture_probe"])))

    deferred = []
    for row in tables["research_backlog.csv"]:
        deferred.append({"item_kind": row["artifact_kind"], "catalog_id": row["catalog_id"],
                         "artifact_key": row["artifact_key"], "blocker_code": row["blocker_code"],
                         "required_evidence": row["required_evidence"], "provenance": row["provenance"]})
    role_by_id = {row["catalog_id"]: row for row in tables["profile_catalog_roles.csv"]}
    for catalog_id, entity in entities.items():
        if entity["entry_class"] != "instance":
            continue
        role = role_by_id[catalog_id]
        if role["profile_role"] == "reserve" or entity["completeness_state"] == "identity_only":
            deferred.append({"item_kind": "profile_reserve" if role["profile_role"] == "reserve" else "identity_only",
                             "catalog_id": catalog_id, "artifact_key": role["candidate_provider_entry_id"],
                             "blocker_code": "promotion_requirements" if role["profile_role"] == "reserve" else "research_content",
                             "required_evidence": role["activation_requirement"] if role["activation_requirement"] != NONE else "Typed structural and initialization contract.",
                             "provenance": role["provenance"]})
    covered_products = PROBE_PRODUCTS
    for product in tables["product_families.csv"]:
        if product["product_code"] not in covered_products:
            deferred.append({"item_kind": "unreferenced_product", "catalog_id": NONE,
                             "artifact_key": product["product_code"], "blocker_code": "profile_relevance",
                             "required_evidence": "A composition probe, slice dependency, household salience, bottleneck, financial market, or scenario transmission.",
                             "provenance": product["provenance"]})
    for row in tables["scenario_candidates.csv"]:
        deferred.append({"item_kind": "scenario_candidate", "catalog_id": row["catalog_id"],
                         "artifact_key": row["scenario_id"], "blocker_code": "scenario_manifest",
                         "required_evidence": row["relevance"], "provenance": row["provenance"]})
    deferred_fields = ["item_kind", "catalog_id", "artifact_key", "blocker_code", "required_evidence", "provenance"]
    write_table(generated / "deferred_backlog.csv", deferred_fields,
                sorted(deferred, key=lambda row: tuple(row[field] for field in deferred_fields)))

    # Identity views remain useful but do not imply runtime readiness.
    for field in ("identity_clade", "domain", "research_status"):
        groups: dict[str, list[dict[str, str]]] = defaultdict(list)
        for row in tables["entities.csv"]:
            groups[row[field]].append(row)
        for key, rows in groups.items():
            safe = re.sub(r"[^a-z0-9_]+", "_", key.lower())
            write_table(generated / field / f"{safe}.csv", SCHEMA["tables"]["entities.csv"],
                        sorted(rows, key=lambda row: row["catalog_id"]))
    write_completeness(tables)
    print(f"generated catalog evidence: {sum(1 for _ in generated.rglob('*.csv'))} CSV files")
    return 1 if errors else 0


def canonical_prose_ids(design_root: Path, prefixes: tuple[str, ...]) -> set[str]:
    prose_ids: set[str] = set()
    for path in sorted(design_root.rglob("*.md")):
        if path.name == "AGENTS.md":
            continue
        source = path.read_text()
        if "**Status:** `canonical`" not in source.splitlines()[1:7]:
            continue
        for token in source.split("`")[1::2]:
            value = token.strip()
            if (value.startswith(prefixes) and "<" not in value and " " not in value
                    and ":" not in value and "*" not in value and value.count(".") >= 2):
                prose_ids.add(value)
    return prose_ids


def compare(_: argparse.Namespace) -> int:
    design_root = REPO_ROOT / "docs" / "design"
    prefixes = tuple({value.split(".", 1)[0] + "." for value in SCHEMA["stable_ids"].values() if value.startswith(("type.", "profile.", "channel.", "interface.", "backlog."))})
    prefixes += ("region.", "generator.", "process.", "industry.", "adapter.", "mechanism.", "market.", "inst.",
                 "federation.", "facility.", "reference.", "lens.", "office.", "body.", "staff.", "sovereign.",
                 "outlet.", "network.", "schedule.", "record.", "law.", "cohort.", "firm.", "coalition.",
                 "agreement.", "person.")
    prose_ids = canonical_prose_ids(design_root, prefixes)
    data_ids = ({row["catalog_id"] for row in read_table("entities.csv")} |
                {row["type_id"] for row in read_table("types.csv")} |
                {row["profile_id"] for row in read_table("world_profiles.csv")} |
                {row["channel_code"] for row in read_table("external_channels.csv")} |
                {row["interface_code"] for row in read_table("market_interfaces.csv")})
    rows = [{"catalog_id": value, "in_prose": str(value in prose_ids).lower(), "in_data": str(value in data_ids).lower(),
             "status": "matched" if value in prose_ids and value in data_ids else "missing_in_data" if value in prose_ids else "data_only"}
            for value in sorted(prose_ids | data_ids)]
    write_table(ROOT / "generated" / "source_comparison.csv", ["catalog_id", "in_prose", "in_data", "status"], rows)
    missing = [row for row in rows if row["status"] == "missing_in_data"]
    print(f"source comparison: {len(prose_ids)} prose IDs, {len(data_ids)} data IDs, {len(missing)} missing")
    return 1 if missing else 0


def write_completeness(tables: dict[str, list[dict[str, str]]]) -> None:
    instances = [row for row in tables["entities.csv"] if row["entry_class"] == "instance"]
    typed = [row for row in instances if row["instance_of"] not in {UNKNOWN, NONE} and row["research_status"] not in {UNKNOWN, NONE}]
    details = eligibility_details(tables)
    eligible = sum(value[0] for value in details.values())
    roles = tables["profile_catalog_roles.csv"]
    slice_roles = [row for row in roles if row["profile_role"] == "slice_candidate"]
    slice_eligible = sum(details[row["catalog_id"]][0] for row in slice_roles)
    statuses = composition_statuses(tables)
    contract_complete = sum(status[0] == "complete" for status in statuses.values())
    runtime_blocked = sum(status[1] == "blocked" for status in statuses.values())
    core_circuit = {row["instrument_code"] for row in tables["instrument_families.csv"] if row["dependency_cut"] in {"Core", "Circuit"}}
    bucket_families = {row["instrument_code"] for row in tables["instrument_buckets.csv"]}
    product_roles: dict[str, set[str]] = defaultdict(set)
    manifests = []
    for path in sorted((REPO_ROOT / "scenarios").glob("*/manifest.json")):
        document = json.loads(path.read_text())
        manifests.append(
            f"  scenario representation manifest: {document['manifest_id']} ({document['replay_hash']})"
        )
    if not manifests:
        manifests = ["  no scenario representation manifest is authored"]
    for row in tables["product_channel_roles.csv"]:
        product_roles[row["product_code"]].add(row["flow_role"])
    product_closed = sum("source" in product_roles[p] and bool(product_roles[p] & {"destination", "market"}) for p in PROBE_PRODUCTS)
    lines = [
        "catalog completeness",
        "identity inventory:",
        f"  typed-subject coverage: {len(typed)}/{len(instances)} ({100 * len(typed) / len(instances):.1f}%)",
        "  identity inventory is not runtime readiness",
        "catalog contract:",
        f"  catalog-eligible instances: {eligible}/{len(instances)}",
        f"  early-2006 profile assignment: {len(roles)}/{len(instances)}",
        f"  slice-candidate catalog closure: {slice_eligible}/{len(slice_roles)}",
        f"  external channels with one residual: {len(EXPECTED_CHANNELS)}/{len(EXPECTED_CHANNELS)}",
        f"  external provider/interface rows: {len(tables['external_market_channels.csv'])}",
        f"  probe-referenced product source + destination/market coverage: {product_closed}/{len(PROBE_PRODUCTS)}",
        f"  instrument-family closure: {len(tables['instrument_families.csv'])}/15",
        f"  active Core + Circuit bucket-family closure: {len(bucket_families & core_circuit)}/{len(core_circuit)}",
        f"  composition-probe contract closure: {contract_complete}/{len(EXPECTED_PROBES)}",
        f"  composition-probe runtime blocked: {runtime_blocked}/{len(EXPECTED_PROBES)}",
        "runtime boundary:",
        *manifests,
        "  required manifest contents: identity clades; owner classes; fidelity tiers; residual mappings; protected population dimensions; permitted cell transitions; boundary-interface versions; adapter status; initialization reconciliation; fallbacks; replay hash",
        "  no geographic-completeness percentage is defined",
    ]
    (ROOT / "generated" / "completeness.txt").write_text("\n".join(lines) + "\n")


def report(_: argparse.Namespace) -> int:
    tables, errors = collect_errors()
    write_completeness(tables)
    output = (ROOT / "generated" / "completeness.txt").read_text()
    print(output, end="")
    if errors:
        emit_issues(errors)
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate and derive Representation Catalog data")
    sub = parser.add_subparsers(dest="command", required=True)
    commands = (
        ("init", init_tables), ("import", import_inventory), ("validate", validate),
        ("audit-eligibility", audit_eligibility), ("compare", compare),
        ("generate", generate), ("report", report),
    )
    for name, func in commands:
        command = sub.add_parser(name)
        command.set_defaults(func=func)
        if name == "validate":
            command.add_argument("--strict-warnings", action="store_true",
                                 help="accepted for workflow compatibility; runtime blockers are backlog rows, not warnings")
    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
