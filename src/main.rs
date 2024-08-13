use std::env;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Initialize variables for Git name, email, and JSON flag
    let mut git_name: Option<String> = None;
    let mut git_email: Option<String> = None;
    let mut output_json = false;

    // Parse command-line arguments manually
    let mut i = 1; // Start at 1 to skip the executable name
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--user" => {
                if i + 1 < args.len() {
                    git_name = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --user requires a value");
                    return;
                }
            }
            "-e" | "--email" => {
                if i + 1 < args.len() {
                    git_email = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --email requires a value");
                    return;
                }
            }
            "-j" | "--json" => {
                output_json = true;
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                return;
            }
        }
        i += 1;
    }

    // Check if Git is installed
    if !is_git_installed() {
        let error_message = "git is not installed.";
        if output_json {
            println!(
                "{{\"error\": \"{}\", \"action\": \"install git please\"}}",
                error_message
            );
            return;
        } else {
            eprintln!("{}", error_message);
            if git_name.is_some() || git_email.is_some() {
                eprintln!("Please install Git manually and run this tool again with the provided arguments.");
                return;
            } else {
                if prompt_user("Would you like to install Git? (y/n): ").to_lowercase() == "y" {
                    if install_git() {
                        println!("Git has been installed successfully.");
                        // Continue to configuration directly after installation
                    } else {
                        eprintln!("Failed to install Git. Please install it manually and run this tool again.");
                        return;
                    }
                } else {
                    eprintln!("Please install Git manually and run this tool again.");
                    return;
                }
            }
        }
    }

    // Proceed with configuration (either after checking that Git is installed or after installation)
    configure_git_if_needed(git_name, git_email, output_json);
}

fn configure_git_if_needed(git_name: Option<String>, git_email: Option<String>, output_json: bool) {
    // Check existing Git configuration
    let current_name = get_git_config("user.name");
    let current_email = get_git_config("user.email");

    if output_json {
        let json_output = format!(
            r#"{{
    "git_name": "{}",
    "git_email": "{}"
}}"#,
            current_name.unwrap_or_default(),
            current_email.unwrap_or_default()
        );
        println!("{}", json_output);
        return;
    } else {
        if let (Some(name), Some(email)) = (current_name.clone(), current_email.clone()) {
            println!("Git is already configured with the following settings:");
            println!("Name: {}", name);
            println!("Email: {}", email);

            if git_name.is_none() && git_email.is_none() {
                if prompt_user("Would you like to reconfigure Git? (y/n): ").to_lowercase() != "y" {
                    println!("Keeping existing Git configuration.");
                    return;
                }
            }
        }
    }

    // Prompt for missing Git name or email
    let git_name = git_name.or_else(|| Some(prompt_user("Enter your Git name: "))).unwrap();
    let git_email = git_email.or_else(|| Some(prompt_user("Enter your Git email: "))).unwrap();

    // Configure Git with the provided name and email
    configure_git("user.name", &git_name);
    configure_git("user.email", &git_email);
}

// Function to check if Git is installed
fn is_git_installed() -> bool {
    run_command("git", &["--version"]).is_ok()
}

// Function to get the current Git configuration
fn get_git_config(key: &str) -> Option<String> {
    run_command("git", &["config", "--global", key])
        .ok()
        .and_then(|output| {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if value.is_empty() {
                None
            } else {
                Some(value)
            }
        })
}

// Function to prompt user for input
fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Make sure the prompt is printed before reading input

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string() // Remove any trailing newline or spaces
}

// Function to install Git
fn install_git() -> bool {
    run_command("sudo", &["apt-get", "update"]).is_ok() && run_command("sudo", &["apt-get", "install", "-y", "git"]).is_ok()
}

// Function to configure Git
fn configure_git(key: &str, value: &str) {
    if run_command("git", &["config", "--global", key, value]).is_ok() {
        println!("Git {} set to '{}'", key, value);
    } else {
        eprintln!("Failed to set Git {}: {}", key, value);
    }
}

// General function to run a command and return the output or error
fn run_command(cmd: &str, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
    Command::new(cmd).args(args).output()
}
