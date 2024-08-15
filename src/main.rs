use gitup::{backup_config, restore_config, create_profile, use_profile, is_git_installed, install_git, configure_git, get_git_config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut git_name: Option<String> = None;
    let mut git_email: Option<String> = None;
    let mut profile_name: Option<String> = None;
    let mut backup_path: Option<String> = None;
    let mut restore_path: Option<String> = None;

    let mut i = 1; // Skip the first argument which is the program name
    while i < args.len() {
        match args[i].as_str() {
            "--user" => {
                if i + 1 < args.len() {
                    git_name = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --user requires a value");
                    return;
                }
            }
            "--email" => {
                if i + 1 < args.len() {
                    git_email = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --email requires a value");
                    return;
                }
            }
            "--create-profile" => {
                if i + 1 < args.len() {
                    profile_name = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --create-profile requires a value");
                    return;
                }
            }
            "--use-profile" => {
                if i + 1 < args.len() {
                    profile_name = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --use-profile requires a value");
                    return;
                }
            }
            "--backup" => {
                if i + 1 < args.len() {
                    backup_path = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --backup requires a value");
                    return;
                }
            }
            "--restore" => {
                if i + 1 < args.len() {
                    restore_path = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --restore requires a value");
                    return;
                }
            }
            "--install" => {
                if !is_git_installed().unwrap_or(false) {
                    if install_git().is_ok() {
                        println!("Git installed successfully.");
                    } else {
                        eprintln!("Failed to install Git.");
                    }
                } else {
                    println!("Git is already installed.");
                }
            }
            "--config" => {
                if let (Some(name), Some(email)) = (git_name.clone(), git_email.clone()) {
                    if configure_git(&name, &email).is_ok() {
                        println!("Git configured successfully.");
                    } else {
                        eprintln!("Failed to configure Git.");
                    }
                } else {
                    eprintln!("Please provide both --user and --email options.");
                }
            }
            "--show-config" => {
                if let Some(name) = get_git_config("user.name").unwrap_or(None) {
                    println!("Git user.name: {}", name);
                } else {
                    println!("Git user.name is not set.");
                }

                if let Some(email) = get_git_config("user.email").unwrap_or(None) {
                    println!("Git user.email: {}", email);
                } else {
                    println!("Git user.email is not set.");
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
            }
        }
        i += 1;
    }

    /// If a profile name is provided, create or use the profile
    if let Some(profile) = profile_name {
        if let (Some(name), Some(email)) = (git_name, git_email) {
            if create_profile(&profile, &name, &email).is_ok() {
                println!("Profile '{}' created successfully.", profile);
            } else {
                eprintln!("Failed to create profile '{}'.", profile);
            }
        } else {
            if use_profile(&profile).is_ok() {
                println!("Switched to profile '{}'.", profile);
            } else {
                eprintln!("Failed to switch to profile '{}'.", profile);
            }
        }
    }

    /// If backup or restore path is provided, backup or restore the configuration
    if let Some(path) = backup_path {
        if backup_config(&path).is_ok() {
            println!("Configuration backed up to '{}'.", path);
        } else {
            eprintln!("Failed to backup configuration.");
        }
    }

    /// If restore path is provided, restore the configuration
    if let Some(path) = restore_path {
        if restore_config(&path).is_ok() {
            println!("Configuration restored from '{}'.", path);
        } else {
            eprintln!("Failed to restore configuration.");
        }
    }
}
