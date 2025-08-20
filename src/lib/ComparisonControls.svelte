<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let {
    currentDirectory = "",
    comparisonSource = $bindable("working"),
    comparisonTarget = $bindable("HEAD"),
  } = $props();

  /** @type {any} */
  let gitRefs = $state(null);
  let isExpanded = $state(false);

  function getDisplayName(value) {
    if (value === "working") return "Working Directory";
    if (value === "staged") return "Staged Files";
    if (value === "HEAD") return "HEAD";

    if (gitRefs?.recent_commits) {
      const commit = gitRefs.recent_commits.find((c) => c.short_hash === value);
      if (commit) return commit.name;
    }

    return value;
  }

  async function loadGitRefs() {
    if (!currentDirectory) return;
    try {
      gitRefs = await invoke("get_git_refs", {
        directoryPath: currentDirectory,
      });
    } catch (error) {
      console.error("Failed to load git refs:", error);
    }
  }

  function resetComparison() {
    comparisonSource = "working";
    comparisonTarget = "HEAD";
  }

  function toggleExpanded() {
    isExpanded = !isExpanded;
  }

  onMount(() => {
    if (currentDirectory) {
      loadGitRefs();
    }
  });

  $effect(() => {
    if (currentDirectory) {
      loadGitRefs();
    }
  });
</script>

<div class="comparison-controls">
  <div
    class="comparison-header"
    onclick={toggleExpanded}
    onkeydown={(e) => (e.key === "Enter" || e.key === " ") && toggleExpanded()}
    tabindex="0"
    role="button"
    aria-expanded={isExpanded}
  >
    <div class="comparison-summary">
      <span class="comparison-label">Comparing:</span>
      <strong>{getDisplayName(comparisonSource)}</strong>
      <span class="vs">vs</span>
      <strong>{getDisplayName(comparisonTarget)}</strong>
    </div>
    <div class="expand-icon" class:expanded={isExpanded}>▼</div>
  </div>

  {#if isExpanded}
    <div class="comparison-details">
      <div class="comparison-row">
        <div class="comparison-field">
          <label for="source-select">Compare from:</label>
          <select id="source-select" bind:value={comparisonSource}>
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
          <select id="target-select" bind:value={comparisonTarget}>
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
    </div>
  {/if}
</div>

<style>
  .comparison-controls {
    margin-top: 1rem;
    border: 1px solid #dee2e6;
    border-radius: 6px;
    overflow: hidden;
  }

  .comparison-header {
    padding: 0.75rem 1rem;
    background: #f8f9fa;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    user-select: none;
    transition: background-color 0.2s;
  }

  .comparison-header:hover {
    background: #e9ecef;
  }

  .comparison-summary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .comparison-label {
    color: #6c757d;
  }

  .vs {
    color: #6c757d;
    font-style: italic;
  }

  .expand-icon {
    font-size: 0.7rem;
    color: #6c757d;
    transition: transform 0.2s;
  }

  .expand-icon.expanded {
    transform: rotate(180deg);
  }

  .comparison-details {
    padding: 1rem;
    background: #fff;
    border-top: 1px solid #dee2e6;
  }

  .comparison-row {
    display: flex;
    align-items: end;
    gap: 1rem;
    flex-wrap: wrap;
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

  @media (prefers-color-scheme: dark) {
    .comparison-controls {
      border-color: #555;
    }

    .comparison-header {
      background: #444;
      color: #f6f6f6;
    }

    .comparison-header:hover {
      background: #555;
    }

    .comparison-label,
    .vs,
    .expand-icon {
      color: #adb5bd;
    }

    .comparison-details {
      background: #333;
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
  }
</style>
