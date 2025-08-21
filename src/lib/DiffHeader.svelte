<script>
  import SearchBar from "./SearchBar.svelte";
  import ComparisonControls from "./ComparisonControls.svelte";
  import DiffStats from "./DiffStats.svelte";
  import UntrackedToggle from "./UntrackedToggle.svelte";
  import ContextSizeControl from "./ContextSizeControl.svelte";

  let {
    gitDiffResult = null,
    contextSize = $bindable(3),
    searchTerm = $bindable(""),
    visibleCount = $bindable(0),
    totalCount = $bindable(0),
    includeUntracked = $bindable(false),
    currentDirectory = "",
    comparisonSource = $bindable("working"),
    comparisonTarget = $bindable("HEAD"),
    onRefresh = () => {},
  } = $props();
</script>

<div class="diff-header">
  <DiffStats {gitDiffResult} />

  <div class="toolbar">
    <SearchBar
      bind:searchTerm
      bind:resultsCount={visibleCount}
      bind:totalCount
    />
    <UntrackedToggle bind:includeUntracked />
    <ContextSizeControl bind:contextSize />
    <button onclick={() => onRefresh()} class="refresh-btn" title="Refresh">
      🔄
    </button>
  </div>

  <ComparisonControls
    {currentDirectory}
    bind:comparisonSource
    bind:comparisonTarget
  />
</div>

<style>
  .diff-header {
    background: #f8f9fa;
    padding: 1rem;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    margin-bottom: 0;
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

  @media (prefers-color-scheme: dark) {
    .diff-header {
      background: #333;
      border-color: #555;
    }

    .refresh-btn {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .refresh-btn:hover {
      background: #555;
    }
  }
</style>
