// FORM execution
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;

use super::term;

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

pub fn run_form(input: &str, form_path: &PathBuf, _verbose: bool) -> Result<String, String> {
    // Verbose logging for debugging FORM execution
    crate::vprintln!("Running FORM with input ({} bytes)", input.len());

    let mut child = Command::new(form_path)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn FORM: {}", e))?;

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();

    stdin.write_all(input.as_bytes()).map_err(|e| format!("Failed to write: {}", e))?;
    if !input.trim_end().ends_with(".end") {
        stdin.write_all(b"\n.end").map_err(|e| format!("Failed to write .end: {}", e))?;
    }
    drop(stdin);

    let mut output = Vec::new();
    stdout.read_to_end(&mut output).map_err(|e| format!("Failed to read: {}", e))?;

    let status = child.wait().map_err(|e| format!("Failed to wait: {}", e))?;
    let output_str = String::from_utf8(output).map_err(|e| format!("Invalid UTF-8: {}", e))?;

    term::verbose_println("");

    if !status.success() {
        return Err(format!("FORM exited with status: {}", status));
    }
    Ok(output_str)
}

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
