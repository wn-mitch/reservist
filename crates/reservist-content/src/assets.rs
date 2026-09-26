//! Approved presentation assets and their metadata contract.
//!
//! Every image under `assets/` has one `presentation_assets.csv` row with a
//! stable asset ID, one of the four image roles, an approval state, an
//! approved source reference, rights, pixel dimensions, a crop, and a focal
//! point. Asset rows carry no catalog identity: `presentation_refs.csv` binds a
//! catalog entry to an approved asset of the matching role, so replacing an
//! image never touches simulation identity. Each image has a pinned Godot import
//! preset, and generated import caches stay out of the asset tree.

use crate::catalog::issue;
use crate::{Issue, Row, Tables};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub(crate) const ASSETS: &str = "presentation_assets.csv";
const REFS: &str = "presentation_refs.csv";

/// Import parameters each preset pins in the sibling `.import` file.
const PRESETS: &[(&str, &[(&str, &str)])] = &[(
    "pixel_art_lossless",
    &[
        ("importer", "\"texture\""),
        ("compress/mode", "0"),
        ("mipmaps/generate", "false"),
        ("process/fix_alpha_border", "true"),
        ("process/size_limit", "0"),
    ],
)];

/// The asset folder each role lives under.
fn role_folder(role: &str) -> Option<&'static str> {
    match role {
        "portrait" | "actor_cutout" => Some("assets/characters/"),
        "background_plate" => Some("assets/plates/"),
        "event_illustration" => Some("assets/events/"),
        _ => None,
    }
}

/// Width, height, and PNG color type read from the IHDR header.
pub(crate) fn png_header(bytes: &[u8]) -> Option<(u32, u32, u8)> {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    if bytes.len() < 26 || &bytes[..8] != SIGNATURE || &bytes[12..16] != b"IHDR" {
        return None;
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some((width, height, bytes[25]))
}

fn numbers(value: &str, count: usize) -> Option<Vec<u32>> {
    let parts: Vec<u32> = value
        .split(';')
        .map(|part| part.trim().parse().ok())
        .collect::<Option<_>>()?;
    (parts.len() == count).then_some(parts)
}

fn import_params(text: &str) -> BTreeMap<&str, &str> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim(), value.trim()))
        .collect()
}

fn walk(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            found.push(path.clone());
            walk(&path, found);
        } else {
            found.push(path);
        }
    }
}

fn check_asset(row: &Row, repository: &Path, errors: &mut Vec<Issue>) {
    let id = row["asset_id"].as_str();
    let mut fail = |message: String| issue(errors, "presentation_asset", id, message);
    if !id.starts_with("asset.") {
        fail("asset_id must start with asset.".into());
    }
    let role = row["asset_role"].as_str();
    let path = row["asset_path"].as_str();
    match role_folder(role) {
        Some(folder) if path.starts_with(folder) => {}
        Some(folder) => fail(format!("{role} assets live under {folder}")),
        None => fail(format!("unknown asset role {role}")),
    }
    let source = row["source_ref"].as_str();
    if !source.starts_with("https://") && !repository.join(source).is_file() {
        fail(format!(
            "source_ref {source} is neither a URL nor a repository file"
        ));
    }
    let file = repository.join(path);
    let Some((width, height, color)) = std::fs::read(&file)
        .ok()
        .and_then(|bytes| png_header(&bytes))
    else {
        fail(format!("{path} is not a readable PNG"));
        return;
    };
    let declared = (
        row["width_px"].parse::<u32>(),
        row["height_px"].parse::<u32>(),
    );
    if declared != (Ok(width), Ok(height)) {
        fail(format!(
            "declared size does not match the {width}x{height} image"
        ));
    }
    match numbers(&row["crop"], 4) {
        Some(crop)
            if crop[2] > 0
                && crop[3] > 0
                && crop[0] + crop[2] <= width
                && crop[1] + crop[3] <= height =>
        {
            match numbers(&row["focal_point"], 2) {
                Some(focal)
                    if (crop[0]..crop[0] + crop[2]).contains(&focal[0])
                        && (crop[1]..crop[1] + crop[3]).contains(&focal[1]) => {}
                _ => fail("focal_point must be x;y inside the crop".into()),
            }
        }
        _ => fail("crop must be x;y;width;height inside the image".into()),
    }
    // An actor cutout composes over plates, so it must carry transparency.
    if role == "actor_cutout" && !matches!(color, 4 | 6) {
        fail("actor cutouts need an alpha channel".into());
    }
    let preset = row["import_preset"].as_str();
    let Some((_, pinned)) = PRESETS.iter().find(|(name, _)| *name == preset) else {
        fail(format!("unknown import preset {preset}"));
        return;
    };
    let import = std::fs::read_to_string(repository.join(format!("{path}.import")));
    match import {
        Err(_) => fail(format!("{path} has no pinned .import file")),
        Ok(text) => {
            let params = import_params(&text);
            for (key, value) in *pinned {
                if params.get(key) != Some(value) {
                    fail(format!("{path}.import must pin {key}={value} for {preset}"));
                }
            }
        }
    }
}

/// Validates asset rows against files under `repository`.
pub(crate) fn validate_assets(tables: &Tables, repository: &Path, errors: &mut Vec<Issue>) {
    let rows: Vec<&Row> = tables.get(ASSETS).into_iter().flatten().collect();
    let mut by_path = BTreeMap::<&str, &Row>::new();
    for row in &rows {
        check_asset(row, repository, errors);
        if by_path.insert(row["asset_path"].as_str(), row).is_some() {
            issue(
                errors,
                "presentation_asset",
                &row["asset_path"],
                "path has two asset rows",
            );
        }
    }
    let mut found = Vec::new();
    walk(&repository.join("assets"), &mut found);
    for path in found {
        let relative = path
            .strip_prefix(repository)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if name == ".godot" || name.ends_with(".ctex") {
            issue(
                errors,
                "presentation_asset",
                &relative,
                "generated import caches stay out of assets/",
            );
        } else if name.ends_with(".png") && !by_path.contains_key(relative.as_str()) {
            issue(
                errors,
                "presentation_asset",
                &relative,
                "image has no presentation_assets row",
            );
        }
    }
    let ignored = std::fs::read_to_string(repository.join(".gitignore")).unwrap_or_default();
    if !ignored.lines().any(|line| line.trim() == "godot/.godot/") {
        issue(
            errors,
            "presentation_asset",
            ".gitignore",
            "the Godot import cache godot/.godot/ must be ignored",
        );
    }
    for reference in tables.get(REFS).into_iter().flatten() {
        let path = reference["asset_path"].as_str();
        match by_path.get(path) {
            Some(asset)
                if asset["approval_state"] == "approved"
                    && asset["asset_role"] == reference["asset_role"] => {}
            _ => issue(
                errors,
                "presentation_asset",
                &reference["catalog_id"],
                format!(
                    "{path} is not an approved {} asset",
                    reference["asset_role"]
                ),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(width: u32, height: u32, color: u8) -> Vec<u8> {
        let mut bytes = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR".to_vec();
        bytes.extend(width.to_be_bytes());
        bytes.extend(height.to_be_bytes());
        bytes.extend([8, color, 0, 0, 0]);
        bytes
    }

    const IMPORT: &str = "[remap]\n\nimporter=\"texture\"\n\n[params]\n\ncompress/mode=0\nmipmaps/generate=false\nprocess/fix_alpha_border=true\nprocess/size_limit=0\n";

    struct Repo(PathBuf);

    impl Repo {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir()
                .join(format!("reservist-assets-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(&root).unwrap();
            std::fs::write(root.join(".gitignore"), "target/\ngodot/.godot/\n").unwrap();
            std::fs::write(root.join("PROMPTS.txt"), "prompt").unwrap();
            Self(root)
        }

        fn image(&self, path: &str, bytes: &[u8], import: Option<&str>) {
            let file = self.0.join(path);
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(&file, bytes).unwrap();
            if let Some(import) = import {
                std::fs::write(self.0.join(format!("{path}.import")), import).unwrap();
            }
        }
    }

    impl Drop for Repo {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn asset(id: &str, role: &str, path: &str) -> Row {
        [
            ("asset_id", id),
            ("asset_role", role),
            ("asset_path", path),
            ("approval_state", "approved"),
            ("source_ref", "PROMPTS.txt"),
            ("rights", "project_generated"),
            ("width_px", "64"),
            ("height_px", "48"),
            ("crop", "0;0;64;48"),
            ("focal_point", "32;20"),
            ("import_preset", "pixel_art_lossless"),
            ("provenance", "test"),
        ]
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
        .collect()
    }

    /// One asset per role, each valid.
    fn world(repo: &Repo) -> Tables {
        let roles = [
            (
                "asset.portrait.test",
                "portrait",
                "assets/characters/test/portrait.png",
                2,
            ),
            (
                "asset.cutout.test",
                "actor_cutout",
                "assets/characters/test/cutout.png",
                6,
            ),
            (
                "asset.plate.test",
                "background_plate",
                "assets/plates/office.png",
                2,
            ),
            (
                "asset.event.test",
                "event_illustration",
                "assets/events/scene.png",
                2,
            ),
        ];
        let mut rows = Vec::new();
        for (id, role, path, color) in roles {
            repo.image(path, &png(64, 48, color), Some(IMPORT));
            rows.push(asset(id, role, path));
        }
        let mut tables = Tables::new();
        tables.insert(ASSETS.into(), rows);
        tables.insert(
            REFS.into(),
            vec![
                [
                    ("catalog_id", "person.us.test"),
                    ("asset_role", "portrait"),
                    ("asset_path", "assets/characters/test/portrait.png"),
                ]
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect(),
            ],
        );
        tables
    }

    fn messages(tables: &Tables, repo: &Repo) -> Vec<String> {
        let mut errors = Vec::new();
        validate_assets(tables, &repo.0, &mut errors);
        errors.into_iter().map(|issue| issue.issue).collect()
    }

    #[test]
    fn all_four_roles_validate() {
        let repo = Repo::new("valid");
        assert_eq!(messages(&world(&repo), &repo), Vec::<String>::new());
    }

    #[test]
    fn metadata_defects_are_rejected_per_role() {
        let cases: &[(usize, &str, &str, &str)] = &[
            (0, "width_px", "65", "declared size"),
            (0, "crop", "0;0;80;48", "crop must be"),
            (3, "focal_point", "70;20", "focal_point must be"),
            (
                2,
                "asset_path",
                "assets/events/office.png",
                "not a readable PNG",
            ),
            (2, "source_ref", "missing.txt", "source_ref"),
            (3, "import_preset", "lossy", "unknown import preset"),
            (0, "asset_id", "portrait.test", "must start with asset."),
            (
                3,
                "asset_role",
                "background_plate",
                "live under assets/plates/",
            ),
        ];
        for (index, field, value, expected) in cases {
            let repo = Repo::new(&format!("defect-{field}"));
            let mut tables = world(&repo);
            tables.get_mut(ASSETS).unwrap()[*index].insert((*field).into(), (*value).into());
            let found = messages(&tables, &repo);
            assert!(
                found.iter().any(|message| message.contains(expected)),
                "{field}={value}: {found:?}"
            );
        }
    }

    #[test]
    fn cutouts_need_alpha_and_imports_stay_pinned() {
        let repo = Repo::new("pins");
        let tables = world(&repo);
        repo.image("assets/characters/test/cutout.png", &png(64, 48, 2), None);
        repo.image(
            "assets/plates/office.png",
            &png(64, 48, 2),
            Some(&IMPORT.replace("compress/mode=0", "compress/mode=2")),
        );
        let found = messages(&tables, &repo);
        assert!(
            found
                .iter()
                .any(|message| message.contains("alpha channel")),
            "{found:?}"
        );
        assert!(
            found
                .iter()
                .any(|message| message.contains("compress/mode=0")),
            "{found:?}"
        );
    }

    #[test]
    fn unregistered_images_caches_and_unapproved_references_are_rejected() {
        let repo = Repo::new("tree");
        let mut tables = world(&repo);
        repo.image("assets/events/stray.png", &png(8, 8, 2), Some(IMPORT));
        std::fs::create_dir_all(repo.0.join("assets/.godot")).unwrap();
        std::fs::write(repo.0.join(".gitignore"), "target/\n").unwrap();
        tables.get_mut(ASSETS).unwrap()[0].insert("approval_state".into(), "candidate".into());
        let found = messages(&tables, &repo);
        for expected in [
            "no presentation_assets row",
            "import caches",
            "godot/.godot/",
            "not an approved portrait",
        ] {
            assert!(
                found.iter().any(|message| message.contains(expected)),
                "{expected}: {found:?}"
            );
        }
    }
}
