Here is a `README.md` file for your Rust-based text search tool that uses the `clap` crate for argument parsing:

```markdown
# Rust Text Search Tool (Grep-like)

This is a simple command-line utility written in Rust, designed to search for a specific pattern in files within a directory (and its subdirectories). It mimics the functionality of the `grep` command, offering both case-sensitive and case-insensitive search modes.

## Features
- **Recursive Search**: The tool recursively searches through directories and subdirectories for the given pattern.
- **Case Sensitivity Option**: Supports case-sensitive and case-insensitive search modes.
- **File and Directory Support**: Handles both files and directories, printing matches found within the files.
- **UTF-8 Validation**: Skips files that are not valid UTF-8 encoded.

## Requirements
- [Rust](https://www.rust-lang.org/): Ensure you have the latest stable version of Rust installed. You can install Rust by following the [official installation guide](https://www.rust-lang.org/tools/install).

## Installation

1. **Clone the Repository**

   First, clone the repository to your local machine:

   ```bash
   git clone https://github.com/your-username/rust-text-search.git
   cd rust-text-search
   ```

2. **Build the Project**

   Use Cargo to build the project:

   ```bash
   cargo build
   ```

   This will compile the project and create a `rustgrep` binary inside the `target/debug` directory.

## Usage

### Command-Line Arguments

1. `pattern`: The text string or pattern to search for. (Required)
2. `path`: The directory to search in. (Optional, defaults to the current directory `.`)
3. `--ignore-case` (`-i`): A flag to perform a case-insensitive search. (Optional)

### Examples

#### 1. Case-Sensitive Search (Default)
To search for the pattern `error` in the current directory:

```bash
cargo run "error"
```

#### 2. Case-Insensitive Search
To search for the pattern `TODO` in the `/home/user/projects` directory, ignoring case:

```bash
cargo run "TODO" --path /home/user/projects --ignore-case
```

#### 3. Searching a Specific Directory
To search for the pattern `fix` in the `/home/user/logs` directory:

```bash
cargo run "fix" --path /home/user/logs
```

## How It Works

1. **Recursive Directory Search**: The program searches through the specified directory and its subdirectories.
2. **UTF-8 Validation**: The program reads files as raw bytes and attempts to convert them into valid UTF-8 strings. Files that cannot be decoded as UTF-8 are skipped with a warning.
3. **Search Modes**: The search can be case-sensitive or case-insensitive based on the `--ignore-case` flag.
4. **Pattern Matching**: The program reads each file line by line and checks if the given pattern exists in any line. If a match is found, the file path, line number, and line content are displayed.

## Output

When a match is found, the output is displayed as follows:

```
Found in /path/to/file.txt: Line 12: This is a line containing the word error.
```

If no matches are found, there will be no output.

## Error Handling

- If a file cannot be opened, an error message will be printed.
- Files that are not valid UTF-8 will be skipped with a warning.
- If an invalid directory path is provided, an error will be printed.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions

Feel free to fork this project and open issues or pull requests if you'd like to contribute. Contributions are welcome!

```

### Key Sections:

- **Features**: Highlights the key capabilities of the tool, such as recursive search and support for case-insensitive searches.
- **Requirements**: Specifies that Rust needs to be installed to run the tool.
- **Installation**: Describes how to clone the project and build it using Cargo.
- **Usage**: Provides examples for different ways to use the tool with different command-line arguments.
- **How It Works**: Explains the tool’s functionality and how it handles files, directories, and search modes.
- **Output**: Describes how the results are displayed.
- **Error Handling**: Explains how errors, such as invalid files or directories, are handled.
- **License**: States the licensing information for the project.
- **Contributions**: Encourages other developers to contribute to the project.

This should be a complete and helpful `README.md` for your text search tool implemented in Rust.
