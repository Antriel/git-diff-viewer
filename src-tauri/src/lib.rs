use chrono::DateTime;
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

async fn check_editor_available(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

async fn find_available_editor() -> Result<&'static str, String> {
    // Try VSCode variants first
    let code_variants = if cfg!(target_os = "windows") {
        vec!["code.cmd", "code.exe", "code"]
    } else {
        vec!["code"]
    };

    for variant in code_variants {
        if check_editor_available(variant, &["--version"]).await {
            return Ok(variant);
        }
    }

    // Try cmd approach on Windows
    if cfg!(target_os = "windows")
        && check_editor_available("cmd", &["/c", "code", "--version"]).await
    {
        return Ok("cmd-code");
    }

    // Try other editors
    if check_editor_available("subl", &["--version"]).await {
        return Ok("subl");
    }

    if check_editor_available("atom", &["--version"]).await {
        return Ok("atom");
    }

    if cfg!(target_os = "windows") {
        if check_editor_available("notepad++", &["--version"]).await {
            return Ok("notepad++");
        }
        if check_editor_available("notepad", &["/?"]).await {
            return Ok("notepad");
        }
    }

    Err("No supported editor found. Please install VS Code, Sublime Text, or another supported editor.".to_string())
}

fn build_editor_command(editor: &str, file_path: &str, line_number: Option<u32>) -> Command {
    let mut command = if editor == "cmd-code" {
        let mut c = Command::new("cmd");
        c.arg("/c").arg("code").arg("-r");
        c
    } else {
        Command::new(editor)
    };

    match editor {
        "code" | "code.cmd" | "code.exe" | "cmd-code" => {
            if editor != "cmd-code" {
                command.arg("-r");
            }
            if let Some(line) = line_number {
                command.arg("-g").arg(format!("{}:{}", file_path, line));
            } else {
                command.arg(file_path);
            }
        }
        "subl" | "atom" => {
            if let Some(line) = line_number {
                command.arg(format!("{}:{}", file_path, line));
            } else {
                command.arg(file_path);
            }
        }
        "notepad++" => {
            if let Some(line) = line_number {
                command.arg("-n").arg(line.to_string()).arg(file_path);
            } else {
                command.arg(file_path);
            }
        }
        _ => {
            command.arg(file_path);
        }
    }

    command
}

async fn run_git_command(args: &[&str], directory: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to execute git command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Git command failed: {}", stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

async fn check_git_repo(directory: &str) -> Result<(), String> {
    run_git_command(&["rev-parse", "--git-dir"], directory)
        .await
        .map_err(|_| "Not a git repository or git not found".to_string())
        .map(|_| ())
}

async fn run_git_command_optional(args: &[&str], directory: &str) -> Option<String> {
    run_git_command(args, directory).await.ok()
}

#[command]
async fn open_file_in_editor(
    file_path: String,
    working_directory: String,
    line_number: Option<u32>,
) -> Result<(), String> {
    // Normalize and construct absolute path
    let absolute_path = if Path::new(&file_path).is_absolute() {
        file_path
    } else {
        Path::new(&working_directory)
            .join(&file_path)
            .to_str()
            .ok_or("Invalid path encoding")?
            .to_string()
    };

    let editor = find_available_editor().await?;
    let mut command = build_editor_command(editor, &absolute_path, line_number);

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
    check_git_repo(&directory_path).await?;

    let mut git_refs = GitRefs {
        branches: Vec::new(),
        recent_commits: Vec::new(),
    };

    // Get branches
    if let Ok(branch_text) = run_git_command(
        &["branch", "-a", "--format=%(refname:short)"],
        &directory_path,
    )
    .await
    {
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
    if let Ok(commit_text) = run_git_command(
        &["log", "--oneline", "-20", "--pretty=format:%h|%s"],
        &directory_path,
    )
    .await
    {
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
    check_git_repo(&directory_path).await?;

    // Get git diff with source/target model
    let context_arg = format!("-U{}", context_lines.unwrap_or(3));

    // Determine what to compare based on source and target
    let source = comparison_source.as_deref().unwrap_or("working");
    let target = comparison_target.as_deref().unwrap_or("HEAD");

    // Prepare owned strings for ranges to avoid borrowing issues
    let range = match source {
        "staged" | "working" => String::new(),
        _ => {
            if source == target {
                format!("{}..HEAD", source)
            } else {
                format!("{}..{}", source, target)
            }
        }
    };

    let diff_args = match source {
        "staged" => {
            // Compare staged files against target
            if target != "HEAD" {
                vec!["diff", &context_arg, "--staged", target]
            } else {
                vec!["diff", &context_arg, "--staged"]
            }
        }
        "working" => {
            // Compare working directory against target
            vec!["diff", &context_arg, target]
        }
        _ => {
            // Source is a commit/branch, compare it against target (commit-to-commit)
            vec!["diff", &context_arg, &range]
        }
    };

    let mut all_diff_text = run_git_command(&diff_args, &directory_path).await?;

    // Handle untracked files if requested
    if include_untracked.unwrap_or(false) {
        if let Some(untracked_files) = run_git_command_optional(
            &["ls-files", "--others", "--exclude-standard"],
            &directory_path,
        )
        .await
        {
            for untracked_file in untracked_files.lines() {
                if !untracked_file.trim().is_empty() {
                    let null_device = if cfg!(target_os = "windows") {
                        "NUL"
                    } else {
                        "/dev/null"
                    };

                    // git diff --no-index returns exit code 1 when files differ, which is expected
                    // So we need to handle this manually instead of using run_git_command_optional
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
                        .await;

                    if let Ok(output) = untracked_diff {
                        let untracked_diff_text = String::from_utf8_lossy(&output.stdout);
                        if !untracked_diff_text.trim().is_empty() {
                            if !all_diff_text.trim().is_empty() {
                                all_diff_text.push('\n');
                            }
                            all_diff_text.push_str(&untracked_diff_text);
                        }
                    }
                }
            }
        }
    }

    if all_diff_text.trim().is_empty() && source == "working" && target == "HEAD" {
        // Only check for staged changes when doing default comparison (working vs HEAD)
        if let Ok(staged_text) =
            run_git_command(&["diff", &context_arg, "--cached"], &directory_path).await
        {
            if !staged_text.trim().is_empty() {
                return Ok(parse_diff_to_hunks(&staged_text, &directory_path));
            }
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
                .and_then(|time| {
                    DateTime::from_timestamp(
                        time.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                        0,
                    )
                })
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "unknown".to_string());
            (size, modified)
        }
        Err(_) => (0, "unknown".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
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
