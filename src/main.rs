use clap::Parser;
use colored::Colorize;
use include_dir::{include_dir, Dir};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

static TEMPLATE: Dir = include_dir!("src/neon-api-template");

#[derive(Parser, Debug)]
#[command(
    name = "create-neon-api",
    version,
    about = "Bootstrap a Rust backend wired for the Neon Data API",
    long_about = None,
    styles = clap_style(),
    after_help = "\
\x1b[1;36mEXAMPLES:\x1b[0m
  \x1b[1mcreate-neon-api\x1b[0m                      # Interactive mode
  \x1b[1mcreate-neon-api my-api\x1b[0m               # Create 'my-api' directly
  \x1b[1mcreate-neon-api my-api --no-build\x1b[0m    # Skip initial cargo build"
)]
pub struct Cli {
    #[arg(help = "Project name (prompts interactively if omitted)")]
    pub project_name: Option<String>,

    #[arg(
        short = 'B',
        long = "no-build",
        help = "Skip running `cargo build` after scaffolding"
    )]
    pub no_build: bool,

    #[arg(short = 'q', long = "quiet", help = "Print only errors")]
    pub quiet: bool,
}

fn clap_style() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Cyan.into()))
                .bold(),
        )
        .error(
            anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Red.into()))
                .bold(),
        )
        .valid(anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Green.into())))
}

fn main() {
    let cli = Cli::parse();

    if !cli.quiet {
        eprintln!("{}", "  Create Neon API".bright_cyan().bold());
    }

    let project_name = match cli.project_name {
        Some(ref name) if !name.trim().is_empty() => name.trim().to_string(),
        _ => prompt_project_name(),
    };

    if project_name.is_empty() {
        eprintln!("{}  project name is required", "error:".bright_red().bold());
        std::process::exit(1);
    }
    if !is_valid_package_name(&project_name) {
        eprintln!(
            "{}  invalid project name — use lowercase letters, digits, hyphens, underscores",
            "error:".bright_red().bold()
        );
        std::process::exit(1);
    }

    let project_path = Path::new(&project_name);
    if project_path.exists() {
        eprintln!(
            "{}  '{}' already exists",
            "error:".bright_red().bold(),
            project_name.bright_white().bold()
        );
        std::process::exit(1);
    }

    if !cli.quiet {
        eprintln!("{} extracting template...", "→".bright_cyan().bold());
    }

    // Create the root directory first—Dir::extract() doesn't create it
    if let Err(e) = fs::create_dir_all(project_path) {
        eprintln!(
            "{}  failed to create project: {}",
            "error:".bright_red().bold(),
            e
        );
        std::process::exit(1);
    }

    if let Err(e) = TEMPLATE.extract(project_path) {
        eprintln!(
            "{}  failed to create project: {}",
            "error:".bright_red().bold(),
            e
        );
        std::process::exit(1);
    }

    update_cargo_toml(project_path, &project_name, cli.quiet);

    if !cli.no_build {
        if !cli.quiet {
            eprintln!();
        }
        run_cargo_build(project_path, &project_name, cli.quiet);
    }

    if !cli.quiet {
        eprintln!();
        eprintln!(
            "{}  cd {}",
            "→".bright_cyan().bold(),
            project_name.bright_white().bold()
        );
        eprintln!("   cp .env.example .env");
        eprintln!("   # edit .env with your Neon credentials");
        eprintln!("   cargo run");
    }
}

fn prompt_project_name() -> String {
    print!("{}  project name: ", "?".bright_yellow().bold());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("stdin read failed");
    input.trim().to_string()
}

fn is_valid_package_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

fn update_cargo_toml(project_path: &Path, project_name: &str, quiet: bool) {
    let cargo_toml_path = project_path.join("Cargo.toml");
    match fs::read_to_string(&cargo_toml_path) {
        Ok(content) => {
            let updated = content
                .lines()
                .map(|line| {
                    if line.starts_with("name = ") {
                        format!("name = \"{}\"", project_name)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            if let Err(e) = fs::write(&cargo_toml_path, updated) {
                eprintln!("{}  Cargo.toml: {}", "warning:".bright_yellow(), e);
            } else if !quiet {
                eprintln!("{}  Cargo.toml updated", "✓".bright_green().bold());
            }
        }
        Err(e) => {
            eprintln!("{}  Cargo.toml: {}", "warning:".bright_yellow(), e);
        }
    }
}

fn run_cargo_build(project_path: &Path, project_name: &str, quiet: bool) {
    if !quiet {
        eprintln!("{} building dependencies...", "→".bright_cyan().bold());
    }

    let spinner = if !quiet {
        let s = ProgressBar::new_spinner();
        s.set_style(
            ProgressStyle::with_template("{spinner:.cyan} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        s.set_message(format!("compiling {project_name}"));
        Some(s)
    } else {
        None
    };

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(project_path)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            if let Some(ref s) = spinner {
                s.finish_with_message("done");
            }
            if !quiet {
                eprintln!("{}  dependencies built", "✓".bright_green().bold());
            }
        }
        Ok(o) => {
            if let Some(ref s) = spinner {
                s.finish_with_message("warnings");
            }
            if !quiet {
                eprintln!(
                    "{}  cargo build finished with warnings:\n{}",
                    "warning:".bright_yellow(),
                    String::from_utf8_lossy(&o.stderr)
                );
            }
        }
        Err(e) => {
            if let Some(ref s) = spinner {
                s.finish_with_message("skipped");
            }
            eprintln!("{}  cargo build: {}", "warning:".bright_yellow(), e);
        }
    }
}
