# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Git Diff Viewer is a Tauri-based desktop application built with SvelteKit that provides a GUI for viewing and searching git diff results. The application allows users to compare different git states (working directory vs HEAD, between commits, etc.), search within diffs, and navigate directly to files in their editor.

## Commands

### Development
- `npm run check` - Run Svelte type checking

## Architecture

### Frontend (SvelteKit + Svelte 5)
- **Framework**: SvelteKit with static adapter for SPA mode (required for Tauri)
- **UI Library**: Custom components using CSS custom properties for theming
- **State Management**: Svelte 5 runes (`$state`, `$effect`) for reactive state
- **Styling**: Component-scoped CSS with global theme variables

### Backend (Tauri + Rust)
- **Main Backend**: `src-tauri/src/lib.rs` contains all Tauri commands
- **Git Operations**: Rust backend handles all git commands via `tokio::process::Command`
- **Editor Integration**: Automatic detection and launching of VS Code, Sublime Text, Atom, Notepad++, etc.

### Key Components
- **Main App**: `src/routes/+page.svelte` - Root component with application state
- **DiffViewer**: `src/lib/DiffViewer.svelte` - Core diff display component
- **ComparisonControls**: `src/lib/ComparisonControls.svelte` - Git ref selection and comparison options
- **SearchBar**: `src/lib/SearchBar.svelte` - Search functionality with highlighting
- **DirectorySelector**: `src/lib/DirectorySelector.svelte` - Git repository selection

### Data Flow
1. User selects a git repository via DirectorySelector
2. Frontend calls `get_git_diff` Tauri command with comparison parameters
3. Rust backend executes git commands and parses diff output
4. Structured diff data returned to frontend as `GitDiffResult`
5. DiffViewer renders the diff with syntax highlighting (highlight.js)
6. Search functionality uses mark.js for text highlighting

### Tauri Commands
- `get_git_diff(directory_path, context_lines, include_untracked, comparison_source, comparison_target)` - Main diff generation
- `get_git_refs(directory_path)` - Fetch available branches and recent commits
- `open_file_in_editor(file_path, working_directory, line_number)` - Launch editor at specific file/line

### Storage & Settings
- LocalStorage used for user preferences (theme, search mode, context size, recent projects)
- Window state persistence via `tauri-plugin-window-state`

### Theme System
- Supports light/dark/auto themes with CSS custom properties
- Dynamic highlight.js theme switching
- Smooth transitions between themes

## Development Notes

### Frontend Technologies
- **Svelte 5**: Uses new runes syntax (`$state`, `$effect`)
- **SvelteKit**: Configured as SPA with static adapter
- **highlight.js**: Syntax highlighting for diff content
- **mark.js**: Search term highlighting

### Backend Dependencies
- **chrono**: Date/time handling for file stats
- **regex**: Git diff parsing
- **tokio**: Async process execution
- **serde**: JSON serialization for Tauri commands

### Cross-platform Considerations
- Windows-specific command creation (hidden console windows)
- Path handling for different file systems
- Editor detection varies by platform (Windows vs Unix)

### Testing & Quality
- TypeScript checking via `svelte-check`
- No specific test framework configured - add tests as needed
