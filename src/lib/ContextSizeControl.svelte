<script>
  import { debounce } from "./utils.js";

  let { contextSize = $bindable(3) } = $props();

  let displayContextSize = $state(contextSize);
  let isUpdating = $state(false);

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

<div class="context-controls">
  <span>Context:</span>
  <button onclick={() => adjustContext(-3)} disabled={displayContextSize === 0}
    >-</button
  >
  <span
    class="context-size"
    class:updating={isUpdating}
    class:dragging={isDragging}
    role="slider"
    aria-valuenow={displayContextSize}
    aria-valuemin="0"
    aria-label="Context size"
    tabindex="0"
    onmousedown={handleMouseDown}
    onwheel={handleWheel}>{displayContextSize}</span
  >
  <button onclick={() => adjustContext(3)}>+</button>
</div>

<style>
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
