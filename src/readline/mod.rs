use std::io::{self, Write};

#[cfg(unix)]
const ENTER: u8 = 0x0A;
#[cfg(unix)]
const BACKSPACE: u8 = 0x7F;
#[cfg(unix)]
const CTRL_C: u8 = 0x03;
#[cfg(unix)]
const CTRL_D: u8 = 0x04;

#[cfg(unix)]
use std::mem;
#[cfg(unix)]
use std::os::fd::AsRawFd;

/// Set terminal to raw or cooked mode
#[cfg(unix)]
fn set_raw_mode(enabled: bool) -> io::Result<()> {
    use libc::{ECHO, ICANON, TCSANOW, VMIN, VTIME, tcsetattr, termios};

    let fd = std::io::stdin().as_raw_fd();
    unsafe {
        let mut term: termios = mem::zeroed();
        if libc::tcgetattr(fd, &mut term) != 0 {
            return Err(io::Error::last_os_error());
        }

        if enabled {
            // Save original settings (we'd normally store this globally)
            let original = term;

            // Set raw mode
            term.c_lflag &= !(ICANON | ECHO);
            term.c_cc[VMIN] = 1; // Minimum number of characters for non-blocking read
            term.c_cc[VTIME] = 0; // Timeout in deciseconds

            if tcsetattr(fd, TCSANOW, &term) != 0 {
                return Err(io::Error::last_os_error());
            }

            // Note: In production code, you'd save original_term to restore later
            let _ = original;
        } else {
            // Restore cooked mode (with echo)
            term.c_lflag |= ICANON | ECHO;
            if tcsetattr(fd, TCSANOW, &term) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }
    Ok(())
}

/// Read a single byte from stdin
#[cfg(unix)]
fn read_single_byte() -> io::Result<u8> {
    use std::io::Read;
    let mut buffer = [0u8; 1];
    let n = std::io::stdin().read(&mut buffer)?;
    if n == 1 {
        Ok(buffer[0])
    } else {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Failed to read character",
        ))
    }
}

/// Read a line with history navigation support (Unix version)
#[cfg(unix)]
pub fn readline_with_history(prompt: &str, history: &[String]) -> io::Result<Option<String>> {
    let _ = set_raw_mode(true);
    let mut result = String::new();
    let mut cursor_pos = 0;
    let mut history_index: Option<usize> = None;
    let mut temp_input = String::new(); // For saving current input while browsing history

    print!("{}", prompt);
    io::stdout().flush()?;

    loop {
        use crate::unicode::read_utf8_char;

        match read_utf8_char() {
            Ok(ch) => {
                if ch as u8 == CTRL_C || ch as u8 == CTRL_D {
                    // Exit on Ctrl+C or Ctrl+D
                    let _ = set_raw_mode(false);
                    println!();
                    return Ok(None);
                } else if ch as u8 == ENTER {
                    // Enter key - submit
                    println!();
                    let _ = set_raw_mode(false);
                    return Ok(Some(result.clone()));
                } else if ch as u8 == BACKSPACE {
                    // Backspace - remove character before cursor
                    if cursor_pos > 0 {
                        // Find previous character boundary
                        let prev_char_start = result[..cursor_pos]
                            .char_indices()
                            .last()
                            .map(|(i, _)| i)
                            .unwrap_or(0);
                        result.remove(prev_char_start);
                        cursor_pos = prev_char_start;
                        // Redraw line
                        print!("\r{}{}\x1B[0K", prompt, result);
                        // Move cursor back
                        if cursor_pos < result.len() {
                            print!("\x1B[{}D", result.len() - cursor_pos);
                        }
                        io::stdout().flush()?;
                    }
                } else if ch as u8 == 0x1B {
                    // Escape sequence - check for arrow keys
                    // Read the next two characters
                    if let Ok(ch1) = read_single_byte() {
                        if ch1 == b'[' {
                            if let Ok(ch2) = read_single_byte() {
                                match ch2 {
                                    b'A' => {
                                        // Up arrow - previous history
                                        if history.is_empty() {
                                            continue;
                                        }

                                        // Save current input if this is first navigation
                                        if history_index.is_none() {
                                            temp_input = result.clone();
                                            history_index = Some(history.len());
                                        }

                                        if let Some(idx) = history_index {
                                            if idx > 0 {
                                                history_index = Some(idx - 1);
                                                result = history[idx - 1].clone();
                                                cursor_pos = result.len();
                                                // Redraw line
                                                print!("\r{}{}\x1B[0K", prompt, result);
                                                io::stdout().flush()?;
                                            }
                                        }
                                    }
                                    b'B' => {
                                        // Down arrow - next history
                                        if let Some(idx) = history_index {
                                            if idx < history.len() {
                                                if idx + 1 == history.len() {
                                                    // Return to current input
                                                    result = temp_input.clone();
                                                    history_index = None;
                                                } else {
                                                    history_index = Some(idx + 1);
                                                    result = history[idx + 1].clone();
                                                }
                                                cursor_pos = result.len();
                                                // Redraw line
                                                print!("\r{}{}\x1B[0K", prompt, result);
                                                io::stdout().flush()?;
                                            }
                                        }
                                    }
                                    b'C' => {
                                        // Right arrow - move to next character boundary
                                        if cursor_pos < result.len() {
                                            // Find next character boundary
                                            let next_pos = result[cursor_pos..]
                                                .char_indices()
                                                .nth(1)
                                                .map(|(i, _)| cursor_pos + i)
                                                .unwrap_or(result.len());
                                            cursor_pos = next_pos;
                                            print!("\x1B[C");
                                            io::stdout().flush()?;
                                        }
                                    }
                                    b'D' => {
                                        // Left arrow - move to previous character boundary
                                        if cursor_pos > 0 {
                                            // Find previous character boundary
                                            cursor_pos = result[..cursor_pos]
                                                .char_indices()
                                                .last()
                                                .map(|(i, _)| i)
                                                .unwrap_or(0);
                                            print!("\x1B[D");
                                            io::stdout().flush()?;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                } else if !ch.is_control() {
                    // Printable character
                    result.insert(cursor_pos, ch);
                    // Update cursor_pos by adding the byte length of the inserted character
                    cursor_pos += ch.len_utf8();
                    // Redraw line from cursor position
                    print!("\r{}\x1B[0K{}", prompt, &result[..cursor_pos]);
                    // Print rest of line
                    if cursor_pos < result.len() {
                        print!("{}", &result[cursor_pos..]);
                        // Move cursor back to correct position
                        print!("\x1B[{}D", result.len() - cursor_pos);
                    }
                    io::stdout().flush()?;
                }
            }
            Err(e) => {
                let _ = set_raw_mode(false);
                return Err(e);
            }
        }
    }
}

/// Read a line with history navigation support (Non-Unix fallback)
#[cfg(not(unix))]
pub fn readline_with_history(prompt: &str, _history: &[String]) -> io::Result<Option<String>> {
    // Fallback for non-Unix systems - use basic readline
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if input.trim().is_empty() {
        return Ok(None);
    }

    Ok(Some(input.trim().to_string()))
}
