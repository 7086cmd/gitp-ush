use std::env;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    // If no arguments provided, show help
    if args.len() < 2 {
        println!("Usage: gitp <command>");
        println!("\nExample:");
        println!("  gitp ush    -> executes: git push -u origin <current-branch>");
        exit(0);
    }

    // Check if first argument is "ush"
    if args[1] == "ush" {
        execute_git_push(&args[2..]);
    } else {
        println!("Unknown command: {}", args[1]);
        println!("Try: gitp ush");
        exit(1);
    }
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

fn execute_git_push(extra_args: &[String]) {
    // Get the current branch
    let branch = match get_current_branch() {
        Some(b) => b,
        None => {
            eprintln!("Error: Could not determine current git branch");
            exit(1);
        }
    };

    println!("Current branch: {}", branch);

    // Build the git push command with automatic upstream
    let mut args = vec!["-u".to_string(), "origin".to_string(), branch.clone()];

    // Add any extra arguments provided by the user
    args.extend(extra_args.iter().cloned());

    println!("Executing: git push {}", args.join(" "));

    // Execute git push
    let status = Command::new("git")
        .arg("push")
        .args(&args)
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("Successfully pushed to origin/{}", branch);
            } else {
                exit(exit_status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("Error executing git push: {}", e);
            exit(1);
        }
    }
}
