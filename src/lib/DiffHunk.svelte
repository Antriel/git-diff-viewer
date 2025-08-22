<script>
  import { invoke } from "@tauri-apps/api/core";
  import { parseHunkHeader, applySyntaxHighlighting } from "./utils.js";
  import FileStats from "./FileStats.svelte";
  import CodeLine from "./CodeLine.svelte";
  import Mark from "mark.js";
  import { tick } from "svelte";

  let {
    hunk,
    searchTerm = "",
    currentDirectory = "",
    hunkIndex = 0,
  } = $props();

  let baseLines = $state([]);
  let renderedLines = $state([]);
  let codeElement;
  let markInstance;

  async function openFileInEditor() {
    try {
      const { newStart } = parseHunkHeader(hunk.hunk_header);

      await invoke("open_file_in_editor", {
        filePath: hunk.file_name,
        workingDirectory: currentDirectory,
        lineNumber: newStart,
      });
    } catch (error) {
      console.error("Failed to open file in editor:", error);
      alert(`Failed to open file: ${error}`);
    }
  }

  function buildBaseLines() {
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

      let oldNum = " ";
      let newNum = " ";

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

      // Apply syntax highlighting once
      const syntaxHighlighted = applySyntaxHighlighting(
        content,
        hunk.file_name
      );

      result.push({
        oldNum,
        newNum,
        rawContent: line,
        syntaxHighlighted,
        cssClasses,
        isAdded,
        isRemoved,
        isContext,
      });
    }

    baseLines = result;
  }

  async function applyMarkJsHighlighting() {
    if (!codeElement) return;

    // Find all line-content elements within added/removed lines
    const lineContents = codeElement.querySelectorAll(
      ".code-line.added .line-content, .code-line.removed .line-content"
    );

    if (lineContents.length === 0) return;

    // Clean up previous mark instance if it exists
    if (markInstance) {
      await new Promise((resolve) => {
        markInstance.unmark({ done: resolve });
      });
    }

    // Create fresh mark instance and apply highlighting
    markInstance = new Mark(lineContents);

    if (searchTerm) {
      markInstance.mark(searchTerm, {
        element: "mark",
        className: "highlight",
        acrossElements: true,
        separateWordSearch: false,
      });
    }
  }

  function applySearchHighlighting() {
    // Check for matches and set isMatch appropriately
    renderedLines = baseLines.map((baseLine) => {
      const isMatch =
        searchTerm &&
        (baseLine.isAdded || baseLine.isRemoved) &&
        baseLine.rawContent.toLowerCase().includes(searchTerm.toLowerCase());

      return {
        oldNum: baseLine.oldNum,
        newNum: baseLine.newNum,
        content: baseLine.syntaxHighlighted,
        cssClasses: baseLine.cssClasses,
        isMatch,
      };
    });
  }

  // Build base lines when hunk changes
  $effect(() => {
    buildBaseLines();
  });

  // Apply search highlighting when search term or base lines change
  $effect(() => {
    if (baseLines.length > 0) {
      applySearchHighlighting();
    }
  });

  // Apply mark.js highlighting after DOM updates
  $effect(() => {
    if (renderedLines.length > 0 && codeElement) {
      tick().then(() => {
        applyMarkJsHighlighting();
      });
    }
  });
</script>

<div class="hunk">
  <div class="hunk-header">
    <div class="file-info">
      <strong class="file-name">{hunk.file_name}</strong>
      <span class="hunk-location">{hunk.hunk_header}</span>
      <button
        class="open-file-btn"
        onclick={openFileInEditor}
        title="Open file in editor">📂</button
      >
    </div>
    <FileStats stats={hunk.stats} />
  </div>

  <div class="code-container">
    <pre class="code"><code bind:this={codeElement}
        >{#each renderedLines as line, i}<CodeLine
            oldNum={line.oldNum}
            newNum={line.newNum}
            content={line.content}
            cssClasses={line.cssClasses}
            isMatch={line.isMatch}
          />{/each}</code
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
    border: 1px solid var(--border-light);
    margin: 1rem 0;
    border-radius: 8px;
    overflow: hidden;
    background: var(--component-bg);
  }

  .hunk-header {
    background: var(--header-bg);
    padding: 0.75rem;
    border-bottom: 1px solid var(--border-light);
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
    color: var(--text-color);
    word-break: break-all;
  }

  .hunk-location {
    font-family: monospace;
    color: var(--text-muted);
    background: var(--component-bg-hover);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
  }

  .open-file-btn {
    background: none;
    border: 1px solid var(--border-light);
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    color: var(--text-muted);
    transition: all 0.2s ease;
    margin-left: 0.5rem;
  }

  .open-file-btn:hover {
    background: var(--component-bg-hover);
    border-color: var(--border-medium);
    color: var(--text-color);
  }

  .open-file-btn:active {
    background: var(--component-bg-hover);
    opacity: 0.8;
  }

  .code-container {
    overflow-x: auto;
    overflow-y: hidden;
  }

  .code {
    margin: 0;
    padding: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas,
      "Liberation Mono", monospace;
  }

  :global(.highlight) {
    background: #fff3cd;
    color: #856404;
    font-weight: bold;
    padding: 0 2px;
    border-radius: 2px;
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
  }
</style>
