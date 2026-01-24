// FORM execution module
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::fmt;
use std::time::{Duration, Instant};

/// Custom error type for FORM execution errors
/// Provides better type safety and error context than String
#[derive(Debug)]
pub enum FormError {
    SpawnError(std::io::Error),
    WriteError(std::io::Error),
    ReadError(std::io::Error),
    ExecutionError { status: i32, stderr: String },
    Timeout,
    InvalidUtf8(std::string::FromUtf8Error),
    NotFound,
}

impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormError::SpawnError(e) => write!(f, "Failed to spawn FORM: {}", e),
            FormError::WriteError(e) => write!(f, "Failed to write to FORM: {}", e),
            FormError::ReadError(e) => write!(f, "Failed to read from FORM: {}", e),
            FormError::ExecutionError { status, stderr } => {
                if stderr.is_empty() {
                    write!(f, "FORM exited with status {}", status)
                } else {
                    write!(f, "FORM error (exit {}): {}", status, stderr.trim())
                }
            }
            FormError::Timeout => write!(f, "FORM execution timed out"),
            FormError::InvalidUtf8(e) => write!(f, "Invalid UTF-8 in output: {}", e),
            FormError::NotFound => write!(f, "FORM executable not found"),
        }
    }
}

impl std::error::Error for FormError {}

/// Result of FORM execution with timing information
#[derive(Debug)]
pub struct FormResult {
    pub output: String,
    pub stderr: String,
    pub duration: Duration,
    pub exit_code: i32,
}

/// Finds the FORM executable in common locations.
///
/// Searches in this order:
/// 1. `FORM_PATH` environment variable (if set)
/// 2. `sources/form` (local directory)
/// 3. `../sources/form` (parent directory)  
/// 4. Directories in PATH environment variable
///
/// # Returns
///
/// `Some(PathBuf)` if found, `None` otherwise.
pub fn find_form_executable() -> Option<PathBuf> {
    // 1. Check FORM_PATH environment variable first
    if let Ok(form_path) = env::var("FORM_PATH") {
        let path = PathBuf::from(&form_path);
        if path.exists() {
            return Some(path);
        }
        // Also try as directory containing 'form'
        let form_in_dir = path.join("form");
        if form_in_dir.exists() {
            return Some(form_in_dir);
        }
    }
    
    // 2. Check local sources directory
    let local = PathBuf::from("sources/form");
    if local.exists() {
        return Some(local);
    }

    // 3. Check parent sources directory
    let parent = PathBuf::from("../sources/form");
    if parent.exists() {
        return Some(parent);
    }

    // 4. Search in PATH
    if let Ok(path) = env::var("PATH") {
        for dir in env::split_paths(&path) {
            let form_path = dir.join("form");
            if form_path.exists() {
                return Some(form_path);
            }
        }
    }
    
    None
}

/// Validates FORM code for obvious errors before execution.
/// Returns Ok(()) if valid, Err with description if invalid.
pub fn validate_input(input: &str) -> Result<(), String> {
    let lines: Vec<&str> = input.lines().collect();
    
    // Check for unbalanced parentheses/brackets
    let mut paren_count = 0i32;
    let mut bracket_count = 0i32;
    let mut brace_count = 0i32;
    
    for (line_num, line) in lines.iter().enumerate() {
        // Skip comments
        if line.trim_start().starts_with('*') {
            continue;
        }
        
        for ch in line.chars() {
            match ch {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
            
            if paren_count < 0 {
                return Err(format!("Unmatched ')' at line {}", line_num + 1));
            }
            if bracket_count < 0 {
                return Err(format!("Unmatched ']' at line {}", line_num + 1));
            }
            if brace_count < 0 {
                return Err(format!("Unmatched '}}' at line {}", line_num + 1));
            }
        }
    }
    
    if paren_count > 0 {
        return Err(format!("Unclosed parenthesis: {} '(' without matching ')'", paren_count));
    }
    if bracket_count > 0 {
        return Err(format!("Unclosed bracket: {} '[' without matching ']'", bracket_count));
    }
    if brace_count > 0 {
        return Err(format!("Unclosed brace: {} '{{' without matching '}}'", brace_count));
    }
    
    Ok(())
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
/// `Ok(FormResult)` with FORM output on success, `Err(FormError)` on failure.
pub fn run_form(input: &str, form_path: &PathBuf, verbose: bool) -> Result<FormResult, FormError> {
    let start = Instant::now();
    
    if verbose {
        eprintln!("[verbose] Running FORM with {} bytes of input", input.len());
        eprintln!("[verbose] Using FORM at: {}", form_path.display());
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

    // Prepare input - ensure it ends with .end
    let full_input = if !input.trim_end().ends_with(".end") {
        format!("{}\n.end", input)
    } else {
        input.to_string()
    };

    // Write input to FORM
    stdin.write_all(full_input.as_bytes()).map_err(FormError::WriteError)?;
    drop(stdin);

    // Read stdout and stderr
    let mut output = Vec::new();
    stdout.read_to_end(&mut output).map_err(FormError::ReadError)?;
    
    let mut stderr_output = Vec::new();
    stderr.read_to_end(&mut stderr_output).map_err(FormError::ReadError)?;

    let status = child.wait().map_err(FormError::ReadError)?;
    let duration = start.elapsed();
    
    let output_str = String::from_utf8(output).map_err(FormError::InvalidUtf8)?;
    let stderr_str = String::from_utf8_lossy(&stderr_output).to_string();

    if verbose {
        eprintln!("[verbose] FORM completed in {:?}", duration);
        if !stderr_str.is_empty() {
            eprintln!("[verbose] FORM stderr: {}", stderr_str);
        }
    }

    let exit_code = status.code().unwrap_or(-1);
    
    if !status.success() {
        return Err(FormError::ExecutionError {
            status: exit_code,
            stderr: stderr_str,
        });
    }
    
    Ok(FormResult {
        output: output_str,
        stderr: stderr_str,
        duration,
        exit_code,
    })
}

/// Formats FORM output by removing timing and metadata lines.
///
/// Filters out FORM version info, timing statistics, and other metadata,
/// leaving only the actual computation results.
///
/// # Arguments
///
/// * `output` - Raw output from FORM execution
/// * `show_timing` - Whether to include timing information
///
/// # Returns
///
/// Formatted output string with metadata removed.
pub fn format_output(output: &str, show_timing: bool) -> String {
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();
    let mut in_header = true;
    let mut timing_line = None;
    
    for line in &lines {
        // Skip FORM header lines
        if in_header {
            if line.starts_with("FORM ") 
                || line.contains("Version")
                || line.trim().is_empty()
                || line.contains("Run at:")
                || line.trim_start().starts_with("Generated terms")
            {
                continue;
            }
            in_header = false;
        }
        
        // Capture timing line separately
        if line.contains("sec out of") || line.trim_start().starts_with("Time =") {
            timing_line = Some(*line);
            continue;
        }
        
        result.push(*line);
    }
    
    // Remove trailing empty lines
    while result.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        result.pop();
    }
    
    let mut formatted = result.join("\n");
    
    // Optionally append timing
    if show_timing {
        if let Some(timing) = timing_line {
            if !formatted.is_empty() {
                formatted.push_str("\n\n");
            }
            formatted.push_str(timing.trim());
        }
    }
    
    formatted
}

/// Extract just the timing information from FORM output
pub fn extract_timing(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("sec out of") {
            return Some(line.trim().to_string());
        }
    }
    None
}

/// Parse FORM error messages for better display
pub fn parse_form_error(stderr: &str, code: &str) -> String {
    let mut result = String::new();
    let code_lines: Vec<&str> = code.lines().collect();
    
    for line in stderr.lines() {
        // Try to extract line numbers from error messages
        if line.contains("Line") || line.contains("line") {
            result.push_str(line);
            result.push('\n');
            
            // Try to find line number and show context
            if let Some(num_str) = extract_line_number(line) {
                if let Ok(line_num) = num_str.parse::<usize>() {
                    if line_num > 0 && line_num <= code_lines.len() {
                        result.push_str("    → ");
                        result.push_str(code_lines[line_num - 1]);
                        result.push('\n');
                    }
                }
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    
    result
}

fn extract_line_number(text: &str) -> Option<&str> {
    // Look for patterns like "Line 5" or "line 12"
    let text_lower = text.to_lowercase();
    if let Some(pos) = text_lower.find("line") {
        let after_line = &text[pos + 4..];
        let trimmed = after_line.trim_start();
        let num_end = trimmed.find(|c: char| !c.is_ascii_digit()).unwrap_or(trimmed.len());
        if num_end > 0 {
            return Some(&trimmed[..num_end]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_balanced_parens() {
        assert!(validate_input("id f(x) = g(x);").is_ok());
        assert!(validate_input("id f(x = g(x);").is_err());
        assert!(validate_input("id f(x)) = g(x);").is_err());
    }
    
    #[test]
    fn test_validate_brackets() {
        assert!(validate_input("id f[x] = 1;").is_ok());
        assert!(validate_input("id f[x = 1;").is_err());
    }
    
    #[test]
    fn test_format_output() {
        let output = "FORM 4.3\n\n   E =\n      x^2;\n\n  0.00 sec out of 0.00 sec\n";
        let formatted = format_output(output, false);
        assert!(formatted.contains("E ="));
        assert!(!formatted.contains("FORM"));
        assert!(!formatted.contains("sec out of"));
    }
}
