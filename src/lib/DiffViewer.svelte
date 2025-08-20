<script>
  import DiffHunk from "./DiffHunk.svelte";

  let {
    gitDiffResult,
    searchTerm = $bindable(""),
    currentDirectory = "",
    visibleCount = $bindable(0),
    totalCount = $bindable(0),
  } = $props();

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

  // Update bindable props
  $effect(() => {
    visibleCount = filteredHunks.length;
    totalCount = gitDiffResult?.hunks?.length || 0;
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
    <div>
      {#each filteredHunks as hunk, index}
        <DiffHunk {hunk} {searchTerm} {currentDirectory} hunkIndex={index} />
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
