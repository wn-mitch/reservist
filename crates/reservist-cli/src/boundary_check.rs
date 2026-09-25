use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

use reservist_content::ContentError;
use serde::Deserialize;

const FORBIDDEN_DEPENDENCIES: &[&str] = &["tokio", "hyper", "reqwest", "ureq", "rand", "getrandom"];
const FORBIDDEN_GODOT: &[&str] = &[
    "FileAccess",
    "ConfigFile",
    "ResourceSaver",
    "HTTPRequest",
    "HTTPClient",
    "TCPServer",
    "StreamPeerTCP",
    "PacketPeerUDP",
    "WebSocketPeer",
    "ENetConnection",
    "randf",
    "randi",
    "RandomNumberGenerator",
];

#[derive(clap::Args)]
pub(crate) struct BoundaryArgs {
    #[command(subcommand)]
    boundary: Boundary,
}

#[derive(clap::Subcommand)]
enum Boundary {
    /// Reject network, randomness, and wall-clock dependencies in runtime crates.
    Dependencies,
    /// Reject persistence, networking, and randomness APIs in Godot presentation sources.
    Godot,
    /// Validate documentation metadata, ownership, navigation, links, and provenance.
    Docs {
        /// Repository root to inspect. Defaults to the current workspace.
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Exercise the Godot boundary and reject engine errors even on a zero exit status.
    GodotRuntime {
        #[arg(long, default_value = "godot")]
        godot: String,
    },
}

fn repository_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("CLI manifest is nested under the workspace root")
}

fn cargo_tree(package: &str, feature_edges: bool) -> Result<String, ContentError> {
    let mut command = Command::new("cargo");
    command
        .current_dir(repository_root())
        .args(["tree", "--locked", "-p", package, "--edges"]);
    if feature_edges {
        command.arg("normal,features");
    } else {
        command
            .arg("normal")
            .args(["--prefix", "none", "--format", "{p}"]);
    }
    let output = command.output().map_err(|error| {
        ContentError::new(
            "boundaries",
            format!("could not run cargo tree for {package}: {error}"),
        )
    })?;
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map_err(|error| ContentError::new("boundaries", error.to_string()))
    } else {
        Err(ContentError::new(
            "boundaries",
            String::from_utf8_lossy(&output.stderr).trim(),
        ))
    }
}

fn dependency_errors() -> Result<Vec<String>, ContentError> {
    let mut errors = Vec::new();
    for package in ["reservist-core", "reservist-content", "reservist-godot"] {
        let tree = cargo_tree(package, false)?;
        let names = tree
            .lines()
            .filter_map(|line| line.split_whitespace().next())
            .collect::<BTreeSet<_>>();
        for forbidden in FORBIDDEN_DEPENDENCIES {
            if names.contains(forbidden) {
                errors.push(format!("{package}: forbidden dependency {forbidden}"));
            }
        }
        if names.contains("chrono")
            && cargo_tree(package, true)?.contains("chrono feature \"clock\"")
        {
            errors.push(format!("{package}: chrono clock feature enables wall time"));
        }
    }
    Ok(errors)
}

fn visit_godot_sources(path: &Path, errors: &mut Vec<String>) -> Result<(), ContentError> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            if entry.file_name() != ".godot" {
                visit_godot_sources(&path, errors)?;
            }
        } else if matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("gd" | "tscn")
        ) {
            let source = fs::read_to_string(&path)?;
            for (index, line) in source.lines().enumerate() {
                for forbidden in FORBIDDEN_GODOT {
                    if contains_identifier(line, forbidden) {
                        let relative = path.strip_prefix(repository_root()).unwrap_or(&path);
                        errors.push(format!(
                            "{}:{}: forbidden {forbidden}",
                            relative.display(),
                            index + 1
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn contains_identifier(line: &str, identifier: &str) -> bool {
    line.match_indices(identifier).any(|(index, _)| {
        let before = line[..index].chars().next_back();
        let after = line[index + identifier.len()..].chars().next();
        !before.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
            && !after.is_some_and(|character| character.is_ascii_alphanumeric() || character == '_')
    })
}

fn godot_errors() -> Result<Vec<String>, ContentError> {
    let mut errors = Vec::new();
    visit_godot_sources(&repository_root().join("godot"), &mut errors)?;
    Ok(errors)
}

fn godot_runtime_errors(executable: &str) -> Result<Vec<String>, ContentError> {
    let output = Command::new(executable)
        .arg("--headless")
        .arg("--path")
        .arg(repository_root().join("godot"))
        .args(["--script", "res://tests/run.gd"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    print!("{stdout}");
    eprint!("{stderr}");
    let mut errors = stdout
        .lines()
        .chain(stderr.lines())
        .filter(|line| {
            let line = line.trim_start();
            line.starts_with("SCRIPT ERROR:") || line.starts_with("ERROR:")
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if !output.status.success() {
        errors.push(format!("Godot exited with {}", output.status));
    }
    if !stdout
        .lines()
        .any(|line| line == "Godot boundary tests passed")
    {
        errors.push("Godot did not finish the boundary assertions.".into());
    }
    Ok(errors)
}

const DOC_STATUSES: &[&str] = &["canonical", "mandate", "evidence", "proposal", "index"];
const LEGACY_REVISION: &str = "4f70592a0874d54305125456da4f5359cda0ef6e";
const REMOVED_DOC_NAMES: &[&str] = &[
    "01-research-questions-simulator-foundations.md",
    "02-research-clowder-simulation-substrate.md",
    "03-research-comparative-simulation-games.md",
    "04-design-discussion-minimum-simulation-kernel.md",
    "05-design-discussion-representation-bible.md",
    "06-design-discussion-representation-catalog.md",
    "07-design-discussion-burrow-composition-probe.md",
    "08-research-simulator-foundations.md",
    "09-design-discussion-economist-pundit-media.md",
    "10-design-discussion-epistemic-fairness-interface.md",
    "11-design-discussion-bernankey-mvp-slice.md",
    "12-structure-outline-bernankey-mvp-cycle.md",
    "13-research-game-architecture.md",
    "14-design-discussion-game-architecture.md",
    "15-open-questions-decision-handoff.md",
    "16-agent-first-design-bible-workflow-v0_2.md",
    "17-current-design-amendment-review-draft-v0_2.md",
    "18-current-design-amendment-v0_3.md",
    "19-regional-atlas-and-supply-chains.md",
    "task.md",
    "pr-description.md",
    "RDR-2026-09-05-01-fable-design-review.md",
    "FABLE_IMPLEMENTATION_PLAN.md",
    "FABLE_IMPLEMENTATION_PLAN_PROMPT.md",
    "MARKET_LAB_REPORT.md",
    "MARKET_LAB_VALIDATION.md",
];

#[derive(Debug)]
struct Doc {
    path: PathBuf,
    id: String,
    status: String,
    depends_on: Vec<String>,
    verifies: Vec<String>,
    proposes_changes_to: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SourceMap {
    schema_version: u32,
    legacy_revision: String,
    #[serde(default)]
    source: Vec<LegacySource>,
    /// Exact repository paths cited by catalog provenance that resolve only at
    /// `legacy_revision`, such as inventory files since repartitioned.
    #[serde(default)]
    legacy_path: Vec<LegacyPath>,
}

#[derive(Debug, Deserialize)]
struct LegacyPath {
    path: String,
}

/// Legacy provenance resolvable through the source map.
struct Provenance {
    basenames: BTreeMap<String, String>,
    paths: BTreeSet<String>,
}

#[derive(Debug, Deserialize)]
struct LegacySource {
    key: String,
    path: String,
}

fn visit_files(
    path: &Path,
    predicate: &impl Fn(&Path) -> bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), ContentError> {
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            visit_files(&path, predicate, files)?;
        } else if predicate(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn markdown_files(path: &Path) -> Result<Vec<PathBuf>, ContentError> {
    let mut files = Vec::new();
    visit_files(
        path,
        &|candidate| candidate.extension().and_then(|value| value.to_str()) == Some("md"),
        &mut files,
    )?;
    files.sort();
    Ok(files)
}

fn display_path<'a>(root: &'a Path, path: &'a Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}

fn metadata_value(line: Option<&&str>, field: &str) -> Option<String> {
    let line = line?.strip_prefix(&format!("**{field}:** "))?;
    let value = line.strip_prefix('`')?.strip_suffix('`')?;
    Some(value.to_owned())
}

fn reference_list(value: &str) -> Vec<String> {
    if value == "none" {
        return Vec::new();
    }
    value
        .split(',')
        .map(|item| item.trim().trim_matches('`').to_owned())
        .filter(|item| !item.is_empty())
        .collect()
}

fn valid_doc_id(id: &str) -> bool {
    id.split('.').all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    })
}

fn parse_doc(root: &Path, path: PathBuf, errors: &mut Vec<String>) -> Option<Doc> {
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!(
                "{}: could not read Markdown: {error}",
                display_path(root, &path).display()
            ));
            return None;
        }
    };
    let lines = source.lines().collect::<Vec<_>>();
    let label = display_path(root, &path).display();
    if !lines.first().is_some_and(|line| line.starts_with("# ")) {
        errors.push(format!("{label}: document must begin with one H1"));
        return None;
    }
    if lines.iter().skip(1).any(|line| line.starts_with("# ")) {
        errors.push(format!("{label}: document contains more than one H1"));
    }
    if lines.get(1).copied() != Some("") {
        errors.push(format!("{label}: H1 must be followed by a blank line"));
    }
    let Some(id) = metadata_value(lines.get(2), "ID") else {
        errors.push(format!("{label}: missing or malformed ID metadata"));
        return None;
    };
    let Some(status) = metadata_value(lines.get(3), "Status") else {
        errors.push(format!("{label}: missing or malformed Status metadata"));
        return None;
    };
    let Some(depends_on) = metadata_value(lines.get(4), "Depends on") else {
        errors.push(format!("{label}: missing or malformed Depends on metadata"));
        return None;
    };
    if !valid_doc_id(&id) {
        errors.push(format!("{label}: malformed document ID `{id}`"));
    }
    if !DOC_STATUSES.contains(&status.as_str()) {
        errors.push(format!("{label}: unsupported status `{status}`"));
    }
    let mut verifies = Vec::new();
    let mut proposes_changes_to = Vec::new();
    match status.as_str() {
        "evidence" => match metadata_value(lines.get(5), "Verifies") {
            Some(value) => verifies = reference_list(&value),
            None => errors.push(format!(
                "{label}: evidence document requires Verifies metadata"
            )),
        },
        "proposal" => match metadata_value(lines.get(5), "Proposes changes to") {
            Some(value) => proposes_changes_to = reference_list(&value),
            None => errors.push(format!(
                "{label}: proposal document requires Proposes changes to metadata"
            )),
        },
        _ => {}
    }
    Some(Doc {
        path,
        id,
        status,
        depends_on: reference_list(&depends_on),
        verifies,
        proposes_changes_to,
    })
}

fn active_markdown_files(root: &Path) -> Result<Vec<PathBuf>, ContentError> {
    let mut files = markdown_files(&root.join("docs"))?;
    for path in markdown_files(&root.join("catalog"))? {
        if path.file_name().and_then(|value| value.to_str()) == Some("AGENTS.md") {
            files.push(path);
        }
    }
    let scenario_agents = root.join("scenarios/AGENTS.md");
    if scenario_agents.exists() {
        files.push(scenario_agents);
    }
    let claude = root.join("CLAUDE.md");
    if claude.exists() {
        files.push(claude);
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn markdown_links(source: &str) -> Vec<(String, Option<String>)> {
    let mut links = Vec::new();
    let mut rest = source;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else {
            break;
        };
        let raw = rest[..end].trim().trim_matches(['<', '>']);
        let target = raw.split_whitespace().next().unwrap_or(raw);
        if !target.is_empty()
            && !target.starts_with("http://")
            && !target.starts_with("https://")
            && !target.starts_with("mailto:")
            && !target.starts_with("doc:")
        {
            let (path, anchor) = target
                .split_once('#')
                .map_or((target, None), |(path, anchor)| {
                    (path, Some(anchor.to_owned()))
                });
            links.push((path.to_owned(), anchor));
        }
        rest = &rest[end + 1..];
    }
    links
}

fn normalized_path(base: &Path, target: &str) -> PathBuf {
    let joined = if target.starts_with('/') {
        PathBuf::from(target)
    } else {
        base.join(target)
    };
    let mut normalized = PathBuf::new();
    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(value) => normalized.push(value),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::Prefix(value) => normalized.push(value.as_os_str()),
        }
    }
    normalized
}

fn heading_anchor(heading: &str) -> String {
    let mut anchor = String::new();
    let mut separator = false;
    for character in heading.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() || character == '-' || character == '_' {
            if separator && !anchor.is_empty() {
                anchor.push('-');
            }
            separator = false;
            anchor.push(character);
        } else if character.is_whitespace() {
            separator = true;
        }
    }
    anchor
}

fn anchors(source: &str) -> BTreeSet<String> {
    source
        .lines()
        .filter_map(|line| {
            let heading = line.trim_start_matches('#');
            (heading.len() != line.len() && heading.starts_with(' '))
                .then(|| heading_anchor(heading.trim()))
        })
        .collect()
}

fn resolved_link_targets(path: &Path, source: &str) -> BTreeSet<PathBuf> {
    let base = path.parent().unwrap_or_else(|| Path::new(""));
    markdown_links(source)
        .into_iter()
        .filter(|(target, _)| !target.is_empty())
        .map(|(target, _)| normalized_path(base, &target))
        .collect()
}

fn validate_links(root: &Path, path: &Path, source: &str, errors: &mut Vec<String>) {
    let base = path.parent().unwrap_or(root);
    for (target, anchor) in markdown_links(source) {
        let resolved = if target.is_empty() {
            path.to_owned()
        } else {
            normalized_path(base, &target)
        };
        if !resolved.exists() {
            errors.push(format!(
                "{}: broken link `{}`",
                display_path(root, path).display(),
                target
            ));
            continue;
        }
        if let Some(anchor) = anchor {
            let Ok(target_source) = fs::read_to_string(&resolved) else {
                errors.push(format!(
                    "{}: could not read anchor target `{}`",
                    display_path(root, path).display(),
                    resolved.display()
                ));
                continue;
            };
            if !anchors(&target_source).contains(&anchor) {
                errors.push(format!(
                    "{}: broken anchor `#{anchor}` in `{}`",
                    display_path(root, path).display(),
                    display_path(root, &resolved).display()
                ));
            }
        }
    }
}

fn has_markdown(path: &Path) -> Result<bool, ContentError> {
    Ok(!markdown_files(path)?.is_empty())
}

fn validate_navigation(root: &Path, errors: &mut Vec<String>) -> Result<(), ContentError> {
    let docs_root = root.join("docs");
    if !docs_root.exists() {
        errors.push("docs: documentation directory is missing".into());
        return Ok(());
    }
    let mut directories = BTreeSet::new();
    for path in markdown_files(&docs_root)? {
        if let Some(parent) = path.parent() {
            directories.insert(parent.to_owned());
        }
    }
    for directory in directories {
        let agent_path = directory.join("AGENTS.md");
        if !agent_path.exists() {
            errors.push(format!(
                "{}: missing folder-local AGENTS.md",
                display_path(root, &directory).display()
            ));
            continue;
        }
        let source = fs::read_to_string(&agent_path)?;
        let targets = resolved_link_targets(&agent_path, &source);
        for entry in fs::read_dir(&directory)? {
            let entry = entry?;
            let child = entry.path();
            if entry.file_type()?.is_file()
                && child.extension().and_then(|value| value.to_str()) == Some("md")
                && child.file_name().and_then(|value| value.to_str()) != Some("AGENTS.md")
                && !targets.contains(&child)
            {
                errors.push(format!(
                    "{}: does not link immediate child `{}`",
                    display_path(root, &agent_path).display(),
                    child.file_name().unwrap_or_default().to_string_lossy()
                ));
            } else if entry.file_type()?.is_dir() && has_markdown(&child)? {
                let child_agents = child.join("AGENTS.md");
                if !targets.contains(&child) && !targets.contains(&child_agents) {
                    errors.push(format!(
                        "{}: does not link immediate child directory `{}`",
                        display_path(root, &agent_path).display(),
                        child.file_name().unwrap_or_default().to_string_lossy()
                    ));
                }
            }
        }
    }
    Ok(())
}

fn validate_front_door(
    root: &Path,
    front_door: &Path,
    required: impl IntoIterator<Item = PathBuf>,
    errors: &mut Vec<String>,
) -> Result<(), ContentError> {
    if !front_door.exists() {
        return Ok(());
    }
    let source = fs::read_to_string(front_door)?;
    let targets = resolved_link_targets(front_door, &source);
    for required_path in required {
        if !targets.contains(&required_path) {
            errors.push(format!(
                "{}: does not route `{}`",
                display_path(root, front_door).display(),
                display_path(root, &required_path).display()
            ));
        }
    }
    Ok(())
}

fn validate_front_doors(root: &Path, errors: &mut Vec<String>) -> Result<(), ContentError> {
    let design = root.join("docs/design");
    let mut design_agents = Vec::new();
    if design.exists() {
        for entry in fs::read_dir(&design)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let path = entry.path().join("AGENTS.md");
                if path.exists() {
                    design_agents.push(path);
                }
            }
        }
    }
    validate_front_door(
        root,
        &root.join("docs/CURRENT_DESIGN_BIBLE.md"),
        design_agents,
        errors,
    )?;
    for (front_door, tree) in [
        ("docs/BUILD_MANDATE.md", "docs/build/mandate"),
        ("docs/BUILD_REVIEW.md", "docs/build/review"),
    ] {
        let tree = root.join(tree);
        let leaves = markdown_files(&tree)?
            .into_iter()
            .filter(|path| path.file_name().and_then(|value| value.to_str()) != Some("AGENTS.md"));
        validate_front_door(root, &root.join(front_door), leaves, errors)?;
    }
    Ok(())
}

fn has_markdown_line_locator(source: &str) -> bool {
    source.match_indices(".md:").any(|(index, _)| {
        let suffix = &source[index + 4..];
        let mut bytes = suffix.bytes();
        let Some(first) = bytes.next() else {
            return false;
        };
        if !first.is_ascii_digit() {
            return false;
        }
        suffix
            .bytes()
            .take_while(|byte| byte.is_ascii_digit() || *byte == b'-')
            .any(|byte| byte.is_ascii_digit())
    })
}

fn markdown_basenames(source: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for (end, _) in source.match_indices(".md") {
        let prefix = &source[..end];
        let start = prefix
            .char_indices()
            .rev()
            .find(|(_, character)| {
                !(character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | '/'))
            })
            .map_or(0, |(index, character)| index + character.len_utf8());
        let candidate = format!("{}.md", &source[start..end]);
        if let Some(name) = Path::new(&candidate)
            .file_name()
            .and_then(|value| value.to_str())
        {
            names.insert(name.to_owned());
        }
    }
    names
}

fn load_source_map(root: &Path, errors: &mut Vec<String>) -> Provenance {
    let path = root.join("docs/source-map.toml");
    let source = match fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            errors.push(format!(
                "{}: could not read provenance source map: {error}",
                display_path(root, &path).display()
            ));
            return Provenance::empty();
        }
    };
    let parsed: SourceMap = match toml::from_str(&source) {
        Ok(parsed) => parsed,
        Err(error) => {
            errors.push(format!(
                "{}: invalid provenance source map: {error}",
                display_path(root, &path).display()
            ));
            return Provenance::empty();
        }
    };
    if parsed.schema_version != 1 {
        errors.push("docs/source-map.toml: schema_version must be 1".into());
    }
    if parsed.legacy_revision != LEGACY_REVISION {
        errors.push(format!(
            "docs/source-map.toml: legacy_revision must be `{LEGACY_REVISION}`"
        ));
    }
    let mut map = BTreeMap::new();
    for entry in parsed.source {
        if Path::new(&entry.path)
            .file_name()
            .and_then(|value| value.to_str())
            != Some(entry.key.as_str())
        {
            errors.push(format!(
                "docs/source-map.toml: key `{}` does not match path `{}`",
                entry.key, entry.path
            ));
        }
        if map.insert(entry.key.clone(), entry.path).is_some() {
            errors.push(format!(
                "docs/source-map.toml: duplicate source key `{}`",
                entry.key
            ));
        }
    }
    let mut paths = BTreeSet::new();
    for entry in parsed.legacy_path {
        if root.join(&entry.path).exists() {
            errors.push(format!(
                "docs/source-map.toml: legacy path `{}` still exists; cite it directly",
                entry.path
            ));
        }
        if !paths.insert(entry.path.clone()) {
            errors.push(format!(
                "docs/source-map.toml: duplicate legacy path `{}`",
                entry.path
            ));
        }
    }
    Provenance {
        basenames: map,
        paths,
    }
}

impl Provenance {
    fn empty() -> Self {
        Self {
            basenames: BTreeMap::new(),
            paths: BTreeSet::new(),
        }
    }
}

/// Repository-relative `catalog/inventory/**.csv` paths cited in `source`.
fn inventory_path_citations(source: &str) -> BTreeSet<String> {
    const PREFIX: &str = "catalog/inventory/";
    let mut paths = BTreeSet::new();
    for (start, _) in source.match_indices(PREFIX) {
        let rest = &source[start..];
        let end = rest
            .find(|character: char| {
                !(character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | '/'))
            })
            .unwrap_or(rest.len());
        let candidate = &rest[..end];
        if candidate.ends_with(".csv") {
            paths.insert(candidate.to_owned());
        }
    }
    paths
}

fn validate_catalog_provenance(
    root: &Path,
    source_map: &Provenance,
    errors: &mut Vec<String>,
) -> Result<(), ContentError> {
    let mut csv_files = Vec::new();
    visit_files(
        &root.join("catalog"),
        &|path| path.extension().and_then(|value| value.to_str()) == Some("csv"),
        &mut csv_files,
    )?;
    for path in csv_files {
        let source = fs::read_to_string(&path)?;
        for name in markdown_basenames(&source) {
            if !source_map.basenames.contains_key(&name) {
                errors.push(format!(
                    "{}: unmapped legacy provenance `{name}`",
                    display_path(root, &path).display()
                ));
            }
        }
        for cited in inventory_path_citations(&source) {
            if !root.join(&cited).is_file() && !source_map.paths.contains(&cited) {
                errors.push(format!(
                    "{}: provenance cites missing `{cited}`; add it to docs/source-map.toml legacy_path",
                    display_path(root, &path).display()
                ));
            }
        }
    }
    Ok(())
}

fn validate_relationships(docs: &[Doc], errors: &mut Vec<String>) {
    let statuses = docs
        .iter()
        .map(|doc| (doc.id.as_str(), doc.status.as_str()))
        .collect::<BTreeMap<_, _>>();
    for doc in docs {
        let validate =
            |field: &str, references: &[String], allowed: &[&str], errors: &mut Vec<String>| {
                for reference in references {
                    match statuses.get(reference.as_str()) {
                        None => errors.push(format!(
                            "{}: {field} references unknown ID `{reference}`",
                            doc.path.display()
                        )),
                        Some(status) if !allowed.contains(status) => errors.push(format!(
                            "{}: {field} cannot target {status} ID `{reference}`",
                            doc.path.display()
                        )),
                        Some(_) => {}
                    }
                }
            };
        match doc.status.as_str() {
            "canonical" => validate("Depends on", &doc.depends_on, &["canonical"], errors),
            "mandate" => validate(
                "Depends on",
                &doc.depends_on,
                &["canonical", "mandate"],
                errors,
            ),
            "evidence" => {
                validate(
                    "Depends on",
                    &doc.depends_on,
                    &["canonical", "mandate", "evidence"],
                    errors,
                );
                validate("Verifies", &doc.verifies, &["canonical", "mandate"], errors);
            }
            "proposal" => {
                validate(
                    "Depends on",
                    &doc.depends_on,
                    &["canonical", "proposal"],
                    errors,
                );
                validate(
                    "Proposes changes to",
                    &doc.proposes_changes_to,
                    &["canonical"],
                    errors,
                );
            }
            "index" => validate("Depends on", &doc.depends_on, DOC_STATUSES, errors),
            _ => {}
        }
    }
}

fn docs_advisories(root: &Path) -> Result<Vec<String>, ContentError> {
    let mut advisories = Vec::new();
    for path in active_markdown_files(root)? {
        let words = fs::read_to_string(&path)?.split_whitespace().count();
        if (501..=600).contains(&words) {
            advisories.push(format!(
                "{}: {words} words (target is at most 500)",
                display_path(root, &path).display()
            ));
        }
    }
    Ok(advisories)
}

fn docs_errors(root: &Path) -> Result<Vec<String>, ContentError> {
    let mut errors = Vec::new();
    let docs_paths = markdown_files(&root.join("docs"))?;
    let mut docs = Vec::new();
    let mut ids = BTreeMap::<String, PathBuf>::new();
    for path in &docs_paths {
        let source = fs::read_to_string(path)?;
        let label = display_path(root, path).display();
        let words = source.split_whitespace().count();
        if words >= 601 {
            errors.push(format!("{label}: {words} words exceeds hard limit 600"));
        }
        if has_markdown_line_locator(&source) {
            errors.push(format!("{label}: active Markdown contains a line locator"));
        }
        for removed in REMOVED_DOC_NAMES {
            if source.contains(removed) {
                errors.push(format!(
                    "{label}: references removed legacy document `{removed}`"
                ));
            }
        }
        validate_links(root, path, &source, &mut errors);
        if path.file_name().and_then(|value| value.to_str()) != Some("AGENTS.md")
            && let Some(doc) = parse_doc(root, path.clone(), &mut errors)
        {
            if let Some(previous) = ids.insert(doc.id.clone(), path.clone()) {
                errors.push(format!(
                    "{label}: duplicate document ID `{}` also owned by {}",
                    doc.id,
                    display_path(root, &previous).display()
                ));
            }
            docs.push(doc);
        }
    }
    for path in active_markdown_files(root)? {
        if !path.starts_with(root.join("docs")) {
            let source = fs::read_to_string(&path)?;
            let words = source.split_whitespace().count();
            if words >= 601 {
                errors.push(format!(
                    "{}: {words} words exceeds hard limit 600",
                    display_path(root, &path).display()
                ));
            }
            if has_markdown_line_locator(&source) {
                errors.push(format!(
                    "{}: active Markdown contains a line locator",
                    display_path(root, &path).display()
                ));
            }
            for removed in REMOVED_DOC_NAMES {
                if source.contains(removed) {
                    errors.push(format!(
                        "{}: references removed legacy document `{removed}`",
                        display_path(root, &path).display()
                    ));
                }
            }
            validate_links(root, &path, &source, &mut errors);
        }
    }
    validate_relationships(&docs, &mut errors);
    validate_navigation(root, &mut errors)?;
    validate_front_doors(root, &mut errors)?;
    let source_map = load_source_map(root, &mut errors);
    validate_catalog_provenance(root, &source_map, &mut errors)?;
    Ok(errors)
}

pub(crate) fn run(args: BoundaryArgs) -> Result<(), ContentError> {
    let (name, errors) = match args.boundary {
        Boundary::Dependencies => ("dependencies", dependency_errors()?),
        Boundary::Docs { root } => {
            let root = match root.as_deref() {
                Some(root) => root,
                None => repository_root(),
            };
            for advisory in docs_advisories(root)? {
                eprintln!("advisory: {advisory}");
            }
            ("docs", docs_errors(root)?)
        }
        Boundary::Godot => ("godot", godot_errors()?),
        Boundary::GodotRuntime { godot } => ("godot-runtime", godot_runtime_errors(&godot)?),
    };
    if errors.is_empty() {
        println!("{name} boundary passed");
        return Ok(());
    }
    for error in &errors {
        eprintln!("{error}");
    }
    Err(ContentError::new(
        "boundaries",
        format!("{} {name} boundary violation(s)", errors.len()),
    ))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

    struct Fixture {
        root: PathBuf,
    }

    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "reservist-docs-boundary-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            if root.exists() {
                fs::remove_dir_all(&root).unwrap();
            }
            fs::create_dir_all(root.join("docs/design/state")).unwrap();
            fs::create_dir_all(root.join("catalog")).unwrap();
            let fixture = Self { root };
            fixture.write(
                "docs/AGENTS.md",
                "# Documentation\n\n[Current design](CURRENT_DESIGN_BIBLE.md)\n\n[Design](design/AGENTS.md)\n",
            );
            fixture.write(
                "docs/CURRENT_DESIGN_BIBLE.md",
                "# Current design\n\n**ID:** `index.current-design`\n**Status:** `index`\n**Depends on:** `none`\n\n[State](design/state/AGENTS.md)\n",
            );
            fixture.write(
                "docs/design/AGENTS.md",
                "# Design\n\n[State](state/AGENTS.md)\n",
            );
            fixture.write(
                "docs/design/state/AGENTS.md",
                "# State\n\n[Rule](rule.md)\n",
            );
            fixture.write(
                "docs/design/state/rule.md",
                "# Rule\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `none`\n\nThe canonical rule.\n",
            );
            fixture.write(
                "docs/source-map.toml",
                "schema_version = 1\nlegacy_revision = \"4f70592a0874d54305125456da4f5359cda0ef6e\"\n",
            );
            fixture
        }

        fn write(&self, path: &str, source: &str) {
            let path = self.root.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, source).unwrap();
        }

        fn errors(&self) -> Vec<String> {
            docs_errors(&self.root).unwrap()
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).unwrap();
        }
    }

    #[test]
    fn accepts_valid_documentation_tree() {
        assert_eq!(Fixture::new().errors(), Vec::<String>::new());
    }

    #[test]
    fn rejects_duplicate_ownership() {
        let fixture = Fixture::new();
        fixture.write(
            "docs/design/state/other.md",
            "# Other\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `none`\n",
        );
        fixture.write(
            "docs/design/state/AGENTS.md",
            "# State\n\n[Rule](rule.md)\n\n[Other](other.md)\n",
        );
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("duplicate document ID"))
        );
    }

    #[test]
    fn rejects_invalid_dependency_edge() {
        let fixture = Fixture::new();
        fixture.write(
            "docs/design/state/proposal.md",
            "# Proposal\n\n**ID:** `proposal.state.rule`\n**Status:** `proposal`\n**Depends on:** `none`\n**Proposes changes to:** `design.state.rule`\n",
        );
        fixture.write(
            "docs/design/state/rule.md",
            "# Rule\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `proposal.state.rule`\n",
        );
        fixture.write(
            "docs/design/state/AGENTS.md",
            "# State\n\n[Rule](rule.md)\n\n[Proposal](proposal.md)\n",
        );
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("Depends on cannot target proposal"))
        );
    }

    #[test]
    fn inventory_path_provenance_must_exist_or_be_a_mapped_legacy_path() {
        let fixture = Fixture::new();
        let cited = "catalog/inventory/retired/entities.csv";
        fixture.write(
            "catalog/entities.csv",
            &format!("catalog_id,provenance\nperson.test,{cited}\n"),
        );
        assert!(fixture.errors().iter().any(|error| {
            error.contains("provenance cites missing `catalog/inventory/retired/entities.csv`")
        }));
        fixture.write(
            "docs/source-map.toml",
            &format!("schema_version = 1\nlegacy_revision = \"4f70592a0874d54305125456da4f5359cda0ef6e\"\n\n[[legacy_path]]\npath = \"{cited}\"\n"),
        );
        assert_eq!(fixture.errors(), Vec::<String>::new());
        fixture.write(cited, "catalog_id,provenance\n");
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("still exists; cite it directly"))
        );
    }

    #[test]
    fn rejects_601_words() {
        let fixture = Fixture::new();
        let source = format!(
            "# Rule\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `none`\n\n{}",
            "word ".repeat(601)
        );
        fixture.write("docs/design/state/rule.md", &source);
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("exceeds hard limit 600"))
        );
    }

    #[test]
    fn rejects_missing_navigation() {
        let fixture = Fixture::new();
        fixture.write("docs/design/state/AGENTS.md", "# State\n");
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("does not link immediate child `rule.md`"))
        );
    }

    #[test]
    fn rejects_broken_link() {
        let fixture = Fixture::new();
        fixture.write(
            "docs/design/state/rule.md",
            "# Rule\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `none`\n\n[Missing](missing.md)\n",
        );
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("broken link `missing.md`"))
        );
    }

    #[test]
    fn rejects_active_markdown_line_locator() {
        let fixture = Fixture::new();
        fixture.write(
            "docs/design/state/rule.md",
            "# Rule\n\n**ID:** `design.state.rule`\n**Status:** `canonical`\n**Depends on:** `none`\n\nDo not cite legacy.md:12 here.\n",
        );
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("active Markdown contains a line locator"))
        );
    }

    #[test]
    fn rejects_unmapped_catalog_locator() {
        let fixture = Fixture::new();
        fixture.write(
            "catalog/entities.csv",
            "catalog_id,provenance\nentity.one,legacy-source.md:12\n",
        );
        assert!(
            fixture
                .errors()
                .iter()
                .any(|error| error.contains("unmapped legacy provenance `legacy-source.md`"))
        );
    }
}
