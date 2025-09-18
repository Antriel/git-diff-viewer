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
    searchMode = $bindable("both"),
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
      bind:searchMode
      bind:resultsCount={visibleCount}
      bind:totalCount
    />
    <UntrackedToggle bind:includeUntracked />
    <ContextSizeControl bind:contextSize />
    <button onclick={() => onRefresh()} class="btn-icon" title="Refresh">
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
    background: var(--component-bg);
    padding: 0.5rem;
    border: 1px solid var(--border-light);
    border-radius: 8px;
    margin-bottom: 0;
  }

  .toolbar {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

</style>
