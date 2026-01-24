// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::env;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::Instant;
use tauri::State;

/// Session state managed by Tauri
struct AppState {
    history: Mutex<Vec<HistoryEntry>>,
    session_count: Mutex<usize>,
    form_path: Mutex<Option<PathBuf>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryEntry {
    number: usize,
    input: String,
    output: Option<String>,
    error: Option<String>,
    duration_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
struct FormResult {
    success: bool,
    output: String,
    error: Option<String>,
    duration_ms: u64,
    session_number: usize,
}

#[derive(Debug, Serialize)]
struct AppInfo {
    version: String,
    form_path: Option<String>,
    session_count: usize,
    history_count: usize,
}

/// Find FORM executable
fn find_form_executable() -> Option<PathBuf> {
    // Check FORM_PATH environment variable
    if let Ok(form_path) = env::var("FORM_PATH") {
        let path = PathBuf::from(&form_path);
        if path.exists() {
            return Some(path);
        }
        let form_in_dir = path.join("form");
        if form_in_dir.exists() {
            return Some(form_in_dir);
        }
    }

    // Check common locations
    let locations = [
        "form",
        "sources/form",
        "../sources/form",
        "/usr/local/bin/form",
        "/usr/bin/form",
    ];

    for loc in &locations {
        let path = PathBuf::from(loc);
        if path.exists() {
            return Some(path);
        }
    }

    // Search in PATH
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

/// Execute FORM code
fn run_form(input: &str, form_path: &PathBuf) -> Result<(String, u64), String> {
    let start = Instant::now();

    // Get a writable temp directory for FORM to use
    let temp_dir = std::env::temp_dir();
    
    // Prepare input - ensure it ends with .end
    let full_input = if !input.trim_end().ends_with(".end") {
        format!("{}\n.end\n", input)
    } else {
        format!("{}\n", input)
    };

    let mut child = Command::new(form_path)
        .arg("-")
        .current_dir(&temp_dir)  // Set working directory to temp so FORM can write temp files
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn FORM: {}", e))?;

    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    stdin
        .write_all(full_input.as_bytes())
        .map_err(|e| format!("Failed to write to FORM: {}", e))?;
    drop(stdin);

    let mut output = Vec::new();
    stdout
        .read_to_end(&mut output)
        .map_err(|e| format!("Failed to read FORM output: {}", e))?;

    let mut stderr_output = Vec::new();
    stderr
        .read_to_end(&mut stderr_output)
        .map_err(|e| format!("Failed to read FORM stderr: {}", e))?;

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for FORM: {}", e))?;

    let duration_ms = start.elapsed().as_millis() as u64;
    let output_str = String::from_utf8_lossy(&output).to_string();
    let stderr_str = String::from_utf8_lossy(&stderr_output).to_string();

    if !status.success() {
        // Include both stdout and stderr in error for better debugging
        let error_msg = if !stderr_str.trim().is_empty() {
            stderr_str
        } else if !output_str.trim().is_empty() {
            // Sometimes FORM writes errors to stdout
            output_str
        } else {
            format!("FORM exited with status: {}", status)
        };
        return Err(error_msg);
    }

    Ok((format_output(&output_str), duration_ms))
}

/// Format FORM output by removing metadata
fn format_output(output: &str) -> String {
    let mut result = Vec::new();
    let mut in_header = true;

    for line in output.lines() {
        // Skip header lines
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

        // Skip timing and statistics lines
        if line.contains("sec out of") 
            || line.trim_start().starts_with("Time =")
            || line.contains("Terms in output")
            || line.contains("Bytes used")
            || line.contains("Terms active")
            || line.contains("Bytes in use")
        {
            continue;
        }

        result.push(line);
    }

    // Remove leading empty lines
    while result.first().map(|l| l.trim().is_empty()).unwrap_or(false) {
        result.remove(0);
    }

    // Remove trailing empty lines
    while result.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        result.pop();
    }

    result.join("\n")
}

/// Tauri command: Execute FORM code
#[tauri::command]
fn execute_form(input: String, state: State<AppState>) -> FormResult {
    let form_path_guard = state.form_path.lock().unwrap();
    let form_path = match form_path_guard.as_ref() {
        Some(p) => p.clone(),
        None => {
            return FormResult {
                success: false,
                output: String::new(),
                error: Some("FORM executable not found. Set FORM_PATH environment variable.".into()),
                duration_ms: 0,
                session_number: 0,
            }
        }
    };
    drop(form_path_guard);

    // Increment session count
    let mut session_count = state.session_count.lock().unwrap();
    *session_count += 1;
    let current_session = *session_count;
    drop(session_count);

    // Execute FORM
    let result = match run_form(&input, &form_path) {
        Ok((output, duration_ms)) => {
            // Add to history
            let mut history = state.history.lock().unwrap();
            history.push(HistoryEntry {
                number: current_session,
                input: input.clone(),
                output: Some(output.clone()),
                error: None,
                duration_ms: Some(duration_ms),
            });

            FormResult {
                success: true,
                output,
                error: None,
                duration_ms,
                session_number: current_session,
            }
        }
        Err(e) => {
            // Add to history
            let mut history = state.history.lock().unwrap();
            history.push(HistoryEntry {
                number: current_session,
                input: input.clone(),
                output: None,
                error: Some(e.clone()),
                duration_ms: None,
            });

            FormResult {
                success: false,
                output: String::new(),
                error: Some(e),
                duration_ms: 0,
                session_number: current_session,
            }
        }
    };

    result
}

/// Tauri command: Get history
#[tauri::command]
fn get_history(count: Option<usize>, state: State<AppState>) -> Vec<HistoryEntry> {
    let history = state.history.lock().unwrap();
    let n = count.unwrap_or(10).min(history.len());
    history.iter().rev().take(n).cloned().collect()
}

/// Tauri command: Clear history
#[tauri::command]
fn clear_history(state: State<AppState>) {
    let mut history = state.history.lock().unwrap();
    history.clear();
    let mut session_count = state.session_count.lock().unwrap();
    *session_count = 0;
}

/// Tauri command: Get app info
#[tauri::command]
fn get_app_info(state: State<AppState>) -> AppInfo {
    let form_path = state.form_path.lock().unwrap();
    let history = state.history.lock().unwrap();
    let session_count = state.session_count.lock().unwrap();

    AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        form_path: form_path.as_ref().map(|p| p.to_string_lossy().to_string()),
        session_count: *session_count,
        history_count: history.len(),
    }
}

/// Tauri command: Set FORM path manually
#[tauri::command]
fn set_form_path(path: String, state: State<AppState>) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let mut form_path = state.form_path.lock().unwrap();
    *form_path = Some(path_buf);
    Ok(format!("FORM path set to: {}", path))
}

fn main() {
    let form_path = find_form_executable();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            history: Mutex::new(Vec::new()),
            session_count: Mutex::new(0),
            form_path: Mutex::new(form_path),
        })
        .invoke_handler(tauri::generate_handler![
            execute_form,
            get_history,
            clear_history,
            get_app_info,
            set_form_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
