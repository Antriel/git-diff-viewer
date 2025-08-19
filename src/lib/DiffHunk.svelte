<script>
  import hljs from "highlight.js";
  import { invoke } from "@tauri-apps/api/core";

  let { hunk, searchTerm = "", currentDirectory = "", hunkIndex = 0 } = $props();

  let renderedLines = $state([]);

  function escapeHtml(str) {
    return str
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }

  function parseHunkHeader(header) {
    const match = header.match(/@@ -(\d+)(?:,\d+)? \+(\d+)(?:,\d+)? @@/);
    return match
      ? {
          oldStart: parseInt(match[1], 10),
          newStart: parseInt(match[2], 10),
        }
      : { oldStart: 1, newStart: 1 };
  }

  async function openFileInEditor() {
    try {
      const { newStart } = parseHunkHeader(hunk.hunk_header);
      
      await invoke("open_file_in_editor", {
        filePath: hunk.file_name,
        workingDirectory: currentDirectory,
        lineNumber: newStart
      });
    } catch (error) {
      console.error("Failed to open file in editor:", error);
      alert(`Failed to open file: ${error}`);
    }
  }

  function applySyntaxHighlighting(content, fileName) {
    // Get file extension for language detection
    const ext = fileName.split(".").pop()?.toLowerCase();
    const language = hljs.getLanguage(ext);

    try {
      if (language) {
        const result = hljs.highlight(content, { language: ext });
        return result.value;
      } else {
        // Try auto-detection if specific language not found
        const result = hljs.highlightAuto(content);
        return result.value;
      }
    } catch (error) {
      console.warn("Syntax highlighting failed:", error);
      return escapeHtml(content);
    }
  }

  function formatRelativeTime(isoString) {
    if (isoString === "unknown") return "unknown";
    
    const now = new Date();
    const modifiedDate = new Date(isoString);
    const diffMs = now.getTime() - modifiedDate.getTime();
    
    const rtf = new Intl.RelativeTimeFormat('en', { numeric: 'auto' });
    
    const diffSeconds = Math.floor(diffMs / 1000);
    const diffMinutes = Math.floor(diffSeconds / 60);
    const diffHours = Math.floor(diffMinutes / 60);
    const diffDays = Math.floor(diffHours / 24);
    const diffWeeks = Math.floor(diffDays / 7);
    const diffMonths = Math.floor(diffDays / 30);
    const diffYears = Math.floor(diffDays / 365);
    
    if (Math.abs(diffSeconds) < 60) return rtf.format(0, 'second');
    if (Math.abs(diffMinutes) < 60) return rtf.format(-diffMinutes, 'minute');
    if (Math.abs(diffHours) < 24) return rtf.format(-diffHours, 'hour');
    if (Math.abs(diffDays) < 7) return rtf.format(-diffDays, 'day');
    if (Math.abs(diffWeeks) < 4) return rtf.format(-diffWeeks, 'week');
    if (Math.abs(diffMonths) < 12) return rtf.format(-diffMonths, 'month');
    return rtf.format(-diffYears, 'year');
  }

  function formatLocalTime(isoString) {
    if (isoString === "unknown") return "Unknown";
    const date = new Date(isoString);
    return date.toLocaleString();
  }

  function renderHunk() {
    const lines = hunk.hunk_lines;
    const { oldStart, newStart } = parseHunkHeader(hunk.hunk_header);
    const result = [];

    let oldLineNum = oldStart;
    let newLineNum = newStart;

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const prefix = line[0];
      const content = line.slice(1);

      const isAdded = prefix === "+";
      const isRemoved = prefix === "-";
      const isContext = prefix === " ";

      let oldNum = ' ';
      let newNum = ' ';

      if (isContext) {
        oldNum = String(oldLineNum++);
        newNum = String(newLineNum++);
      } else if (isAdded) {
        newNum = String(newLineNum++);
      } else if (isRemoved) {
        oldNum = String(oldLineNum++);
      }

      const cssClasses = [];
      if (isContext) cssClasses.push("context");
      if (isAdded) cssClasses.push("added");
      if (isRemoved) cssClasses.push("removed");

      // Apply syntax highlighting first
      let highlightedContent = applySyntaxHighlighting(content, hunk.file_name);

      // Then apply search term highlighting if needed
      if ((isAdded || isRemoved) && searchTerm) {
        const escapedTerm = escapeHtml(searchTerm);
        const regex = new RegExp(
          `(${escapedTerm.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")})`,
          "gi"
        );
        highlightedContent = highlightedContent.replace(
          regex,
          '<mark class="highlight">$1</mark>'
        );
      }

      // Mark as match if line contains search term
      const isMatch =
        searchTerm &&
        (isAdded || isRemoved) &&
        line.toLowerCase().includes(searchTerm.toLowerCase());

      result.push({
        oldNum,
        newNum,
        content: highlightedContent,
        cssClasses,
        isMatch,
      });
    }

    renderedLines = result;
  }

  // Re-render when hunk or search term changes
  $effect(() => {
    renderHunk();
  });
</script>

<div class="hunk">
  <div class="hunk-header">
    <div class="file-info">
      <strong class="file-name">{hunk.file_name}</strong>
      <span class="hunk-location">{hunk.hunk_header}</span>
      <button class="open-file-btn" onclick={openFileInEditor} title="Open file in editor">📂</button>
    </div>
    <div class="file-stats">
      <span class="added">+{hunk.stats.added}</span>
      <span class="removed">-{hunk.stats.removed}</span>
      <span class="size">{(hunk.stats.size / 1024).toFixed(1)}KB</span>
      <span class="modified" title={formatLocalTime(hunk.stats.modified)}>{formatRelativeTime(hunk.stats.modified)}</span>
    </div>
  </div>

  <div class="code-container">
    <pre class="code"><code
        >{#each renderedLines as line}<div
            class="code-line {line.cssClasses.join(' ')}"
            class:match={line.isMatch}><span class="line-num old"
              >{line.oldNum}</span
            ><span class="line-num new">{line.newNum}</span><span
              class="line-content">{@html line.content}</span
            ></div>{/each}</code
      ></pre>
  </div>
</div>

<style>
  /* Import highlight.js themes using CSS @import for better control */
  @import "highlight.js/styles/github.css" screen and
    (prefers-color-scheme: light);
  @import "highlight.js/styles/github-dark.css" screen and
    (prefers-color-scheme: dark);
  .hunk {
    border: 1px solid #ddd;
    margin: 1rem 0;
    border-radius: 8px;
    overflow: hidden;
    background: white;
  }

  .hunk-header {
    background: #fafafa;
    padding: 0.75rem;
    border-bottom: 1px solid #eee;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .file-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    flex: 1;
  }

  .file-name {
    color: #333;
    word-break: break-all;
  }

  .hunk-location {
    font-family: monospace;
    font-size: 0.85rem;
    color: #666;
    background: #f0f0f0;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
  }

  .open-file-btn {
    background: none;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    font-size: 0.85rem;
    color: #666;
    transition: all 0.2s ease;
    margin-left: 0.5rem;
  }

  .open-file-btn:hover {
    background: #f0f0f0;
    border-color: #999;
    color: #333;
  }

  .open-file-btn:active {
    background: #e0e0e0;
  }

  .file-stats {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
    white-space: nowrap;
  }

  .file-stats .added {
    color: #28a745;
    font-weight: bold;
  }

  .file-stats .removed {
    color: #dc3545;
    font-weight: bold;
  }

  .file-stats .size {
    color: #666;
  }

  .file-stats .modified {
    color: #666;
    cursor: help;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: #f8f9fa;
    border-bottom: 1px solid #eee;
    font-size: 0.85rem;
  }

  .code-container {
    overflow-x: auto;
    overflow-y: hidden;
    /* Removed max-height and disabled vertical scrolling to allow hunks to expand naturally */
  }

  .code {
    margin: 0;
    padding: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas,
      "Liberation Mono", monospace;
    font-size: 0.85rem;
    line-height: 1.4;
  }

  .code-line {
    display: flex;
    align-items: center;
    line-height: 1;
  }

  .line-num {
    flex: 0 0 3em;
    text-align: right;
    padding: 0 0.5em;
    color: #999;
    user-select: none;
    background: #f8f9fa;
    border-right: 1px solid #eee;
  }

  .line-num.new {
    border-left: 1px solid #eee;
  }

  .line-content {
    flex: 1;
    padding: 0 0.75em;
    white-space: pre;
    word-break: break-all;
  }

  .code-line.context {
    background: #fff;
  }

  .code-line.added {
    background: #e6ffed;
  }

  .code-line.removed {
    background: #ffeef0;
  }

  .code-line.match {
    box-shadow: inset 3px 0 0 #ffd700;
  }

  :global(.highlight) {
    background: #fff3cd;
    color: #856404;
    font-weight: bold;
    padding: 0 2px;
    border-radius: 2px;
  }

  @media (prefers-color-scheme: dark) {
    .hunk {
      background: #2a2a2a;
      border-color: #555;
    }

    .hunk-header {
      background: #333;
      border-color: #555;
    }

    .file-name {
      color: #f6f6f6;
    }

    .hunk-location {
      background: #444;
      color: #ccc;
    }

    .open-file-btn {
      border-color: #555;
      color: #ccc;
    }

    .open-file-btn:hover {
      background: #444;
      border-color: #777;
      color: #f6f6f6;
    }

    .open-file-btn:active {
      background: #333;
    }

    .file-stats .size {
      color: #ccc;
    }

    .file-stats .modified {
      color: #ccc;
    }

    .controls {
      background: #333;
      border-color: #555;
      color: #f6f6f6;
    }

    .line-num {
      background: #333;
      border-color: #555;
      color: #999;
    }

    .code-line.context {
      background: #2a2a2a;
    }

    .code-line.added {
      background: #1f3a2e;
    }

    .code-line.removed {
      background: #3a1f1f;
    }

    :global(.highlight) {
      background: #4a4a00;
      color: #ffff88;
    }
  }

  @media (max-width: 768px) {
    .hunk-header {
      flex-direction: column;
      align-items: stretch;
    }

    .file-info {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }

    .file-stats {
      justify-content: space-between;
    }

    .line-num {
      flex: 0 0 2.5em;
    }

    .code {
      font-size: 0.8rem;
    }
  }
</style>
