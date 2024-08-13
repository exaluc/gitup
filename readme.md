# Gitup

Gitup is a simple Rust CLI tool designed to help you set up and configure Git on your system. This tool checks if Git is installed, installs it if necessary, and configures your Git user name and email.

## Features

- **Check Git Installation:** Gitup checks if Git is installed on your system.
- **Install Git:** If Git is not installed, Gitup will prompt you to install it (for Debian/Ubuntu systems).
- **Configure Git:** Gitup allows you to set your Git user name and email globally.
- **JSON Output:** Gitup supports JSON output for easy integration with other tools or scripts.

## Installation

To build Gitup, you'll need to have Rust installed on your system.

1. Clone the repository or create a new Rust project.
2. Navigate to the project directory.
3. Build the project using the command:
   ```
   cargo build --release
   ```
   This will create an optimized binary in the `target/release/` directory.

## Usage

You can run the Gitup tool with various options:

1. **Basic Usage:**
   ```
   ./target/release/gitup
   ```
   This command will check if Git is installed and, if not, prompt you to install it. It will then ask you to configure Git with your name and email.

2. **Provide Git User Information via Command-Line Arguments:**
   ```
   ./target/release/gitup --user "Your Name" --email "your.email@example.com"
   ```
   This command allows you to specify your Git user name and email directly via the command line.

3. **JSON Output:**
   ```
   ./target/release/gitup --json
   ```
   Use this option to get the current Git configuration in JSON format.

## Example

1. **Running with Prompts:**
   ```
   ./target/release/gitup
   ```
   - If Git is not installed, Gitup will prompt you to install it.
   - After installation, it will ask for your Git user name and email.

2. **Running with Arguments:**
   ```
   ./target/release/gitup --user "Jane Doe" --email "jane.doe@example.com"
   ```
   - Gitup will configure Git with the provided user name and email.

3. **Getting JSON Output:**
   ```
   ./target/release/gitup --json
   ```
   - Gitup will output the current Git user name and email in JSON format.

## OS
Debian

## Todo
Archlinux
Almalinux/RockyLinux
Windows
Mac

## Contributing

Feel free to open issues or submit pull requests if you want to contribute to the project.

## Author
Lucian BLETAN