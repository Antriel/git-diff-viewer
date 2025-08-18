<script>
  import hljs from "highlight.js";

  let { hunk, searchTerm = "", hunkIndex = 0 } = $props();

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

      let oldNum = 0;
      let newNum = 0;

      if (isContext) {
        oldNum = oldLineNum++;
        newNum = newLineNum++;
      } else if (isAdded) {
        newNum = newLineNum++;
      } else if (isRemoved) {
        oldNum = oldLineNum++;
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

  function copyHunk() {
    const content = hunk.hunk_lines.join("\n");
    navigator.clipboard
      .writeText(content)
      .then(() => {
        // Simple visual feedback could be added here
        console.log("Copied hunk to clipboard");
      })
      .catch((err) => {
        console.error("Failed to copy:", err);
      });
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
    </div>
    <div class="file-stats">
      <span class="added">+{hunk.stats.added}</span>
      <span class="removed">-{hunk.stats.removed}</span>
      <span class="size">{(hunk.stats.size / 1024).toFixed(1)}KB</span>
    </div>
  </div>

  <div class="controls">
    <button onclick={copyHunk} title="Copy hunk" class="copy-btn">📋</button>
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

  .controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background: #f8f9fa;
    border-bottom: 1px solid #eee;
    font-size: 0.85rem;
  }

  .controls button {
    background: #fff;
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    cursor: pointer;
    font-size: 0.8rem;
    transition: all 0.2s;
  }

  .controls button:hover:not(:disabled) {
    background: #f0f0f0;
  }

  .controls button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .copy-btn {
    margin-left: auto;
  }

  .code-container {
    overflow-x: auto;
    max-height: 500px;
    overflow-y: auto;
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

    .file-stats .size {
      color: #ccc;
    }

    .controls {
      background: #333;
      border-color: #555;
      color: #f6f6f6;
    }

    .controls button {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .controls button:hover:not(:disabled) {
      background: #555;
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
