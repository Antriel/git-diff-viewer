<script>
  import SearchModeSelector from "./SearchModeSelector.svelte";
  import { debounce } from "./utils.js";

  let {
    searchTerm = $bindable(""),
    resultsCount = $bindable(0),
    totalCount = $bindable(0),
    searchMode = $bindable("both"),
  } = $props();

  let inputElement;
  let internalSearchTerm = $state("");

  const debouncedSetSearchTerm = debounce((value) => {
    searchTerm = value;
  }, 300);

  $effect(() => {
    debouncedSetSearchTerm(internalSearchTerm);
  });

  function clearSearch() {
    internalSearchTerm = "";
    searchTerm = "";
    if (inputElement) {
      inputElement.value = "";
    }
  }

  function handleKeydown(event) {
    if (event.key === "Escape") {
      clearSearch();
    }
  }

  // Focus handling
  function focusInput() {
    if (inputElement) {
      inputElement.focus();
      inputElement.select();
    }
  }

  // Global keyboard shortcut
  function handleGlobalKeydown(event) {
    if (event.ctrlKey && event.key === "f") {
      event.preventDefault();
      focusInput();
    }
  }

  // Mount global keyboard listener
  if (typeof window !== "undefined") {
    window.addEventListener("keydown", handleGlobalKeydown);
  }
</script>

<div class="search-bar">
  <div class="search-input-container">
    <input
      bind:this={inputElement}
      type="text"
      placeholder="Search changes... (Ctrl+F)"
      bind:value={internalSearchTerm}
      onkeydown={handleKeydown}
      class="search-input"
    />
    {#if internalSearchTerm}
      <button
        onclick={clearSearch}
        class="clear-btn"
        title="Clear search (Esc)"
      >
        ✕
      </button>
    {/if}
  </div>

  <SearchModeSelector bind:searchMode />

  <div class="search-results">
    {#if searchTerm}
      <span class="results-count">
        {resultsCount} / {totalCount} hunks
      </span>
    {:else}
      <span class="total-count">
        {totalCount} hunks
      </span>
    {/if}
  </div>

  <div class="shortcuts">
    <span class="kbd">Ctrl+F</span> focus search •
    <span class="kbd">Esc</span> clear
  </div>
</div>

<style>
  .search-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex: 1;
    flex-wrap: nowrap;
    overflow: hidden;
  }

  .search-input-container {
    position: relative;
    flex: 1 1 auto;
    min-width: 200px;
    max-width: 400px;
    box-sizing: border-box;
  }

  .search-input {
    width: 100%;
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.9rem;
    transition: border-color 0.2s;
    box-sizing: border-box;
    background: var(--input-bg);
    color: var(--text-color);
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  .search-input:focus {
    outline: none;
    border-color: #0066cc;
    box-shadow: inset 0 0 0 2px rgba(0, 102, 204, 0.2);
  }

  .clear-btn {
    position: absolute;
    right: 0.5rem;
    top: 50%;
    transform: translateY(-50%);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 3px;
    color: var(--text-secondary);
    font-size: 0.8rem;
    transition: all 0.2s;
  }

  .clear-btn:hover {
    background: var(--button-bg-hover);
    color: var(--text-color);
  }

  .search-results {
    flex: 0 0 auto;
    font-size: 0.85rem;
    color: var(--text-secondary);
    white-space: nowrap;
  }

  .results-count {
    font-weight: 600;
  }

  .shortcuts {
    flex: 0 0 auto;
    font-size: 0.75rem;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .kbd {
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 3px;
    padding: 2px 6px;
    font-size: 0.7em;
    font-family: inherit;
    color: var(--text-color);
  }

  @media (max-width: 768px) {
    .search-bar {
      flex-direction: column;
      align-items: stretch;
      gap: 0.5rem;
    }

    .search-input-container {
      max-width: none;
    }

    .shortcuts {
      display: none;
    }
  }
</style>
