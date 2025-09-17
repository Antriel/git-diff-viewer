<script>
  import FileGroup from "./FileGroup.svelte";

  let {
    gitDiffResult,
    searchTerm = $bindable(""),
    currentDirectory = "",
    visibleCount = $bindable(0),
    totalCount = $bindable(0),
    searchMode = "both",
  } = $props();

  // Group hunks by file name and apply filtering
  let groupedFiles = $derived.by(() => {
    if (!gitDiffResult?.hunks) return [];

    // First group hunks by file name
    const fileGroups = {};
    gitDiffResult.hunks.forEach((hunk) => {
      if (!fileGroups[hunk.file_name]) {
        fileGroups[hunk.file_name] = [];
      }
      fileGroups[hunk.file_name].push(hunk);
    });

    // If no search term, return all groups
    if (!searchTerm.trim()) {
      return Object.entries(fileGroups).map(([fileName, hunks]) => ({
        fileName,
        hunks,
      }));
    }

    // Filter groups based on search criteria
    const searchLower = searchTerm.toLowerCase();
    const filteredGroups = [];

    Object.entries(fileGroups).forEach(([fileName, hunks]) => {
      // Check if file name matches
      if (fileName.toLowerCase().includes(searchLower)) {
        filteredGroups.push({ fileName, hunks });
        return;
      }

      // Filter hunks within this file based on search mode
      const matchingHunks = hunks.filter((hunk) => {
        return hunk.hunk_lines.some((line) => {
          const isAdded = line.startsWith("+");
          const isRemoved = line.startsWith("-");

          let shouldSearchLine = false;
          if (searchMode === "added" && isAdded) {
            shouldSearchLine = true;
          } else if (searchMode === "removed" && isRemoved) {
            shouldSearchLine = true;
          } else if (searchMode === "both" && (isAdded || isRemoved)) {
            shouldSearchLine = true;
          }

          return shouldSearchLine && line.toLowerCase().includes(searchLower);
        });
      });

      // Only include file group if it has matching hunks
      if (matchingHunks.length > 0) {
        filteredGroups.push({ fileName, hunks: matchingHunks });
      }
    });

    return filteredGroups;
  });

  // Update bindable props
  $effect(() => {
    const totalFilteredHunks = groupedFiles.reduce((sum, group) => sum + group.hunks.length, 0);
    visibleCount = totalFilteredHunks;
    totalCount = gitDiffResult?.hunks?.length || 0;
  });
</script>

<div class="diff-viewer">
  {#if groupedFiles.length === 0 && searchTerm}
    <div class="no-results">
      <h3>No matches found</h3>
      <p>No files match your search term: "{searchTerm}"</p>
    </div>
  {:else if groupedFiles.length === 0}
    <div class="no-hunks">
      <h3>No changes to display</h3>
      <p>This repository has no uncommitted changes.</p>
    </div>
  {:else}
    <div>
      {#each groupedFiles as fileGroup (fileGroup.fileName)}
        <FileGroup
          fileName={fileGroup.fileName}
          hunks={fileGroup.hunks}
          {searchTerm}
          {searchMode}
          {currentDirectory}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .diff-viewer {
    background: var(--component-bg);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .no-results,
  .no-hunks {
    text-align: center;
    padding: 3rem;
    color: var(--text-muted);
  }

  .no-results h3,
  .no-hunks h3 {
    margin: 0 0 0.5rem 0;
    color: var(--text-color);
  }

  .no-results p,
  .no-hunks p {
    margin: 0;
  }
</style>
