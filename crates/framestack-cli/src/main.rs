//! `framestack`: create, check and inspect FrameStack project manifests.

mod render;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use framestack_core::{MANIFEST_FILE, Report, check};

/// Exit code when the manifest has errors.
const EXIT_INVALID: u8 = 1;
/// Exit code when the command could not run. clap uses 2 for usage errors too.
const EXIT_FAILURE: u8 = 2;

#[derive(Parser)]
#[command(
    name = "framestack",
    version,
    about = "Create, check and inspect FrameStack project manifests",
    after_help = "Exit codes: 0 success, 1 the manifest is invalid, 2 the command could not run."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Write a starter framestack.toml into a directory.
    Init {
        /// Directory to write framestack.toml into. It must already exist.
        #[arg(default_value = ".")]
        dir: PathBuf,
        /// Project name. Defaults to the directory's name.
        #[arg(long)]
        name: Option<String>,
        /// Replace an existing framestack.toml.
        #[arg(long)]
        force: bool,
        /// Print the manifest instead of writing it.
        #[arg(long)]
        dry_run: bool,
    },
    /// Validate a manifest and report every problem found.
    Check {
        #[command(flatten)]
        manifest: ManifestArg,
        #[arg(long, value_enum, default_value_t = CheckFormat::Text)]
        format: CheckFormat,
    },
    /// Show components, relationships and build order.
    Graph {
        #[command(flatten)]
        manifest: ManifestArg,
        #[arg(long, value_enum, default_value_t = GraphFormat::Text)]
        format: GraphFormat,
    },
    /// Print the normalized manifest as JSON.
    Inspect {
        #[command(flatten)]
        manifest: ManifestArg,
    },
}

#[derive(clap::Args)]
struct ManifestArg {
    /// Manifest file, or a directory containing framestack.toml.
    #[arg(long, default_value = MANIFEST_FILE)]
    manifest: PathBuf,
}

#[derive(Clone, Copy, ValueEnum)]
enum CheckFormat {
    Text,
    Json,
}

#[derive(Clone, Copy, ValueEnum)]
enum GraphFormat {
    Text,
    Json,
    Mermaid,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error:#}");
            ExitCode::from(EXIT_FAILURE)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Init {
            dir,
            name,
            force,
            dry_run,
        } => init(&dir, name, force, dry_run),
        Command::Check { manifest, format } => {
            let (path, report) = load(&manifest.manifest)?;
            match format {
                CheckFormat::Text => print!("{}", render::check_text(&path, &report)),
                CheckFormat::Json => println!("{}", render::check_json(&path, &report)?),
            }
            Ok(exit_for(&report))
        }
        Command::Graph { manifest, format } => {
            let (path, report) = load(&manifest.manifest)?;
            let Some(valid) = valid_manifest(&path, &report) else {
                return Ok(ExitCode::from(EXIT_INVALID));
            };
            match format {
                GraphFormat::Text => print!("{}", render::graph_text(valid)),
                GraphFormat::Json => println!("{}", render::graph_json(valid)?),
                GraphFormat::Mermaid => print!("{}", render::graph_mermaid(valid)),
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Inspect { manifest } => {
            let (path, report) = load(&manifest.manifest)?;
            let Some(valid) = valid_manifest(&path, &report) else {
                return Ok(ExitCode::from(EXIT_INVALID));
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&valid.to_normalized_json()?)?
            );
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Resolves `--manifest` (a file, or a directory holding framestack.toml),
/// reads it and checks it.
fn load(argument: &Path) -> Result<(PathBuf, Report)> {
    let path = if argument.is_dir() {
        argument.join(MANIFEST_FILE)
    } else {
        argument.to_path_buf()
    };
    let source = fs::read_to_string(&path)
        .with_context(|| format!("cannot read manifest {}", path.display()))?;
    Ok((path, check(&source)))
}

/// The manifest when it has no errors; otherwise prints the check report.
fn valid_manifest<'a>(path: &Path, report: &'a Report) -> Option<&'a framestack_core::Manifest> {
    match &report.manifest {
        Some(manifest) if report.is_valid() => Some(manifest),
        _ => {
            print!("{}", render::check_text(path, report));
            None
        }
    }
}

fn exit_for(report: &Report) -> ExitCode {
    if report.is_valid() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(EXIT_INVALID)
    }
}

fn init(dir: &Path, name: Option<String>, force: bool, dry_run: bool) -> Result<ExitCode> {
    if !dir.is_dir() {
        bail!("directory {} does not exist", dir.display());
    }
    let target = dir.join(MANIFEST_FILE);
    if target.exists() && !force {
        bail!(
            "{} already exists; pass --force to replace it",
            target.display()
        );
    }
    let name = match name {
        Some(name) => name,
        None => default_project_name(dir)?,
    };
    let contents = render::starter_manifest(&name)?;

    if dry_run {
        eprintln!("dry run: would write {}", target.display());
        print!("{contents}");
    } else {
        fs::write(&target, contents)
            .with_context(|| format!("cannot write {}", target.display()))?;
        println!("wrote {}", target.display());
    }
    Ok(ExitCode::SUCCESS)
}

/// The directory's own name, resolving `.` to the real directory.
fn default_project_name(dir: &Path) -> Result<String> {
    let resolved = dir
        .canonicalize()
        .with_context(|| format!("cannot resolve {}", dir.display()))?;
    Ok(resolved
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| "project".to_owned()))
}
