use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Stdio;
use tauri::command;
use tokio::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHunk {
    file_name: String,
    file_ext: String,
    hunk_header: String,
    hunk_lines: Vec<String>,
    hunk_id: String,
    stats: HunkStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HunkStats {
    added: usize,
    removed: usize,
    size: u64,
    modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitDiffResult {
    hunks: Vec<GitHunk>,
    total_stats: TotalStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotalStats {
    added: usize,
    removed: usize,
    files: usize,
}

#[command]
async fn get_git_diff(directory_path: String) -> Result<GitDiffResult, String> {
    // Check if it's a git repository
    let git_check = Command::new("git")
        .args(&["rev-parse", "--git-dir"])
        .current_dir(&directory_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to execute git command: {}", e))?;

    if !git_check.status.success() {
        return Err("Not a git repository or git not found".to_string());
    }

    // Get git diff
    let diff_output = Command::new("git")
        .args(&["diff", "HEAD"])
        .current_dir(&directory_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to execute git diff: {}", e))?;

    if !diff_output.status.success() {
        let stderr = String::from_utf8_lossy(&diff_output.stderr);
        return Err(format!("Git diff failed: {}", stderr));
    }

    let diff_text = String::from_utf8_lossy(&diff_output.stdout);
    
    if diff_text.trim().is_empty() {
        // Check for staged changes
        let staged_output = Command::new("git")
            .args(&["diff", "--cached"])
            .current_dir(&directory_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to check staged changes: {}", e))?;
        
        let staged_text = String::from_utf8_lossy(&staged_output.stdout);
        if !staged_text.trim().is_empty() {
            return Ok(parse_diff_to_hunks(&staged_text, &directory_path));
        }
        
        return Ok(GitDiffResult {
            hunks: Vec::new(),
            total_stats: TotalStats { added: 0, removed: 0, files: 0 },
        });
    }

    Ok(parse_diff_to_hunks(&diff_text, &directory_path))
}

fn parse_diff_to_hunks(diff_output: &str, base_path: &str) -> GitDiffResult {
    let mut hunks = Vec::new();
    
    if diff_output.trim().is_empty() {
        return GitDiffResult {
            hunks,
            total_stats: TotalStats { added: 0, removed: 0, files: 0 },
        };
    }

    let file_regex = Regex::new(r"^diff --git").unwrap();
    let files: Vec<&str> = file_regex.split(diff_output).skip(1).collect();

    for file_block in files {
        let lines: Vec<&str> = file_block.lines().collect();
        let file_name = extract_file_name(&lines);
        let file_ext = Path::new(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();
        
        let file_stats = get_file_stats(&format!("{}/{}", base_path, file_name));
        
        let mut i = 0;
        let mut hunk_count = 0;
        
        while i < lines.len() {
            if lines[i].starts_with("@@") {
                let hunk_header_regex = Regex::new(r"^(@@[^@]*@@)").unwrap();
                let hunk_header = if let Some(caps) = hunk_header_regex.captures(lines[i]) {
                    caps.get(1).unwrap().as_str().to_string()
                } else {
                    lines[i].to_string()
                };
                
                let mut body = Vec::new();
                i += 1;
                
                while i < lines.len() && !lines[i].starts_with("@@") && !lines[i].starts_with("diff --git") {
                    body.push(lines[i].to_string());
                    i += 1;
                }
                
                let added = body.iter().filter(|line| line.starts_with('+')).count();
                let removed = body.iter().filter(|line| line.starts_with('-')).count();
                
                hunks.push(GitHunk {
                    file_name: file_name.clone(),
                    file_ext: file_ext.clone(),
                    hunk_header,
                    hunk_lines: body,
                    hunk_id: format!("{}-{}", file_name, hunk_count),
                    stats: HunkStats {
                        added,
                        removed,
                        size: file_stats.0,
                        modified: file_stats.1.clone(),
                    },
                });
                
                hunk_count += 1;
                continue;
            }
            i += 1;
        }
    }

    let total_added = hunks.iter().map(|h| h.stats.added).sum();
    let total_removed = hunks.iter().map(|h| h.stats.removed).sum();
    let unique_files: std::collections::HashSet<_> = hunks.iter().map(|h| &h.file_name).collect();
    let file_count = unique_files.len();
    
    GitDiffResult {
        hunks,
        total_stats: TotalStats {
            added: total_added,
            removed: total_removed,
            files: file_count,
        },
    }
}

fn extract_file_name(lines: &[&str]) -> String {
    let plus_line = lines.iter().find(|line| line.starts_with("+++ "));
    let minus_line = lines.iter().find(|line| line.starts_with("--- "));
    
    if let Some(plus) = plus_line {
        if !plus.contains("/dev/null") {
            return plus.replacen("+++ b/", "", 1);
        }
    }
    
    if let Some(minus) = minus_line {
        if !minus.contains("/dev/null") {
            return minus.replacen("--- a/", "", 1);
        }
    }
    
    "Unknown file".to_string()
}

fn get_file_stats(file_path: &str) -> (u64, String) {
    match std::fs::metadata(file_path) {
        Ok(metadata) => {
            let size = metadata.len();
            let modified = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| {
                    let secs = duration.as_secs();
                    chrono::DateTime::from_timestamp(secs as i64, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .unwrap_or_else(|| "unknown".to_string());
            (size, modified)
        },
        Err(_) => (0, "unknown".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_git_diff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
