use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum GitError {
    CommandFailed(String),
    NotFound,
    IoError(io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitProfile {
    pub name: String,
    pub email: String,
}

const PROFILE_FILE: &str = ".git_profiles.toml";

/// Checks if Git is installed on the system.
///
/// # Returns
/// - `Ok(true)` if Git is installed.
/// - `Ok(false)` if Git is not installed.
/// - `Err(GitError)` if the check fails.
pub fn is_git_installed() -> Result<bool, GitError> {
    match Command::new("git").arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(true)
            } else {
                Ok(false)
            }
        }
        Err(_) => Err(GitError::NotFound),
    }
}

/// Installs Git on the system using the appropriate package manager.
///
/// # Returns
/// - `Ok(())` if the installation is successful.
/// - `Err(GitError)` if the installation fails.
pub fn install_git() -> Result<(), GitError> {
    if cfg!(target_os = "macos") {
        if Command::new("brew").arg("--version").output().is_err() {
            return Err(GitError::CommandFailed(
                "Homebrew is not installed. Please install Homebrew first.".to_string(),
            ));
        }
        run_command("brew", &["install", "git"])
    } else if cfg!(target_os = "linux") {
        if Command::new("pacman").arg("-V").output().is_ok() {
            run_command("sudo", &["pacman", "-S", "--noconfirm", "git"])
        } else {
            run_command("sudo", &["apt-get", "install", "-y", "git"])
        }
    } else if cfg!(target_os = "windows") {
        if Command::new("choco").arg("--version").output().is_ok() {
            run_command("choco", &["install", "git", "-y"])
        } else if Command::new("winget").arg("--version").output().is_ok() {
            run_command("winget", &["install", "--id", "Git.Git", "--silent"])
        } else {
            Err(GitError::CommandFailed(
                "Neither Chocolatey nor Winget is installed. Please install one of them first."
                    .to_string(),
            ))
        }
    } else {
        Err(GitError::CommandFailed("OS not supported".to_string()))
    }
}

/// Configures Git with the provided user name and email.
///
/// # Arguments
/// - `name`: The name to set for Git configuration.
/// - `email`: The email to set for Git configuration.
///
/// # Returns
/// - `Ok(())` if the configuration is successful.
/// - `Err(GitError)` if the configuration fails.
pub fn configure_git(name: &str, email: &str) -> Result<(), GitError> {
    if Command::new("git")
        .args(&["config", "--global", "user.name", name])
        .output()
        .is_err()
    {
        return Err(GitError::CommandFailed(String::from(
            "Failed to set Git user.name",
        )));
    }

    if Command::new("git")
        .args(&["config", "--global", "user.email", email])
        .output()
        .is_err()
    {
        return Err(GitError::CommandFailed(String::from(
            "Failed to set Git user.email",
        )));
    }

    Ok(())
}

/// Retrieves the current Git configuration for a given key.
///
/// # Arguments
/// - `key`: The Git configuration key to retrieve (e.g., `user.name`).
///
/// # Returns
/// - `Ok(Some(String))` if the key is found and has a value.
/// - `Ok(None)` if the key is not set.
/// - `Err(GitError)` if the command fails.
pub fn get_git_config(key: &str) -> Result<Option<String>, GitError> {
    match Command::new("git").args(&["config", "--global", key]).output() {
        Ok(output) => {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if value.is_empty() {
                Ok(None)
            } else {
                Ok(Some(value))
            }
        }
        Err(_) => Err(GitError::CommandFailed(String::from(
            "Failed to retrieve Git configuration",
        ))),
    }
}

/// Creates a new Git profile and saves it to the profile file.
///
/// # Arguments
/// - `profile_name`: The name of the profile to create.
/// - `name`: The Git user name associated with this profile.
/// - `email`: The Git user email associated with this profile.
///
/// # Returns
/// - `Ok(())` if the profile is successfully created.
/// - `Err(GitError)` if there is an error creating the profile.
pub fn create_profile(profile_name: &str, name: &str, email: &str) -> Result<(), GitError> {
    let mut profiles = load_profiles()?;
    profiles.insert(profile_name.to_string(), GitProfile {
        name: name.to_string(),
        email: email.to_string(),
    });
    save_profiles(&profiles)
}

/// Switches to the specified Git profile.
///
/// # Arguments
/// - `profile_name`: The name of the profile to switch to.
///
/// # Returns
/// - `Ok(())` if the profile is successfully applied.
/// - `Err(GitError)` if there is an error applying the profile.
pub fn use_profile(profile_name: &str) -> Result<(), GitError> {
    let profiles = load_profiles()?;
    if let Some(profile) = profiles.get(profile_name) {
        configure_git(&profile.name, &profile.email)
    } else {
        Err(GitError::CommandFailed("Profile not found".to_string()))
    }
}

/// Backs up the current Git configuration to the specified file.
///
/// # Arguments
/// - `path`: The file path where the backup will be saved.
///
/// # Returns
/// - `Ok(())` if the backup is successful.
/// - `Err(GitError)` if there is an error during backup.
pub fn backup_config(path: &str) -> Result<(), GitError> {
    let mut file = File::create(path).map_err(GitError::IoError)?;

    if let Some(name) = get_git_config("user.name")? {
        writeln!(file, "user.name={}", name).map_err(GitError::IoError)?;
    }
    if let Some(email) = get_git_config("user.email")? {
        writeln!(file, "user.email={}", email).map_err(GitError::IoError)?;
    }

    Ok(())
}

/// Restores the Git configuration from the specified file.
///
/// # Arguments
/// - `path`: The file path from which the configuration will be restored.
///
/// # Returns
/// - `Ok(())` if the restoration is successful.
/// - `Err(GitError)` if there is an error during restoration.
pub fn restore_config(path: &str) -> Result<(), GitError> {
    let mut file = File::open(path).map_err(GitError::IoError)?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(GitError::IoError)?;

    for line in content.lines() {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            configure_git(parts[0], parts[1])?;
        }
    }

    Ok(())
}

/// Loads profiles from the profile file.
///
/// # Returns
/// - `Ok(HashMap<String, GitProfile>)` containing all profiles.
/// - `Err(GitError)` if there is an error loading the profiles.
fn load_profiles() -> Result<HashMap<String, GitProfile>, GitError> {
    let mut path = dirs::home_dir().ok_or_else(|| GitError::CommandFailed("Cannot find home directory".to_string()))?;
    path.push(PROFILE_FILE);
    if path.exists() {
        let data = fs::read_to_string(path).map_err(GitError::IoError)?;
        let profiles: HashMap<String, GitProfile> = toml::from_str(&data).map_err(|_| GitError::CommandFailed("Failed to parse profile file".to_string()))?;
        Ok(profiles)
    } else {
        Ok(HashMap::new())
    }
}

/// Saves profiles to the profile file.
///
/// # Arguments
/// - `profiles`: A reference to a HashMap containing the profiles to be saved.
///
/// # Returns
/// - `Ok(())` if the profiles are successfully saved.
/// - `Err(GitError)` if there is an error saving the profiles.
fn save_profiles(profiles: &HashMap<String, GitProfile>) -> Result<(), GitError> {
    let mut path = dirs::home_dir().ok_or_else(|| GitError::CommandFailed("Cannot find home directory".to_string()))?;
    path.push(PROFILE_FILE);
    let data = toml::to_string(profiles).map_err(|_| GitError::CommandFailed("Failed to serialize profiles".to_string()))?;
    fs::write(path, data).map_err(GitError::IoError)
}

/// General function to run a command and return the output or error.
///
/// # Arguments
/// - `cmd`: The command to run.
/// - `args`: A slice of arguments to pass to the command.
///
/// # Returns
/// - `Ok(())` if the command runs successfully.
/// - `Err(GitError)` if the command fails.
fn run_command(cmd: &str, args: &[&str]) -> Result<(), GitError> {
    Command::new(cmd)
        .args(args)
        .output()
        .map_err(|_| GitError::CommandFailed(format!("Failed to run command: {}", cmd)))
        .and_then(|output| {
            if output.status.success() {
                Ok(())
            } else {
                Err(GitError::CommandFailed(format!(
                    "Command failed with status: {}",
                    output.status
                )))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_installed() {
        assert!(is_git_installed().unwrap_or(false));
    }

    #[test]
    fn test_configure_git() {
        let name = "Test User";
        let email = "test@example.com";

        assert!(configure_git(name, email).is_ok());

        assert_eq!(get_git_config("user.name").unwrap(), Some(name.to_string()));
        assert_eq!(get_git_config("user.email").unwrap(), Some(email.to_string()));
    }

    #[test]
    fn test_get_git_config() {
        let name = "Test User";
        let _ = configure_git(name, "test@example.com");

        assert_eq!(get_git_config("user.name").unwrap(), Some(name.to_string()));
        assert_eq!(get_git_config("nonexistent.key").unwrap(), None);
    }

    #[test]
    fn test_create_and_use_profile() {
        let profile_name = "test_profile";
        let name = "Profile User";
        let email = "profile@example.com";

        assert!(create_profile(profile_name, name, email).is_ok());
        assert!(use_profile(profile_name).is_ok());

        assert_eq!(get_git_config("user.name").unwrap(), Some(name.to_string()));
        assert_eq!(get_git_config("user.email").unwrap(), Some(email.to_string()));
    }

    #[test]
    fn test_backup_and_restore_config() {
        let backup_path = "test_backup.txt";
        let name = "Backup User";
        let email = "backup@example.com";

        configure_git(name, email).unwrap();
        assert!(backup_config(backup_path).is_ok());

        configure_git("New User", "new@example.com").unwrap();
        assert_eq!(get_git_config("user.name").unwrap(), Some("New User".to_string()));

        assert!(restore_config(backup_path).is_ok());
        assert_eq!(get_git_config("user.name").unwrap(), Some(name.to_string()));
        assert_eq!(get_git_config("user.email").unwrap(), Some(email.to_string()));

        // Clean up test backup file
        let _ = fs::remove_file(backup_path);
    }
}
