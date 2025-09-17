<script>
  import HunkDisplay from "./HunkDisplay.svelte";
  import FileStats from "./FileStats.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { parseHunkHeader } from "./utils.js";

  let {
    fileName,
    hunks = [],
    searchTerm = "",
    searchMode = "both",
    currentDirectory = "",
  } = $props();

  let firstHunk = $derived(hunks[0]);
  let firstChangeLineNumber = 0;

  // Calculate total file stats from all hunks
  let totalStats = $derived.by(() => {
    if (!hunks.length) return { added: 0, removed: 0, size: 0, modified: "" };

    const totalAdded = hunks.reduce((sum, hunk) => sum + hunk.stats.added, 0);
    const totalRemoved = hunks.reduce((sum, hunk) => sum + hunk.stats.removed, 0);

    // Use stats from first hunk for size and modified (they should be the same for all hunks of the same file)
    return {
      added: totalAdded,
      removed: totalRemoved,
      size: firstHunk?.stats.size || 0,
      modified: firstHunk?.stats.modified || "",
    };
  });

  async function openFileInEditor() {
    try {
      // Calculate the first change line number from the first hunk
      const firstHunk = hunks[0];
      if (firstHunk) {
        // This logic is duplicated from DiffHunk - we might want to extract it
        const lines = firstHunk.hunk_lines;
        const { newStart } = parseHunkHeader(firstHunk.hunk_header);

        let newLineNum = newStart;
        for (let i = 0; i < lines.length; i++) {
          const line = lines[i];
          const prefix = line[0];
          const isAdded = prefix === "+";
          const isRemoved = prefix === "-";

          if (firstChangeLineNumber === 0 && (isAdded || isRemoved)) {
            firstChangeLineNumber = newLineNum;
            break;
          }

          if (prefix === " " || isAdded) {
            newLineNum++;
          }
        }
      }

      await invoke("open_file_in_editor", {
        filePath: fileName,
        workingDirectory: currentDirectory,
        lineNumber: firstChangeLineNumber || (firstHunk ? parseHunkHeader(firstHunk.hunk_header).newStart : 1),
      });
    } catch (error) {
      console.error("Failed to open file in editor:", error);
      alert(`Failed to open file: ${error}`);
    }
  }
</script>

<div class="file-group">
  <div class="file-header">
    <div class="file-info">
      <strong class="file-name">{fileName}</strong>
      <span class="hunk-count">{hunks.length} hunk{hunks.length !== 1 ? 's' : ''}</span>
      <button
        class="open-file-btn"
        onclick={openFileInEditor}
        title="Open file in editor">📂</button>
    </div>
    <FileStats stats={totalStats} />
  </div>

  <div class="hunks-container">
    {#each hunks as hunk, index (hunk.hunk_id)}
      <HunkDisplay
        {hunk}
        {searchTerm}
        {searchMode}
        {currentDirectory}
        hunkIndex={index}
      />
    {/each}
  </div>
</div>

<style>
  .file-group {
    border: 1px solid var(--border-light);
    margin: 1.5rem 0;
    border-radius: 8px;
    overflow: hidden;
    background: var(--component-bg);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .file-header {
    background: var(--header-bg);
    padding: 0.6rem 0.75rem;
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
    font-size: 1rem;
    font-weight: 600;
  }

  .hunk-count {
    font-family: monospace;
    color: var(--text-muted);
    background: var(--component-bg-hover);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-size: 0.85rem;
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


  @media (max-width: 768px) {
    .file-header {
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