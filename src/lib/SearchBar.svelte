<script>
  import { createEventDispatcher } from "svelte";

  let { searchTerm = "", resultsCount = 0, totalCount = 0 } = $props();

  const dispatch = createEventDispatcher();

  let inputElement;

  function handleInput(event) {
    const term = event.target.value;
    dispatch("search", { term });
  }

  function clearSearch() {
    if (inputElement) {
      inputElement.value = "";
      dispatch("search", { term: "" });
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
      value={searchTerm}
      oninput={handleInput}
      onkeydown={handleKeydown}
      class="search-input"
    />
    {#if searchTerm}
      <button onclick={clearSearch} class="clear-btn" title="Clear search (Esc)">
        ✕
      </button>
    {/if}
  </div>
  
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
    border: 1px solid #ccc;
    border-radius: 6px;
    font-family: inherit;
    font-size: 0.9rem;
    transition: border-color 0.2s;
    box-sizing: border-box;
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
    color: #666;
    font-size: 0.8rem;
    transition: all 0.2s;
  }

  .clear-btn:hover {
    background: #f0f0f0;
    color: #333;
  }

  .search-results {
    flex: 0 0 auto;
    font-size: 0.85rem;
    color: #666;
    white-space: nowrap;
  }

  .results-count {
    font-weight: 600;
  }

  .shortcuts {
    flex: 0 0 auto;
    font-size: 0.75rem;
    color: #999;
    white-space: nowrap;
  }

  .kbd {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 3px;
    padding: 2px 6px;
    font-size: 0.7em;
    font-family: inherit;
  }

  @media (prefers-color-scheme: dark) {
    .search-input {
      background: #3a3a3a;
      border-color: #555;
      color: #f6f6f6;
    }

    .search-input:focus {
      border-color: #0066cc;
    }

    .search-input::placeholder {
      color: #ccc;
    }

    .clear-btn {
      color: #ccc;
    }

    .clear-btn:hover {
      background: #555;
      color: #f6f6f6;
    }

    .search-results {
      color: #ccc;
    }

    .shortcuts {
      color: #888;
    }

    .kbd {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }
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
