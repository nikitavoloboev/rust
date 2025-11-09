use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, bail, Context, Result};
use clap::{Args, Parser, Subcommand};

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
        Commands::FocusCursorWindow(args) => run_focus_cursor_window(args),
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
    /// Focus the most recent Cursor window recorded in a state file.
    FocusCursorWindow(FocusCursorWindowArgs),
}

#[derive(Args)]
struct FocusCursorWindowArgs {
    /// File that stores the last non-dot Cursor window title.
    #[arg(
        long = "state-file",
        env = "FLOW_CURSOR_LAST_WINDOW_FILE",
        value_name = "FILE"
    )]
    state_file: PathBuf,
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
        let gitignore_contents = fs::read_to_string(&gitignore_path)
            .with_context(|| format!("Unable to read {}", gitignore_path.display()))?;

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

fn run_focus_cursor_window(args: FocusCursorWindowArgs) -> Result<()> {
    let window_title = read_last_window_title(&args.state_file)?;

    println!(
        "Latest Cursor window from {}: {}",
        args.state_file.display(),
        window_title
    );

    let attempt = focus_cursor_window_by_title(&window_title)?;
    if attempt.focused {
        println!("Focused Cursor window \"{window_title}\"");
        return Ok(());
    }

    if let Some(reason) = attempt.reason {
        println!("{reason}");
    } else {
        println!("Unable to focus Cursor window \"{window_title}\"");
    }

    Ok(())
}

fn read_last_window_title(path: &Path) -> Result<String> {
    let contents = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let trimmed = contents.trim();
    if !trimmed.is_empty() {
        return Ok(trimmed.to_owned());
    }

    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        let fallback = file_name.trim();
        if !fallback.is_empty() {
            return Ok(fallback.to_owned());
        }
    }

    bail!("{} did not contain a Cursor window title", path.display());
}

struct FocusCursorAttempt {
    focused: bool,
    reason: Option<String>,
}

impl FocusCursorAttempt {
    fn focused() -> Self {
        Self {
            focused: true,
            reason: None,
        }
    }

    fn info(reason: impl Into<String>) -> Self {
        Self {
            focused: false,
            reason: Some(reason.into()),
        }
    }
}

fn focus_cursor_window_by_title(title: &str) -> Result<FocusCursorAttempt> {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        bail!("window title cannot be empty");
    }

    let script = format!(
        r#"set targetTitle to "{title}"
set matched to false

tell application "System Events"
	if not (exists application process "Cursor") then
		return "NOT_RUNNING"
	end if

	tell application process "Cursor"
		repeat with w in windows
			set winName to ""
			try
				set winName to name of w
			end try

			if winName is targetTitle then
				set matched to true
				try
					set frontmost to true
				end try
				try
					set value of attribute "AXMain" of w to true
				end try
				try
					perform action "AXRaise" of w
				end try
				exit repeat
			end if
		end repeat
	end tell
end tell

if matched then
	tell application "Cursor" to activate
	return "FOCUSED"
end if

return "NOT_FOUND""#,
        title = escape_apple_script_string(trimmed)
    );

    let result = run_osascript(&script)?;
    match result.as_str() {
        "FOCUSED" => match cursor_front_window_title() {
            Ok(current) => {
                let normalized_current = normalize_window_title(&current);
                let normalized_target = normalize_window_title(trimmed);
                if normalized_current == normalized_target {
                    Ok(FocusCursorAttempt::focused())
                } else if current.is_empty() {
                    Ok(FocusCursorAttempt::info(
                        "Cursor focused an unnamed window; please try again",
                    ))
                } else {
                    Ok(FocusCursorAttempt::info(format!(
                        "Cursor focused \"{current}\" instead"
                    )))
                }
            }
            Err(_) => Ok(FocusCursorAttempt::info(
                "Unable to verify Cursor window state",
            )),
        },
        "NOT_RUNNING" => Ok(FocusCursorAttempt::info("Cursor is not running")),
        "NOT_FOUND" => Ok(FocusCursorAttempt::info(format!(
            "No Cursor window titled \"{trimmed}\" was found"
        ))),
        other => {
            if other.is_empty() {
                bail!("focus Cursor window returned an empty response");
            }
            bail!("unexpected osascript response: {other}");
        }
    }
}

fn run_osascript(script: &str) -> Result<String> {
    let output = match Command::new("osascript").arg("-e").arg(script).output() {
        Ok(output) => output,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Err(anyhow!("osascript not found in PATH"));
            }
            return Err(err).context("failed to run osascript");
        }
    };

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let mut message = stderr;
        if message.is_empty() {
            message = stdout;
        } else if !stdout.is_empty() {
            message.push_str("; ");
            message.push_str(&stdout);
        }
        if message.is_empty() {
            message = "osascript exited with an error".to_string();
        }
        bail!(message);
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn cursor_front_window_title() -> Result<String> {
    let script = r#"tell application "System Events"
	if not (exists application process "Cursor") then
		return ""
	end if

	tell application process "Cursor"
		repeat with w in windows
			try
				if value of attribute "AXMain" of w is true then
					return name of w
				end if
			end try
		end repeat

		if (count of windows) > 0 then
			try
				return name of window 1
			end try
		end if
	end tell
end tell

return ""#;

    Ok(run_osascript(script)?)
}

fn normalize_window_title(title: &str) -> String {
    title.trim().to_owned()
}

fn escape_apple_script_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
