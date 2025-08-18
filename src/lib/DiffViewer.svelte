<script>
  import { createEventDispatcher } from "svelte";
  import DiffHunk from "./DiffHunk.svelte";

  let { gitDiffResult, searchTerm = "" } = $props();

  const dispatch = createEventDispatcher();

  // Use derived state to automatically compute filtered hunks
  let filteredHunks = $derived.by(() => {
    if (!gitDiffResult?.hunks) return [];

    if (!searchTerm.trim()) {
      return gitDiffResult.hunks;
    }

    const searchLower = searchTerm.toLowerCase();
    return gitDiffResult.hunks.filter((hunk) => {
      // Search in file name
      if (hunk.file_name.toLowerCase().includes(searchLower)) {
        return true;
      }

      // Search in hunk lines (only added/removed lines)
      return hunk.hunk_lines.some((line) => {
        if (line.startsWith("+") || line.startsWith("-")) {
          return line.toLowerCase().includes(searchLower);
        }
        return false;
      });
    });
  });

  let visibleCount = $derived(filteredHunks.length);

  // Dispatch visible count changes to parent
  $effect(() => {
    dispatch("visibleCountChange", {
      count: visibleCount,
      total: gitDiffResult?.hunks?.length || 0,
    });
  });
</script>

<div class="diff-viewer">
  {#if filteredHunks.length === 0 && searchTerm}
    <div class="no-results">
      <h3>No matches found</h3>
      <p>No hunks match your search term: "{searchTerm}"</p>
    </div>
  {:else if filteredHunks.length === 0}
    <div class="no-hunks">
      <h3>No changes to display</h3>
      <p>This repository has no uncommitted changes.</p>
    </div>
  {:else}
    <div class="hunks-container">
      {#each filteredHunks as hunk, index}
        <DiffHunk {hunk} {searchTerm} hunkIndex={index} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .diff-viewer {
    background: white;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .no-results,
  .no-hunks {
    text-align: center;
    padding: 3rem;
    color: #666;
  }

  .no-results h3,
  .no-hunks h3 {
    margin: 0 0 0.5rem 0;
    color: #333;
  }

  .no-results p,
  .no-hunks p {
    margin: 0;
  }

  .hunks-container {
    /* Removed max-height and overflow-y to allow natural scrolling at page level */
  }

  @media (prefers-color-scheme: dark) {
    .diff-viewer {
      background: #2a2a2a;
    }

    .no-results,
    .no-hunks {
      color: #ccc;
    }

    .no-results h3,
    .no-hunks h3 {
      color: #f6f6f6;
    }
  }
</style>
