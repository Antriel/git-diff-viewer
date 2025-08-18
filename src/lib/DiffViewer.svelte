<script>
  import { createEventDispatcher } from "svelte";
  import DiffHunk from "./DiffHunk.svelte";
  import SearchBar from "./SearchBar.svelte";

  let { gitDiffResult, currentDirectory } = $props();

  const dispatch = createEventDispatcher();

  let searchTerm = $state("");

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

  function handleSearch(event) {
    searchTerm = event.detail.term;
  }

  function handleRefresh() {
    dispatch("refresh");
  }
</script>

<div class="diff-viewer">
  <div class="header">
    <div class="stats">
      <strong>{gitDiffResult.total_stats.files} files</strong> changed,
      <span class="added">+{gitDiffResult.total_stats.added}</span>,
      <span class="removed">-{gitDiffResult.total_stats.removed}</span>
    </div>

    <div class="toolbar">
      <SearchBar
        {searchTerm}
        resultsCount={visibleCount}
        totalCount={gitDiffResult.hunks.length}
        on:search={handleSearch}
      />
      <button onclick={handleRefresh} class="refresh-btn" title="Refresh">
        🔄
      </button>
    </div>
  </div>

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

  .header {
    background: #f8f9fa;
    padding: 1rem;
    border-bottom: 1px solid #dee2e6;
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
    max-height: 70vh;
    overflow-y: auto;
  }

  @media (prefers-color-scheme: dark) {
    .diff-viewer {
      background: #2a2a2a;
    }

    .header {
      background: #333;
      border-color: #555;
    }

    .stats {
      color: #f6f6f6;
    }

    .refresh-btn {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .refresh-btn:hover {
      background: #555;
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
