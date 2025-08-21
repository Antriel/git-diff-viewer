<script>
  import SearchBar from "./SearchBar.svelte";
  import ComparisonControls from "./ComparisonControls.svelte";

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

  let displayContextSize = $state(contextSize);
  let isUpdating = $state(false);

  function debounce(fn, ms) {
    let timeoutId;
    return (...args) => {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => fn(...args), ms);
    };
  }

  const updateContextSize = debounce((newSize) => {
    contextSize = newSize;
    isUpdating = false;
  }, 300);

  function adjustContext(delta) {
    const newSize = Math.max(0, displayContextSize + delta);
    displayContextSize = newSize;
    isUpdating = true;
    updateContextSize(newSize);
  }

  let isDragging = $state(false);
  let dragStartY = 0;
  let dragStartValue = 0;

  function handleMouseDown(event) {
    isDragging = true;
    dragStartY = event.clientY;
    dragStartValue = displayContextSize;
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);
    event.preventDefault();
  }

  function handleMouseMove(event) {
    if (!isDragging) return;
    const deltaY = dragStartY - event.clientY;
    const steps = Math.floor(deltaY / 10);
    const newSize = Math.max(0, dragStartValue + steps);
    if (newSize !== displayContextSize) {
      displayContextSize = newSize;
      isUpdating = true;
    }
  }

  function handleMouseUp() {
    if (isDragging && displayContextSize !== contextSize) {
      contextSize = displayContextSize;
    }
    isDragging = false;
    isUpdating = false;
    document.removeEventListener("mousemove", handleMouseMove);
    document.removeEventListener("mouseup", handleMouseUp);
  }

  function handleWheel(event) {
    event.preventDefault();
    const delta = event.deltaY > 0 ? -1 : 1;
    adjustContext(delta);
  }

  $effect(() => {
    displayContextSize = contextSize;
    isUpdating = false;
  });
</script>

<div class="diff-header">
  <div class="stats">
    {#if gitDiffResult}
      <strong>{gitDiffResult.total_stats.files} files</strong> changed,
      <span class="added">+{gitDiffResult.total_stats.added}</span>,
      <span class="removed">-{gitDiffResult.total_stats.removed}</span>
    {:else}
      <span class="placeholder">No diff data loaded</span>
    {/if}
  </div>

  <div class="toolbar">
    <SearchBar
      bind:searchTerm
      bind:resultsCount={visibleCount}
      bind:totalCount
    />
    <div class="untracked-toggle">
      <label>
        <input type="checkbox" bind:checked={includeUntracked} />
        Include untracked files
      </label>
    </div>
    <div class="context-controls">
      <span>Context:</span>
      <button
        onclick={() => adjustContext(-3)}
        disabled={displayContextSize === 0}>-</button
      >
      <span
        class="context-size"
        class:updating={isUpdating}
        class:dragging={isDragging}
        onmousedown={handleMouseDown}
        onwheel={handleWheel}>{displayContextSize}</span
      >
      <button onclick={() => adjustContext(3)}>+</button>
    </div>
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

  .stats .placeholder {
    color: #6c757d;
    font-style: italic;
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

  .untracked-toggle {
    display: flex;
    align-items: center;
    font-size: 0.85rem;
  }

  .untracked-toggle label {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    cursor: pointer;
    user-select: none;
  }

  .untracked-toggle input[type="checkbox"] {
    cursor: pointer;
  }

  .context-controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85rem;
  }

  .context-controls button {
    background: #fff;
    border: 1px solid #ccc;
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    cursor: pointer;
    font-size: 0.8rem;
    transition: all 0.2s;
  }

  .context-controls button:hover:not(:disabled) {
    background: #f0f0f0;
  }

  .context-controls button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .context-size {
    min-width: 1.5rem;
    text-align: center;
    font-weight: bold;
    transition: opacity 0.2s;
    cursor: ns-resize;
    user-select: none;
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
  }

  .context-size:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .context-size.updating {
    opacity: 0.6;
  }

  .context-size.dragging {
    background: rgba(0, 0, 0, 0.1);
    cursor: ns-resize;
  }

  @media (prefers-color-scheme: dark) {
    .diff-header {
      background: #333;
      border-color: #555;
    }

    .stats {
      color: #f6f6f6;
    }

    .stats .placeholder {
      color: #adb5bd;
    }

    .refresh-btn {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .refresh-btn:hover {
      background: #555;
    }

    .untracked-toggle {
      color: #f6f6f6;
    }

    .context-controls {
      color: #f6f6f6;
    }

    .context-controls button {
      background: #444;
      border-color: #666;
      color: #f6f6f6;
    }

    .context-controls button:hover:not(:disabled) {
      background: #555;
    }

    .context-size:hover {
      background: rgba(255, 255, 255, 0.05);
    }

    .context-size.dragging {
      background: rgba(255, 255, 255, 0.1);
    }
  }
</style>
