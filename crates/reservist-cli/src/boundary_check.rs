use std::{collections::BTreeSet, fs, path::Path, process::Command};

use reservist_content::ContentError;

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

pub(crate) fn run(args: BoundaryArgs) -> Result<(), ContentError> {
    let (name, errors) = match args.boundary {
        Boundary::Dependencies => ("dependencies", dependency_errors()?),
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
