<script>
  let { searchMode = $bindable("both") } = $props();
</script>

<div class="search-mode-selector">
  <div class="segmented-control">
    <button
      class="segment"
      class:active={searchMode === "added"}
      onclick={() => (searchMode = "added")}
      title="Search in added lines only"
    >
      <span class="segment-icon">+</span>
      <span class="segment-text">Added</span>
    </button>
    <button
      class="segment"
      class:active={searchMode === "removed"}
      onclick={() => (searchMode = "removed")}
      title="Search in removed lines only"
    >
      <span class="segment-icon">−</span>
      <span class="segment-text">Removed</span>
    </button>
    <button
      class="segment"
      class:active={searchMode === "both"}
      onclick={() => (searchMode = "both")}
      title="Search in both added and removed lines"
    >
      <span class="segment-icon">±</span>
      <span class="segment-text">Both</span>
    </button>
  </div>
</div>

<style>
  .search-mode-selector {
    flex: 0 0 auto;
  }

  .segmented-control {
    display: flex;
    background: var(--component-bg);
    border: 1px solid var(--border-light);
    border-radius: 6px;
    overflow: hidden;
    box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .segment {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.4rem 0.7rem;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
    border-right: 1px solid var(--border-light);
    font-size: 0.8rem;
    font-family: inherit;
    min-width: 0;
  }

  .segment:last-child {
    border-right: none;
  }

  .segment:hover {
    background: var(--component-bg-hover);
    color: var(--text-color);
  }

  .segment.active {
    background: var(--accent-color);
    color: var(--accent-text);
    font-weight: 600;
    box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.2);
  }

  .segment.active:hover {
    background: var(--accent-hover);
    color: var(--accent-text);
  }

  /* Active segment icons use accent text color */
  .segment.active .segment-icon {
    color: var(--accent-text);
    font-weight: 700;
  }

  .segment-icon {
    font-weight: bold;
    font-size: 0.9rem;
    line-height: 1;
    min-width: 0.9rem;
    text-align: center;
  }

  .segment-text {
    white-space: nowrap;
  }

  /* Color the icons based on their meaning when not active */
  .segment:not(.active) .segment-icon {
    color: var(--text-muted);
  }

  .segment:not(.active):nth-child(1) .segment-icon {
    color: #4ade80; /* Green that works in both themes */
  }

  .segment:not(.active):nth-child(2) .segment-icon {
    color: #f87171; /* Red that works in both themes */
  }

  @media (max-width: 768px) {
    .segment {
      padding: 0.35rem 0.5rem;
      font-size: 0.75rem;
    }

    .segment-text {
      display: none;
    }

    .segment-icon {
      min-width: auto;
    }
  }
</style>
