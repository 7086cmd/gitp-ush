use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, show help
    if args.len() < 2 {
        println!("Usage: gitp-ush <git-command>");
        println!("\nExample:");
        println!("  gitp ush    -> automatically corrects to: git push -u origin <current-branch>");
        println!("  git push    -> executes: git push -u origin <current-branch>");
        exit(0);
    }

    // Join all arguments (skip the program name)
    let input = args[1..].join(" ");

    // Detect typos and correct them
    let corrected = autocorrect_typo(&input);

    if corrected != input {
        println!("🔧 Autocorrecting: '{}' -> '{}'", input, corrected);
    }

    // Parse the corrected command
    let parts: Vec<&str> = corrected.split_whitespace().collect();

    // Check if this is a git push command
    if parts.len() >= 2 && parts[0] == "git" && parts[1] == "push" {
        execute_git_push(&parts[2..]);
    } else {
        // Execute the command as-is
        execute_command(&corrected);
    }
}

fn autocorrect_typo(input: &str) -> String {
    // Common git typos
    let typos = vec![
        ("gitp ush", "git push"),
        ("git psuh", "git push"),
        ("git puhs", "git push"),
        ("git pus", "git push"),
        ("gi tpush", "git push"),
        ("gti push", "git push"),
    ];

    for (typo, correction) in typos {
        if input.starts_with(typo) {
            let rest = &input[typo.len()..];
            return format!("{}{}", correction, rest);
        }
    }

    input.to_string()
}

fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();
        Some(branch)
    } else {
        None
    }
}

fn execute_git_push(extra_args: &[&str]) {
    // Get the current branch
    let branch = match get_current_branch() {
        Some(b) => b,
        None => {
            eprintln!("❌ Error: Could not determine current git branch");
            exit(1);
        }
    };

    println!("📍 Current branch: {}", branch);

    // Build the git push command with automatic upstream
    let mut args = vec!["-u", "origin", &branch];

    // Add any extra arguments provided by the user
    let extra_args_owned: Vec<String> = extra_args.iter().map(|s| s.to_string()).collect();
    let extra_args_refs: Vec<&str> = extra_args_owned.iter().map(|s| s.as_str()).collect();
    args.extend(extra_args_refs);

    println!("🚀 Executing: git push {}", args.join(" "));

    // Execute git push
    let status = Command::new("git")
        .arg("push")
        .args(&args)
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("✅ Successfully pushed to origin/{}", branch);
            } else {
                exit(exit_status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("❌ Error executing git push: {}", e);
            exit(1);
        }
    }
}

fn execute_command(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let status = Command::new(parts[0])
        .args(&parts[1..])
        .status();

    match status {
        Ok(exit_status) => {
            exit(exit_status.code().unwrap_or(0));
        }
        Err(e) => {
            eprintln!("❌ Error executing command: {}", e);
            exit(1);
        }
    }
}
