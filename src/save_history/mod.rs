//! History file I/O operations
//!
//! This module provides functions for loading and saving command history
//! to/from a file. It is designed to work with the readline module which
//! handles interactive input with history navigation.

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

/// Load command history from a file
///
/// Returns an empty Vec if the file doesn't exist
pub fn load_history(path: &Path) -> std::io::Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

/// Save command history to a file
///
/// Creates a new file or truncates an existing one, then writes each
/// non-empty line from the history vector
pub fn save_history(path: &Path, history: &[String]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;
    for line in history {
        if !line.is_empty() {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}
