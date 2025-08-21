<script>
  let { includeUntracked = $bindable(false) } = $props();
</script>

<div class="untracked-toggle">
  <label>
    <input type="checkbox" bind:checked={includeUntracked} />
    Include untracked files
  </label>
</div>

<style>
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

  @media (prefers-color-scheme: dark) {
    .untracked-toggle {
      color: #f6f6f6;
    }
  }
</style>
