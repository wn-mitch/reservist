use std::fs;
use std::path::PathBuf;

use reservist_core::canon::{canonical_bytes, canonical_text, sha256};
use serde_json::{Value, json};

#[test]
fn canonicalizes_compact_utf8_and_utf16_key_order() {
    let value = json!({"z": 1, "ä": "goose", "a": [true, null, "line\n"]});
    assert_eq!(
        canonical_text(&value).unwrap(),
        r#"{"a":[true,null,"line\n"],"z":1,"ä":"goose"}"#,
    );

    // UTF-16 code-unit order deliberately differs from UTF-8 byte order for
    // supplementary-plane keys versus BMP private-use keys.
    let value = json!({"\u{e000}": "bmp", "\u{10000}": "supplementary"});
    assert_eq!(
        canonical_text(&value).unwrap(),
        r#"{"𐀀":"supplementary","":"bmp"}"#,
    );
}

#[test]
fn uses_ecmascript_shortest_ieee754_number_spelling() {
    let value: Value = serde_json::from_str(
        "[333333333.33333329,1e30,4.5,2e-3,1e-27,-0.0,0.000001,1e-7,1e20,1e21]",
    )
    .unwrap();
    assert_eq!(
        canonical_text(&value).unwrap(),
        "[333333333.3333333,1e+30,4.5,0.002,1e-27,0,0.000001,1e-7,100000000000000000000,1e+21]",
    );
}

#[test]
fn hashes_are_mapping_order_independent() {
    assert_eq!(
        sha256(&json!({"a": 1, "b": 2})),
        sha256(&json!({"b": 2, "a": 1}))
    );
}

#[test]
fn rejects_non_finite_json_input() {
    let path = std::env::temp_dir().join(format!(
        "reservist-canon-nonfinite-{}.json",
        std::process::id()
    ));
    fs::write(&path, "NaN").unwrap();
    assert!(reservist_core::canon::load_json(&path).is_err());
    fs::remove_file(path).unwrap();
}

#[test]
fn oracle_transcript_lines_are_already_canonical() {
    let vector_dir = workspace_root().join("tests/oracle/vectors");
    let entries = fs::read_dir(&vector_dir).unwrap_or_else(|error| {
        panic!(
            "oracle vectors are required; run python3 tools/oracle/export_vectors.py first: {error}"
        )
    });
    let mut vector_names = std::collections::BTreeSet::new();

    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        vector_names.insert(path.file_name().unwrap().to_str().unwrap().to_owned());
        let vector: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        if vector["vector_kind"] == "play_script" {
            continue;
        }
        let transcript = vector["transcript"]
            .as_array()
            .unwrap_or_else(|| panic!("{} has no transcript array", path.display()));
        let lines = vector["transcript_canonical_lines"]
            .as_array()
            .unwrap_or_else(|| panic!("{} has no canonical transcript lines", path.display()));
        assert_eq!(transcript.len(), lines.len(), "{}", path.display());

        for (event, expected) in transcript.iter().zip(lines) {
            let expected = expected.as_str().unwrap();
            assert_eq!(
                canonical_bytes(event).unwrap(),
                expected.as_bytes(),
                "{}",
                path.display()
            );
        }
    }

    let mut expected = std::collections::BTreeSet::new();
    for package in ["WAIT_AND_WARN", "MEASURED_FIRMING", "FIRMING_BIAS"] {
        for request in ["none", "NORMAL", "ACCELERATED", "DECLINED", "MISSED"] {
            expected.insert(format!("{package}__{request}.json"));
        }
    }
    for seed in [20060328, 20060329, 20060330, 20060331] {
        expected.insert(format!("reseed__{seed}.json"));
    }
    expected.insert("play__scripted.json".into());
    assert_eq!(
        vector_names, expected,
        "every pinned oracle path must be covered"
    );
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .to_path_buf()
}
