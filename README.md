# Gitup - Enhanced Rust CLI for Git Configuration

Gitup is a Rust-based command-line tool designed to help users easily set up and manage Git configurations on their systems. This tool has been enhanced with additional features such as profile management, backup and restore functionality, and support for multiple operating systems.

## Features

- **Check Git Installation:** Verify if Git is installed on your system.
- **Install Git:** Automatically install Git using the appropriate package manager for your OS (supports Windows, macOS, and Linux).
- **Configure Git:** Set your Git user name and email globally.
- **Profile Management:** Create, switch between, and manage multiple Git profiles.
- **Backup and Restore:** Backup your Git configuration to a file and restore it later.

## Installation

### Prerequisites

Ensure that you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### Building the Project

1. Clone the repository:
   ```bash
   git clone https://github.com/dkrizhanovskyi/gitup.git
   ```
2. Navigate to the project directory:
   ```bash
   cd gitup
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Commands

- **Check if Git is installed:**
  ```bash
  ./target/release/gitup --install
  ```
  
- **Configure Git with user name and email:**
  ```bash
  ./target/release/gitup --config --user "Your Name" --email "your.email@example.com"
  ```

- **Create and use profiles:**
  ```bash
  ./target/release/gitup --create-profile "work" --user "Work User" --email "work@example.com"
  ./target/release/gitup --use-profile "work"
  ```

- **Backup and restore Git configuration:**
  ```bash
  ./target/release/gitup --backup "backup.txt"
  ./target/release/gitup --restore "backup.txt"
  ```

## Examples

### Creating and Switching Profiles

1. **Create a new profile:**
   ```bash
   ./target/release/gitup --create-profile "personal" --user "John Doe" --email "john.doe@example.com"
   ```

2. **Switch to an existing profile:**
   ```bash
   ./target/release/gitup --use-profile "work"
   ```

### Backup and Restore Configuration

1. **Backup your current configuration:**
   ```bash
   ./target/release/gitup --backup "git_backup.txt"
   ```

2. **Restore your configuration from a backup:**
   ```bash
   ./target/release/gitup --restore "git_backup.txt"
   ```

## Contributing

This project was originally developed by Lucian and has been enhanced by Daniil Krizhanovskyi. Contributions are welcome! Please open an issue or submit a pull request if you have any ideas for improvements or if you've found a bug.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original project by [Lucian](https://github.com/exaluc/gitup).
- Enhancements by [Daniil Krizhanovskyi](https://github.com/dkrizhanovskyi/gitup).

## Author

- Lucian BLETAN - [GitHub](https://github.com/exaluc)
- Daniil Krizhanovskyi - [GitHub](https://github.com/dkrizhanovskyi)
