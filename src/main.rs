use clap::{Arg, Command};
use std::{fs, io, str};

fn search_in_file(file_path: &str, pattern: &str, ignore_case: bool) -> io::Result<()> {
    // Read the file as raw bytes
    let content = fs::read(file_path)?;

    // Try to decode the bytes as a valid UTF-8 string
    let content = match str::from_utf8(&content) {
        Ok(valid_str) => valid_str.to_string(),
        Err(_) => {
            eprintln!("Warning: Skipping non-UTF-8 file: {}", file_path);
            return Ok(()); // Skip this file
        }
    };

    // Apply case-insensitive search if needed
    let search_pattern = if ignore_case {
        content.to_lowercase()
    } else {
        content
    };

    let pattern = if ignore_case { pattern.to_lowercase() } else { pattern.to_string() };

    // Search through the lines of the file
    for (line_number, line) in search_pattern.lines().enumerate() {
        if line.contains(&pattern) {
            println!("Found in {}: Line {}: {}", file_path, line_number + 1, line);
        }
    }

    Ok(())
}

fn search_in_directory(path: &str, pattern: &str, ignore_case: bool) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        // Recursively search directories, process files
        if file_path.is_dir() {
            search_in_directory(file_path.to_str().unwrap(), pattern, ignore_case)?;
        } else if file_path.is_file() {
            search_in_file(file_path.to_str().unwrap(), pattern, ignore_case)?;
        }
    }

    Ok(())
}

fn main() {
    let matches = Command::new("rustgrep")
        .version("1.0")
        .about("A simple grep-like tool in Rust.")
        .arg(
            Arg::new("pattern")
                .help("Pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .default_value(".")
                .help("Path to search in (default is current directory)"),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .help("Ignore case when searching")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let ignore_case = matches.get_flag("ignore-case");

    // Search for the pattern in the directory
    if let Err(e) = search_in_directory(path, pattern, ignore_case) {
        eprintln!("Error: {}", e);
    }
}

