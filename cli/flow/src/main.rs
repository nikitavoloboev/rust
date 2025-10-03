use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => handle_validate(path.as_ref()),
    }
}

#[derive(Parser)]
#[command(name = "flow", version, about = "Flow CLI", propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a project directory against Flow conventions.
    Validate {
        /// Path to the project directory (defaults to current directory).
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

fn handle_validate(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("{} does not exist", path.display());
    }

    let dir = path
        .canonicalize()
        .with_context(|| format!("Unable to resolve directory {}", path.display()))?;

    if !dir.is_dir() {
        bail!("{} is not a directory", dir.display());
    }

    let mut issues = Vec::new();

    let gitignore_path = dir.join(".gitignore");
    if !gitignore_path.exists() {
        issues.push(format!(
            "Missing .gitignore file at {}",
            gitignore_path.display()
        ));
    } else {
        let gitignore_contents = fs::read_to_string(&gitignore_path).with_context(|| {
            format!("Unable to read {}", gitignore_path.display())
        })?;

        let has_core_comment = gitignore_contents
            .lines()
            .any(|line| line.trim() == "# core");

        if !has_core_comment {
            issues.push(format!(
                "{} missing required '# core' marker",
                gitignore_path.display()
            ));
        }
    }

    if issues.is_empty() {
        println!("Validation passed for {}", dir.display());
        Ok(())
    } else {
        for issue in &issues {
            eprintln!("- {issue}");
        }
        Err(anyhow!("Validation failed with {} issue(s)", issues.len()))
    }
}
