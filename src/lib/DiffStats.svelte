<script>
  let { gitDiffResult = null } = $props();
</script>

<div class="stats">
  {#if gitDiffResult}
    <strong>{gitDiffResult.total_stats.files} files</strong> changed,
    <span class="added">+{gitDiffResult.total_stats.added}</span>,
    <span class="removed">-{gitDiffResult.total_stats.removed}</span>
  {:else}
    <span class="placeholder">No diff data loaded</span>
  {/if}
</div>

<style>
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
    color: var(--text-muted);
    font-style: italic;
  }
</style>
