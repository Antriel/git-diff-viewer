<script>
  let { gitDiffResult = null } = $props();
</script>

<div class="stats">
  {#if gitDiffResult}
    <strong>{gitDiffResult.total_stats.files} files</strong> changed,
    <span class="stats-added">+{gitDiffResult.total_stats.added}</span>,
    <span class="stats-removed">-{gitDiffResult.total_stats.removed}</span>
  {:else}
    <span class="placeholder">No diff data loaded</span>
  {/if}
</div>

<style>
  .stats {
    margin-bottom: 0.5rem;
    font-size: 0.85rem;
  }


  .stats .placeholder {
    color: var(--text-muted);
    font-style: italic;
  }
</style>
