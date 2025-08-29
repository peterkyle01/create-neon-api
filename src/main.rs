use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// A CLI tool to bootstrap a Rust backend from a template.
fn main() {
    println!(
        "{}",
        "🚀 Rust Backend Project Generator".bright_cyan().bold()
    );
    println!("{}", "═══════════════════════════════════".bright_cyan());

    // Prompt for project name
    print!("{}", "📝 Enter your project name: ".bright_yellow().bold());
    io::stdout().flush().unwrap();

    let mut project_name = String::new();
    io::stdin()
        .read_line(&mut project_name)
        .expect("Failed to read input");
    let project_name = project_name.trim();

    if project_name.is_empty() {
        eprintln!("{}", "❌ Project name cannot be empty!".bright_red().bold());
        return;
    }

    // Validate project name (basic validation for Cargo package names)
    if !is_valid_package_name(project_name) {
        eprintln!("{}", "❌ Invalid project name! Use only lowercase letters, numbers, hyphens, and underscores.".bright_red().bold());
        return;
    }

    // Replace with the URL of your GitHub template repository.
    let template_url = "https://github.com/peterkyle01/rust-backend-template.git";
    let project_path = Path::new(project_name);

    println!(
        "{} '{}'...",
        "🚀 Creating project".bright_green().bold(),
        project_name.bright_white().bold()
    );

    // Clone the template from the GitHub repository.
    match git2::Repository::clone(template_url, project_path) {
        Ok(_) => println!("{}", "✅ Template cloned successfully!".bright_green()),
        Err(e) => {
            eprintln!(
                "{} {}",
                "❌ Failed to clone repository:".bright_red().bold(),
                e
            );
            return;
        }
    }

    // After cloning, remove the .git directory to start fresh.
    let git_dir = project_path.join(".git");
    if let Err(e) = fs::remove_dir_all(&git_dir) {
        eprintln!(
            "{} {}",
            "⚠️ Failed to remove .git directory:".bright_yellow(),
            e
        );
    }

    // Update the Cargo.toml with the new project name
    update_cargo_toml(project_path, project_name);

    // Run an initial build to fetch and compile dependencies.
    println!(
        "{}",
        "📦 Running initial `cargo build`...".bright_blue().bold()
    );
    let status = Command::new("cargo")
        .arg("build")
        .current_dir(project_path)
        .status();

    if let Ok(s) = status {
        if s.success() {
            println!(
                "{}",
                "🎉 Project created successfully!".bright_green().bold()
            );
            println!(
                "{} `cd {}`, configure your `.env` file, and run `cargo run`.",
                "Next steps:".bright_cyan().bold(),
                project_name.bright_white().bold()
            );
        } else {
            eprintln!(
                "{}",
                "❌ `cargo build` failed. Check your template's dependencies."
                    .bright_red()
                    .bold()
            );
        }
    } else {
        eprintln!(
            "{}",
            "❌ Failed to run `cargo build`. Is Cargo installed?"
                .bright_red()
                .bold()
        );
    }
}

fn is_valid_package_name(name: &str) -> bool {
    // Basic validation for Cargo package names
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

fn update_cargo_toml(project_path: &Path, project_name: &str) {
    let cargo_toml_path = project_path.join("Cargo.toml");

    if let Ok(content) = fs::read_to_string(&cargo_toml_path) {
        // Replace the name field in Cargo.toml
        let updated_content = content
            .lines()
            .map(|line| {
                if line.starts_with("name = ") {
                    format!("name = \"{}\"", project_name)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        if let Err(e) = fs::write(&cargo_toml_path, updated_content) {
            eprintln!(
                "{} {}",
                "⚠️ Failed to update Cargo.toml:".bright_yellow(),
                e
            );
        } else {
            println!(
                "{}",
                "📝 Updated Cargo.toml with project name".bright_blue()
            );
        }
    } else {
        eprintln!(
            "{}",
            "⚠️ Could not read Cargo.toml from template".bright_yellow()
        );
    }
}
