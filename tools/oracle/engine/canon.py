from __future__ import annotations

import hashlib
import json
import math
from collections.abc import Mapping, Sequence
from decimal import Decimal
from typing import Any


class CanonicalizationError(ValueError):
    pass


def _string(value: str) -> str:
    try:
        value.encode("utf-8")
    except UnicodeEncodeError as exc:
        raise CanonicalizationError("lone Unicode surrogates are not valid JSON") from exc
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def _number(value: int | float) -> str:
    if isinstance(value, bool):
        raise TypeError("booleans are not numbers in canonical dispatch")
    if isinstance(value, int):
        return str(value)
    if not math.isfinite(value):
        raise CanonicalizationError("RFC 8785 forbids NaN and infinity")
    if value == 0:
        return "0"

    negative = value < 0
    magnitude = abs(value)
    shortest = repr(magnitude)
    decimal = Decimal(shortest)
    if 1e-6 <= magnitude < 1e21:
        rendered = format(decimal, "f")
        if "." in rendered:
            rendered = rendered.rstrip("0").rstrip(".")
    else:
        if "e" in shortest.lower():
            mantissa, exponent = shortest.lower().split("e")
        else:
            normalized = format(decimal.normalize(), "e")
            mantissa, exponent = normalized.split("e")
        mantissa = mantissa.rstrip("0").rstrip(".")
        exponent_value = int(exponent)
        rendered = f"{mantissa}e{'+' if exponent_value >= 0 else ''}{exponent_value}"
    return f"-{rendered}" if negative else rendered


def _key(value: str) -> bytes:
    return value.encode("utf-16-be", errors="surrogatepass")


def canonical_text(value: Any) -> str:
    if value is None:
        return "null"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, (int, float)):
        return _number(value)
    if isinstance(value, str):
        return _string(value)
    if isinstance(value, Mapping):
        if not all(isinstance(key, str) for key in value):
            raise CanonicalizationError("JSON object keys must be strings")
        members = (
            f"{_string(key)}:{canonical_text(value[key])}"
            for key in sorted(value, key=_key)
        )
        return "{" + ",".join(members) + "}"
    if isinstance(value, Sequence) and not isinstance(value, (bytes, bytearray)):
        return "[" + ",".join(canonical_text(item) for item in value) + "]"
    raise CanonicalizationError(f"unsupported JSON value: {type(value).__name__}")


def canonical_bytes(value: Any) -> bytes:
    return canonical_text(value).encode("utf-8")


def sha256(value: Any) -> str:
    return "sha256:" + hashlib.sha256(canonical_bytes(value)).hexdigest()


def load_json(path: str | bytes | Any) -> Any:
    with open(path, encoding="utf-8") as handle:
        return json.load(handle, parse_constant=_reject_constant)


def _reject_constant(value: str) -> None:
    raise CanonicalizationError(f"non-finite JSON number: {value}")


def write_canonical_json(path: Any, value: Any) -> None:
    path.write_bytes(canonical_bytes(value) + b"\n")
