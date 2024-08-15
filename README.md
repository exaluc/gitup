# Gitup

[Gitup](https://github.com/exaluc/gitup) is a simple Rust CLI tool designed to help you set up and configure Git on your system. This tool checks if Git is installed, installs it if necessary, and configures your Git user name and email.

This project was originally developed by [Lucian BLETAN](https://github.com/exaluc). Recent enhancements, including profile management, backup/restore functionality, and cross-platform support, were added by [Daniil Krizhanovskyi](https://github.com/dkrizhanovskyi).

## Features

- **Check Git Installation:** Gitup checks if Git is installed on your system.
- **Install Git:** If Git is not installed, Gitup will prompt you to install it. Now supports multiple operating systems, including Debian/Ubuntu, Arch Linux, Almalinux/RockyLinux, Windows, and macOS.
- **Configure Git:** Gitup allows you to set your Git user name and email globally.
- **Profile Management:** Manage multiple Git profiles with ease (New feature).
- **Backup and Restore:** Backup and restore your Git configuration (New feature).
- **JSON Output:** Gitup supports JSON output for easy integration with other tools or scripts.

## Installation

To build Gitup, you'll need to have Rust installed on your system.

1. Clone the repository:
   ```bash
   git clone https://github.com/exaluc/gitup.git
   ```
2. Navigate to the project directory:
   ```bash
   cd gitup
   ```
3. Build the project using the command:
   ```bash
   cargo build --release
   ```
   This will create an optimized binary in the `target/release/` directory.

## Usage

You can run the Gitup tool with various options:

1. **Basic Usage:**
   ```bash
   ./target/release/gitup
   ```
   This command will check if Git is installed and, if not, prompt you to install it. It will then ask you to configure Git with your name and email.

2. **Provide Git User Information via Command-Line Arguments:**
   ```bash
   ./target/release/gitup --user "Your Name" --email "your.email@example.com"
   ```
   This command allows you to specify your Git user name and email directly via the command line.

3. **JSON Output:**
   ```bash
   ./target/release/gitup --json
   ```
   Use this option to get the current Git configuration in JSON format.

4. **Profile Management:**
   - **Create a new profile:**
     ```bash
     ./target/release/gitup --create-profile "work" --user "Work User" --email "work@example.com"
     ```
   - **Switch to an existing profile:**
     ```bash
     ./target/release/gitup --use-profile "work"
     ```

5. **Backup and Restore Configuration:**
   - **Backup your current configuration:**
     ```bash
     ./target/release/gitup --backup "git_backup.txt"
     ```
   - **Restore your configuration from a backup:**
     ```bash
     ./target/release/gitup --restore "git_backup.txt"
     ```

## Example

1. **Running with Prompts:**
   ```bash
   ./target/release/gitup
   ```
   - If Git is not installed, Gitup will prompt you to install it.
   - After installation, it will ask for your Git user name and email.

2. **Running with Arguments:**
   ```bash
   ./target/release/gitup --user "Jane Doe" --email "jane.doe@example.com"
   ```
   - Gitup will configure Git with the provided user name and email.

3. **Getting JSON Output:**
   ```bash
   ./target/release/gitup --json
   ```
   - Gitup will output the current Git user name and email in JSON format.

## OS Support

- Debian/Ubuntu
- Arch Linux (New)
- Almalinux/RockyLinux (New)
- Windows (New)
- MacOS (New)

## Contributing

This project was originally created by [Lucian BLETAN](https://github.com/exaluc), and enhanced by [Daniil Krizhanovskyi](https://github.com/dkrizhanovskyi). Feel free to open issues or submit pull requests if you want to contribute to the project.

## Authors

- **Original Author:** [Lucian BLETAN](https://github.com/exaluc)
- **Enhancements by:** [Daniil Krizhanovskyi](https://github.com/dkrizhanovskyi)
