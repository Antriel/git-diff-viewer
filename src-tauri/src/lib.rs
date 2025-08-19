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

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRef {
    name: String,
    ref_type: String, // "branch", "tag", "commit"
    short_hash: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRefs {
    branches: Vec<GitRef>,
    recent_commits: Vec<GitRef>,
}

#[command]
async fn open_file_in_editor(
    file_path: String,
    working_directory: String,
    line_number: Option<u32>,
) -> Result<(), String> {
    // Normalize and construct absolute path
    let absolute_path = if Path::new(&file_path).is_absolute() {
        file_path.clone()
    } else {
        let working_path = Path::new(&working_directory);
        let file_relative = Path::new(&file_path);
        working_path
            .join(file_relative)
            .to_str()
            .ok_or("Invalid path encoding")?
            .to_string()
    };

    // Try to find an available editor across all platforms
    let code_variants = if cfg!(target_os = "windows") {
        vec!["code.cmd", "code.exe", "code"]
    } else {
        vec!["code"]
    };

    let mut code_result = None;
    let mut working_code_cmd = None;

    for variant in code_variants {
        let result = Command::new(variant).arg("--version").output().await;
        if result.is_ok() && result.as_ref().unwrap().status.success() {
            code_result = Some(result.unwrap());
            working_code_cmd = Some(variant);
            break;
        }
    }

    // Also try using Windows cmd to resolve the path
    if code_result.is_none() && cfg!(target_os = "windows") {
        let cmd_result = Command::new("cmd")
            .args(&["/c", "code", "--version"])
            .output()
            .await;
        if cmd_result.is_ok() && cmd_result.as_ref().unwrap().status.success() {
            code_result = Some(cmd_result.unwrap());
            working_code_cmd = Some("cmd-code");
        }
    }

    let cmd = if let Some(_) = code_result {
        working_code_cmd.unwrap_or("code")
    } else {
        let subl_result = Command::new("subl").arg("--version").output().await;
        if subl_result.is_ok() && subl_result.unwrap().status.success() {
            "subl"
        } else {
            let atom_result = Command::new("atom").arg("--version").output().await;
            if atom_result.is_ok() && atom_result.unwrap().status.success() {
                "atom"
            } else if cfg!(target_os = "windows") {
                let notepadpp_result = Command::new("notepad++").arg("--version").output().await;
                if notepadpp_result.is_ok() && notepadpp_result.unwrap().status.success() {
                    "notepad++"
                } else {
                    let notepad_result = Command::new("notepad").arg("/?").output().await;
                    if notepad_result.is_ok() {
                        "notepad"
                    } else {
                        return Err("No supported editor found. Please install VS Code, Sublime Text, or another supported editor.".to_string());
                    }
                }
            } else {
                return Err("No supported editor found (code, subl, or atom)".to_string());
            }
        }
    };

    let mut command = if cmd == "cmd-code" {
        let mut c = Command::new("cmd");
        c.arg("/c").arg("code");
        c
    } else {
        Command::new(cmd)
    };

    // Configure command arguments based on editor
    match cmd {
        cmd if cmd.starts_with("code") || cmd == "cmd-code" => {
            if cmd != "cmd-code" {
                command.arg("-r"); // Reuse existing window
            } else {
                command.arg("-r"); // This will be passed to the 'code' command
            }
            if let Some(line) = line_number {
                command.arg("-g").arg(format!("{}:{}", absolute_path, line));
            } else {
                command.arg(&absolute_path);
            }
        }
        "subl" => {
            // Sublime Text
            if let Some(line) = line_number {
                command.arg(format!("{}:{}", absolute_path, line));
            } else {
                command.arg(&absolute_path);
            }
        }
        "atom" => {
            // Atom
            if let Some(line) = line_number {
                command.arg(format!("{}:{}", absolute_path, line));
            } else {
                command.arg(&absolute_path);
            }
        }
        "notepad++" => {
            // Notepad++
            if let Some(line) = line_number {
                command.arg("-n").arg(line.to_string()).arg(&absolute_path);
            } else {
                command.arg(&absolute_path);
            }
        }
        "notepad" => {
            // Basic notepad doesn't support line numbers
            command.arg(&absolute_path);
        }
        _ => {
            command.arg(&absolute_path);
        }
    }

    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to execute editor command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Editor command failed: {}", stderr));
    }

    Ok(())
}

#[command]
async fn get_git_refs(directory_path: String) -> Result<GitRefs, String> {
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

    let mut git_refs = GitRefs {
        branches: Vec::new(),
        recent_commits: Vec::new(),
    };

    // Get branches
    let branch_output = Command::new("git")
        .args(&["branch", "-a", "--format=%(refname:short)"])
        .current_dir(&directory_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to get branches: {}", e))?;

    if branch_output.status.success() {
        let branch_text = String::from_utf8_lossy(&branch_output.stdout);
        for line in branch_text.lines() {
            let branch_name = line.trim();
            if !branch_name.is_empty() && !branch_name.starts_with("origin/HEAD") {
                git_refs.branches.push(GitRef {
                    name: branch_name.to_string(),
                    ref_type: "branch".to_string(),
                    short_hash: None,
                    message: None,
                });
            }
        }
    }

    // Get recent commits
    let commit_output = Command::new("git")
        .args(&["log", "--oneline", "-20", "--pretty=format:%h|%s"])
        .current_dir(&directory_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to get recent commits: {}", e))?;

    if commit_output.status.success() {
        let commit_text = String::from_utf8_lossy(&commit_output.stdout);
        for line in commit_text.lines() {
            let parts: Vec<&str> = line.splitn(2, '|').collect();
            if parts.len() == 2 {
                git_refs.recent_commits.push(GitRef {
                    name: format!("{} - {}", parts[0], parts[1]),
                    ref_type: "commit".to_string(),
                    short_hash: Some(parts[0].to_string()),
                    message: Some(parts[1].to_string()),
                });
            }
        }
    }

    Ok(git_refs)
}

#[command]
async fn get_git_diff(
    directory_path: String,
    context_lines: Option<u32>,
    include_untracked: Option<bool>,
    comparison_source: Option<String>, // "working" or "staged"
    comparison_target: Option<String>, // target ref (HEAD, branch, commit)
) -> Result<GitDiffResult, String> {
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

    // Get git diff with source/target model
    let context_arg = format!("-U{}", context_lines.unwrap_or(3));
    let mut diff_args = vec!["diff".to_string(), context_arg.clone()];

    // Determine what to compare based on source and target
    let source = comparison_source.as_deref().unwrap_or("working");
    let target = comparison_target.as_deref().unwrap_or("HEAD");

    match source {
        "staged" => {
            // Compare staged files against target
            diff_args.push("--staged".to_string());
            if target != "HEAD" {
                diff_args.push(target.to_string());
            }
        }
        "working" => {
            // Compare working directory against target
            diff_args.push(target.to_string());
        }
        _ => {
            // Source is a commit/branch, compare it against target (commit-to-commit)
            if source == target {
                // Same source and target would result in no diff, so compare source against HEAD
                diff_args.push(format!("{}..HEAD", source));
            } else {
                diff_args.push(format!("{}..{}", source, target));
            }
        }
    }

    let diff_output = Command::new("git")
        .args(&diff_args.iter().map(|s| s.as_str()).collect::<Vec<_>>())
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
    let mut all_diff_text = diff_text.to_string();

    // Handle untracked files if requested
    if include_untracked.unwrap_or(false) {
        let untracked_output = Command::new("git")
            .args(&["ls-files", "--others", "--exclude-standard"])
            .current_dir(&directory_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to get untracked files: {}", e))?;

        let untracked_files = String::from_utf8_lossy(&untracked_output.stdout);

        for untracked_file in untracked_files.lines() {
            if !untracked_file.trim().is_empty() {
                // Get diff for untracked file using git diff --no-index NUL filepath
                let null_device = if cfg!(target_os = "windows") {
                    "NUL"
                } else {
                    "/dev/null"
                };
                let untracked_diff = Command::new("git")
                    .args(&[
                        "diff",
                        "--no-index",
                        &context_arg,
                        null_device,
                        untracked_file,
                    ])
                    .current_dir(&directory_path)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .await
                    .map_err(|e| {
                        format!("Failed to diff untracked file {}: {}", untracked_file, e)
                    })?;

                let untracked_diff_text = String::from_utf8_lossy(&untracked_diff.stdout);
                if !untracked_diff_text.trim().is_empty() {
                    if !all_diff_text.trim().is_empty() {
                        all_diff_text.push('\n');
                    }
                    all_diff_text.push_str(&untracked_diff_text);
                }
            }
        }
    }

    if all_diff_text.trim().is_empty() && source == "working" && target == "HEAD" {
        // Only check for staged changes when doing default comparison (working vs HEAD)
        let staged_output = Command::new("git")
            .args(&["diff", &context_arg, "--cached"])
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
            total_stats: TotalStats {
                added: 0,
                removed: 0,
                files: 0,
            },
        });
    }

    Ok(parse_diff_to_hunks(&all_diff_text, &directory_path))
}

fn parse_diff_to_hunks(diff_output: &str, base_path: &str) -> GitDiffResult {
    let mut hunks = Vec::new();

    if diff_output.trim().is_empty() {
        return GitDiffResult {
            hunks,
            total_stats: TotalStats {
                added: 0,
                removed: 0,
                files: 0,
            },
        };
    }

    let file_regex = Regex::new(r"(?m)^diff --git").unwrap();
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

                while i < lines.len()
                    && !lines[i].starts_with("@@")
                    && !lines[i].starts_with("diff --git")
                {
                    // Skip Git's "No newline at end of file" notice
                    if !lines[i].starts_with("\\ No newline at end of file") {
                        body.push(lines[i].to_string());
                    }
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
                    let nanos = duration.subsec_nanos();
                    chrono::DateTime::from_timestamp(secs as i64, nanos)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_else(|| "unknown".to_string())
                })
                .unwrap_or_else(|| "unknown".to_string());
            (size, modified)
        }
        Err(_) => (0, "unknown".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_git_diff,
            get_git_refs,
            open_file_in_editor
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
