<script>
  import { createEventDispatcher, onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import SearchBar from "./SearchBar.svelte";

  let {
    gitDiffResult = null,
    contextSize = 3,
    searchTerm = "",
    visibleCount = 0,
    totalCount = 0,
    includeUntracked = false,
    currentDirectory = "",
  } = $props();

  /** @type {any} */
  let gitRefs = $state(null);
  let comparisonSource = $state("working"); // "working" or "staged"
  let comparisonTarget = $state("HEAD"); // HEAD, branch name, or commit hash

  const dispatch = createEventDispatcher();

  function handleSearch(event) {
    dispatch("search", event.detail);
  }

  function handleRefresh() {
    dispatch("refresh");
  }

  function adjustContext(delta) {
    const newContextSize = Math.max(0, contextSize + delta);
    if (newContextSize !== contextSize) {
      dispatch("contextChange", { contextLines: newContextSize });
    }
  }

  function handleUntrackedToggle() {
    dispatch("untrackedToggle", { includeUntracked });
  }

  function getDisplayName(value) {
    if (value === "working") return "Working Directory";
    if (value === "staged") return "Staged Files";
    if (value === "HEAD") return "HEAD";

    // Check if it's a commit hash
    if (gitRefs?.recent_commits) {
      const commit = gitRefs.recent_commits.find((c) => c.short_hash === value);
      if (commit) return commit.name;
    }

    // Otherwise it's probably a branch name
    return value;
  }

  async function loadGitRefs() {
    if (!currentDirectory) return;
    try {
      gitRefs = await invoke("get_git_refs", {
        directoryPath: currentDirectory,
      });
      console.log(gitRefs);
    } catch (error) {
      console.error("Failed to load git refs:", error);
    }
  }

  function handleComparisonChange() {
    dispatch("comparisonChange", {
      source: comparisonSource,
      target: comparisonTarget,
    });
  }

  function resetComparison() {
    comparisonSource = "working";
    comparisonTarget = "HEAD";
    handleComparisonChange();
  }

  onMount(() => {
    if (currentDirectory) {
      loadGitRefs();
    }
  });

  // Watch for changes to currentDirectory and reload git refs
  $effect(() => {
    if (currentDirectory) {
      loadGitRefs();
    }
  });
</script>

<div class="diff-header">
  <div class="stats">
    {#if gitDiffResult}
      <strong>{gitDiffResult.total_stats.files} files</strong> changed,
      <span class="added">+{gitDiffResult.total_stats.added}</span>,
      <span class="removed">-{gitDiffResult.total_stats.removed}</span>
    {:else}
      <span class="placeholder">No diff data loaded</span>
    {/if}
  </div>

  <div class="toolbar">
    <SearchBar
      {searchTerm}
      resultsCount={visibleCount}
      {totalCount}
      on:search={handleSearch}
    />
    <div class="untracked-toggle">
      <label>
        <input
          type="checkbox"
          bind:checked={includeUntracked}
          onchange={handleUntrackedToggle}
        />
        Include untracked files
      </label>
    </div>
    <div class="context-controls">
      <span>Context:</span>
      <button onclick={() => adjustContext(-1)} disabled={contextSize === 0}
        >-</button
      >
      <span class="context-size">{contextSize}</span>
      <button onclick={() => adjustContext(1)}>+</button>
    </div>
    <button onclick={handleRefresh} class="refresh-btn" title="Refresh">
      🔄
    </button>
  </div>

  <div class="comparison-controls">
    <div class="comparison-row">
      <div class="comparison-field">
        <label for="source-select">Compare from:</label>
        <select
          id="source-select"
          bind:value={comparisonSource}
          onchange={handleComparisonChange}
        >
          <option value="working">Working Directory</option>
          <option value="staged">Staged Files</option>
          {#if gitRefs?.branches}
            <optgroup label="Branches">
              {#each gitRefs.branches as branch}
                <option value={branch.name}>{branch.name}</option>
              {/each}
            </optgroup>
          {/if}
          {#if gitRefs?.recent_commits}
            <optgroup label="Recent Commits">
              {#each gitRefs.recent_commits as commit}
                <option value={commit.short_hash}>{commit.name}</option>
              {/each}
            </optgroup>
          {/if}
        </select>
      </div>

      <div class="comparison-field">
        <label for="target-select">Compare to:</label>
        <select
          id="target-select"
          bind:value={comparisonTarget}
          onchange={handleComparisonChange}
        >
          <option value="HEAD">HEAD</option>
          {#if gitRefs?.branches}
            <optgroup label="Branches">
              {#each gitRefs.branches as branch}
                <option value={branch.name}>{branch.name}</option>
              {/each}
            </optgroup>
          {/if}
          {#if gitRefs?.recent_commits}
            <optgroup label="Recent Commits">
              {#each gitRefs.recent_commits as commit}
                <option value={commit.short_hash}>{commit.name}</option>
              {/each}
            </optgroup>
          {/if}
        </select>
      </div>

      <button
        onclick={resetComparison}
        class="reset-comparison-btn"
        title="Reset to default"
      >
        ↺
      </button>
    </div>

    <div class="comparison-info">
      <small>
        Comparing:
        <strong>{getDisplayName(comparisonSource)}</strong>
        vs
        <strong>{getDisplayName(comparisonTarget)}</strong>
      </small>
    </div>
  </div>
</div>

<style>
  .diff-header {
    background: #f8f9fa;
    padding: 1rem;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    margin-bottom: 0;
  }

  .stats {
    margin-bottom: 1rem;
    font-size: 0.9rem;
  }

  .stats .added {
    color: #28a745;
    font-weight: bold;
  }

  .stats .removed {
    color: #dc3545;
    font-weight: bold;
  }

  .stats .placeholder {
    color: #6c757d;
    font-style: italic;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .refresh-btn {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 6px;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 1rem;
    transition: all 0.2s;
  }

  .refresh-btn:hover {
    background: #e9ecef;
  }

  .untracked-toggle {
    display: flex;
    align-items: center;
    font-size: 0.85rem;
  }

  .untracked-toggle label {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    cursor: pointer;
    user-select: none;
  }

  .untracked-toggle input[type="checkbox"] {
    cursor: pointer;
  }

  .context-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .context-controls button {
    background: #fff;
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    cursor: pointer;
    font-size: 0.8rem;
    transition: all 0.2s;
  }

  .context-controls button:hover:not(:disabled) {
    background: #f0f0f0;
  }

  .context-controls button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .context-size {
    min-width: 1.5rem;
    text-align: center;
    font-weight: bold;
  }

  .comparison-controls {
    margin-top: 1rem;
    padding-top: 1rem;
    border-top: 1px solid #dee2e6;
  }

  .comparison-row {
    display: flex;
    align-items: end;
    gap: 1rem;
    flex-wrap: wrap;
    margin-bottom: 0.5rem;
  }

  .comparison-field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    min-width: 200px;
  }

  .comparison-field label {
    font-size: 0.8rem;
    font-weight: bold;
    color: #495057;
  }

  .comparison-field select {
    padding: 0.4rem;
    border: 1px solid #ced4da;
    border-radius: 4px;
    font-size: 0.85rem;
    background: #fff;
  }

  .reset-comparison-btn {
    background: #6c757d;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 0.4rem 0.8rem;
    cursor: pointer;
    font-size: 0.85rem;
    height: fit-content;
    transition: all 0.2s;
  }

  .reset-comparison-btn:hover {
    background: #5a6268;
  }

  .comparison-info {
    color: #6c757d;
    font-size: 0.8rem;
  }

  .comparison-info strong {
    color: #495057;
  }

  @media (prefers-color-scheme: dark) {
    .diff-header {
      background: #333;
      border-color: #555;
    }

    .stats {
      color: #f6f6f6;
    }

    .stats .placeholder {
      color: #adb5bd;
    }

    .refresh-btn {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .refresh-btn:hover {
      background: #555;
    }

    .untracked-toggle {
      color: #f6f6f6;
    }

    .context-controls {
      color: #f6f6f6;
    }

    .context-controls button {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .context-controls button:hover:not(:disabled) {
      background: #555;
    }

    .comparison-controls {
      border-color: #555;
    }

    .comparison-field label {
      color: #adb5bd;
    }

    .comparison-field select {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .comparison-info {
      color: #adb5bd;
    }

    .comparison-info strong {
      color: #f6f6f6;
    }
  }
</style>
