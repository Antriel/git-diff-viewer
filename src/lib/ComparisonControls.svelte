<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { createEnterSpaceHandler } from "./utils.js";

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

  const handleToggleKeydown = createEnterSpaceHandler(() => {
    toggleExpanded();
  });

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
    onkeydown={handleToggleKeydown}
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
          class="btn-icon"
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
    border: 1px solid var(--border-color);
    border-radius: 6px;
    overflow: hidden;
  }

  .comparison-header {
    padding: 0.75rem 1rem;
    background: var(--header-bg);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    user-select: none;
    transition: background-color 0.2s;
  }

  .comparison-header:hover {
    background: var(--component-bg-hover);
  }

  .comparison-summary {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .comparison-label {
    color: var(--text-muted);
  }

  .vs {
    color: var(--text-muted);
    font-style: italic;
  }

  .expand-icon {
    font-size: 0.7rem;
    color: var(--text-muted);
    transition: transform 0.2s;
  }

  .expand-icon.expanded {
    transform: rotate(180deg);
  }

  .comparison-details {
    padding: 1rem;
    background: var(--bg-color);
    border-top: 1px solid var(--border-color);
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
    color: var(--text-secondary);
  }

  .comparison-field select {
    padding: 0.4rem;
    border: 1px solid var(--border-light);
    border-radius: 4px;
    font-size: 0.85rem;
    background: var(--input-bg);
    color: var(--text-color);
  }

</style>
