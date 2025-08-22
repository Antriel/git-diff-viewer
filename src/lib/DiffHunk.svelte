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
    searchMode = "both",
    currentDirectory = "",
    hunkIndex = 0,
  } = $props();

  let baseLines = $state([]);
  let renderedLines = $state([]);
  let codeElement = $state();
  let markInstance;
  let selectedText = $state("");
  let hunkElement;
  let isVisible = $state(false);
  let observer;

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

  function handleTextSelection() {
    const selection = window.getSelection();
    const text = selection?.toString().trim();

    if (text && text.length > 0) {
      // If selecting the same text, deselect it
      if (selectedText === text) {
        selectedText = "";
        selection?.removeAllRanges();
      } else {
        selectedText = text;
      }
    } else {
      selectedText = "";
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

    // Clean up previous mark instance if it exists
    if (markInstance) {
      await new Promise((resolve) => {
        markInstance.unmark({ done: resolve });
      });
    }

    // Create fresh mark instance on the entire code element
    markInstance = new Mark(codeElement);

    const excludeSelectors = [".line-num"];

    // Apply search term highlighting (only on added/removed based on search mode)
    if (searchTerm) {
      let selector = "";
      if (searchMode === "added") {
        selector = ".code-line.added .line-content";
      } else if (searchMode === "removed") {
        selector = ".code-line.removed .line-content";
      } else {
        selector =
          ".code-line.added .line-content, .code-line.removed .line-content";
      }

      const lineContents = codeElement.querySelectorAll(selector);
      if (lineContents.length > 0) {
        const searchMarkInstance = new Mark(lineContents);
        searchMarkInstance.mark(searchTerm, {
          element: "mark",
          className: "highlight",
          acrossElements: true,
          separateWordSearch: false,
          exclude: excludeSelectors,
        });
      }
    }

    // Apply selected text highlighting (on all lines)
    if (selectedText) {
      markInstance.mark(selectedText, {
        element: "mark",
        className: "word-highlight",
        acrossElements: true,
        separateWordSearch: false,
        exclude: excludeSelectors,
      });
    }
  }

  function applySearchHighlighting() {
    // Check for matches and set isMatch appropriately based on search mode
    renderedLines = baseLines.map((baseLine) => {
      let shouldHighlight = false;

      if (searchTerm) {
        if (searchMode === "added" && baseLine.isAdded) {
          shouldHighlight = true;
        } else if (searchMode === "removed" && baseLine.isRemoved) {
          shouldHighlight = true;
        } else if (
          searchMode === "both" &&
          (baseLine.isAdded || baseLine.isRemoved)
        ) {
          shouldHighlight = true;
        }
      }

      const isMatch =
        shouldHighlight &&
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

  // Set up intersection observer for lazy loading
  $effect(() => {
    if (hunkElement && !observer) {
      observer = new IntersectionObserver(
        (entries) => {
          const entry = entries[0];
          if (entry.isIntersecting && !isVisible) {
            isVisible = true;
            // Trigger highlighting when becoming visible
            buildBaseLines();
          }
        },
        { threshold: 0.1, rootMargin: "100px" }
      );
      observer.observe(hunkElement);
    }

    return () => {
      if (observer) {
        observer.disconnect();
        observer = null;
      }
    };
  });

  // Build base lines when hunk changes (only if visible)
  $effect(() => {
    if (isVisible) {
      buildBaseLines();
    }
  });

  // Apply search highlighting when search term or base lines change (only if visible)
  $effect(() => {
    if (isVisible && baseLines.length > 0) {
      applySearchHighlighting();
    }
  });

  // Apply mark.js highlighting after DOM updates (only if visible)
  $effect(() => {
    if (isVisible && renderedLines.length > 0 && codeElement) {
      tick().then(() => {
        applyMarkJsHighlighting();
      });
    }
  });

  // Apply highlighting when selected text changes (only if visible)
  $effect(() => {
    selectedText; // Explicitly depend on selectedText
    if (isVisible && codeElement) {
      applyMarkJsHighlighting();
    }
  });
</script>

<div class="hunk" bind:this={hunkElement}>
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
    {#if isVisible}
      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <pre class="code"><code
          bind:this={codeElement}
          onmouseup={handleTextSelection}
          >{#each renderedLines as line, i}<CodeLine
              oldNum={line.oldNum}
              newNum={line.newNum}
              content={line.content}
              cssClasses={line.cssClasses}
              isMatch={line.isMatch}
            />{/each}</code
        ></pre>
    {:else}
      <div class="placeholder">Loading...</div>
    {/if}
  </div>
</div>

<style>
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

  .placeholder {
    padding: 2rem;
    color: var(--text-muted);
    text-align: center;
    font-style: italic;
  }

  .code {
    margin: 0;
    padding: 0;
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas,
      "Liberation Mono", monospace;
  }

  :global(.highlight) {
    background: var(--highlight-bg);
    color: var(--highlight-text);
    font-weight: bold;
    padding: 0 2px;
    border-radius: 2px;
  }

  :global(.word-highlight) {
    background: var(--word-highlight-bg);
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
