// FORM execution
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fmt;

use super::term;

/// Custom error type for FORM execution errors
/// Provides better type safety and error context than String
#[derive(Debug)]
pub enum FormError {
    SpawnError(std::io::Error),
    WriteError(std::io::Error),
    ReadError(std::io::Error),
    ExecutionError(String),
    InvalidUtf8(std::string::FromUtf8Error),
}

impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormError::SpawnError(e) => write!(f, "Failed to spawn FORM: {}", e),
            FormError::WriteError(e) => write!(f, "Failed to write to FORM: {}", e),
            FormError::ReadError(e) => write!(f, "Failed to read from FORM: {}", e),
            FormError::ExecutionError(msg) => write!(f, "FORM execution error: {}", msg),
            FormError::InvalidUtf8(e) => write!(f, "Invalid UTF-8 in output: {}", e),
        }
    }
}

impl std::error::Error for FormError {}

/// Finds the FORM executable in common locations.
///
/// Searches in this order:
/// 1. `sources/form` (local directory)
/// 2. `../sources/form` (parent directory)  
/// 3. Directories in PATH environment variable
///
/// # Returns
///
/// `Some(PathBuf)` if found, `None` otherwise.
pub fn find_form_executable() -> Option<PathBuf> {
    let local = PathBuf::from("sources/form");
    if local.exists() { return Some(local); }

    let parent = PathBuf::from("../sources/form");
    if parent.exists() { return Some(parent); }

    if let Ok(path) = env::var("PATH") {
        for dir in env::split_paths(&path) {
            let form_path = dir.join("form");
            if form_path.exists() { return Some(form_path); }
        }
    }
    None
}

/// Executes FORM with the given input.
///
/// # Arguments
///
/// * `input` - The FORM code to execute
/// * `form_path` - Path to the FORM executable
/// * `verbose` - Enable verbose debug output
///
/// # Returns
///
/// `Ok(String)` with FORM output on success, `Err(FormError)` on failure.
pub fn run_form(input: &str, form_path: &PathBuf, verbose: bool) -> Result<String, FormError> {
    // Verbose logging for debugging FORM execution
    if verbose {
        crate::vprintln!("Running FORM with input ({} bytes)", input.len());
    }

    let mut child = Command::new(form_path)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(FormError::SpawnError)?;

    // Get handles to stdin, stdout, and stderr
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    // Write input to FORM
    stdin.write_all(input.as_bytes()).map_err(FormError::WriteError)?;
    if !input.trim_end().ends_with(".end") {
        stdin.write_all(b"\n.end").map_err(FormError::WriteError)?;
    }
    drop(stdin);

    // Read stdout and stderr
    let mut output = Vec::new();
    stdout.read_to_end(&mut output).map_err(FormError::ReadError)?;
    
    // Capture and report stderr if present
    let mut stderr_output = Vec::new();
    stderr.read_to_end(&mut stderr_output).map_err(FormError::ReadError)?;
    if !stderr_output.is_empty() && verbose {
        eprintln!("FORM stderr: {}", String::from_utf8_lossy(&stderr_output));
    }

    let status = child.wait().map_err(FormError::ReadError)?;
    let output_str = String::from_utf8(output).map_err(FormError::InvalidUtf8)?;

    if verbose {
        term::verbose_println("");
    }

    if !status.success() {
        return Err(FormError::ExecutionError(format!("FORM exited with status: {}", status)));
    }
    Ok(output_str)
}

/// Formats FORM output by removing timing and metadata lines.
///
/// Filters out FORM version info, timing statistics, and other metadata,
/// leaving only the actual computation results.
///
/// # Arguments
///
/// * `output` - Raw output from FORM execution
///
/// # Returns
///
/// Formatted output string with metadata removed.
pub fn format_output(output: &str) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();
    let mut skip_timing = true;
    for line in lines {
        if skip_timing && (line.starts_with("FORM ") || line.contains("sec out of")
            || line.trim_start().starts_with("Time =")
            || line.trim_start().starts_with("Generated terms")) {
            continue;
        }
        skip_timing = false;
        result.push(line);
    }
    result.join("\n")
}
