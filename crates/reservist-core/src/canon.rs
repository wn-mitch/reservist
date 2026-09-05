use std::path::Path;

use ryu_js::Buffer;
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonError {
    #[error("RFC 8785 forbids NaN and infinity")]
    NonFiniteNumber,
    #[error("lone Unicode surrogates are not valid JSON")]
    LoneSurrogate,
    #[error("failed to read JSON from {path}: {source}")]
    ReadJson {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse JSON from {path}: {source}")]
    ParseJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

/// Serializes a valid JSON value according to RFC 8785.
///
/// `serde_json::Value` cannot represent a lone surrogate, but the explicit
/// validation keeps this boundary correct should its string representation
/// change in a future serde_json release.
pub fn canonical_text(value: &Value) -> Result<String, CanonError> {
    let mut output = String::new();
    write_value(value, &mut output)?;
    Ok(output)
}

/// Returns UTF-8 RFC 8785 canonical JSON bytes.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, CanonError> {
    Ok(canonical_text(value)?.into_bytes())
}

/// Returns the SHA-256 digest of RFC 8785 canonical JSON, with the project's
/// `sha256:` identity prefix.
pub fn sha256(value: &Value) -> String {
    let bytes =
        canonical_bytes(value).expect("serde_json::Value contains only finite, valid JSON values");
    let digest = Sha256::digest(bytes);
    use std::fmt::Write;
    let mut identity = String::with_capacity(71);
    identity.push_str("sha256:");
    for byte in digest {
        write!(identity, "{byte:02x}").expect("writing to String cannot fail");
    }
    identity
}

/// Loads JSON while preserving the canonicalizer's failure boundary.
///
/// JSON spelling such as `NaN` and `Infinity` is rejected by serde_json before
/// a `Value` is produced.
pub fn load_json(path: impl AsRef<Path>) -> Result<Value, CanonError> {
    let path = path.as_ref();
    let path_text = path.display().to_string();
    let input = std::fs::read_to_string(path).map_err(|source| CanonError::ReadJson {
        path: path_text.clone(),
        source,
    })?;
    serde_json::from_str(&input).map_err(|source| CanonError::ParseJson {
        path: path_text,
        source,
    })
}

fn write_value(value: &Value, output: &mut String) -> Result<(), CanonError> {
    match value {
        Value::Null => output.push_str("null"),
        Value::Bool(true) => output.push_str("true"),
        Value::Bool(false) => output.push_str("false"),
        Value::Number(number) => write_number(number, output)?,
        Value::String(text) => write_string(text, output)?,
        Value::Array(items) => {
            output.push('[');
            for (index, item) in items.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                write_value(item, output)?;
            }
            output.push(']');
        }
        Value::Object(members) => {
            let mut members: Vec<_> = members.iter().collect();
            members.sort_unstable_by(|(left, _), (right, _)| utf16_code_unit_cmp(left, right));

            output.push('{');
            for (index, (key, member)) in members.into_iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                write_string(key, output)?;
                output.push(':');
                write_value(member, output)?;
            }
            output.push('}');
        }
    }
    Ok(())
}

fn write_number(number: &serde_json::Number, output: &mut String) -> Result<(), CanonError> {
    if let Some(integer) = number.as_i64() {
        output.push_str(&integer.to_string());
        return Ok(());
    }
    if let Some(integer) = number.as_u64() {
        output.push_str(&integer.to_string());
        return Ok(());
    }

    let float = number.as_f64().ok_or(CanonError::NonFiniteNumber)?;
    if !float.is_finite() {
        return Err(CanonError::NonFiniteNumber);
    }
    if float == 0.0 {
        output.push('0');
        return Ok(());
    }

    // ryu-js implements ECMAScript's Number::toString thresholds and shortest
    // IEEE-754 round-trip spelling, which is the number rule RFC 8785 adopts.
    let mut buffer = Buffer::new();
    output.push_str(buffer.format_finite(float));
    Ok(())
}

fn write_string(value: &str, output: &mut String) -> Result<(), CanonError> {
    if value
        .chars()
        .any(|character| (0xD800..=0xDFFF).contains(&(character as u32)))
    {
        return Err(CanonError::LoneSurrogate);
    }

    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{08}' => output.push_str("\\b"),
            '\u{09}' => output.push_str("\\t"),
            '\u{0A}' => output.push_str("\\n"),
            '\u{0C}' => output.push_str("\\f"),
            '\u{0D}' => output.push_str("\\r"),
            character if (character as u32) <= 0x1F => {
                use std::fmt::Write;
                write!(output, "\\u{:04x}", character as u32)
                    .expect("writing to String cannot fail");
            }
            character => output.push(character),
        }
    }
    output.push('"');
    Ok(())
}

fn utf16_code_unit_cmp(left: &str, right: &str) -> std::cmp::Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}
