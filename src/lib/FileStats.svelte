<script>
  import { formatRelativeTime, formatLocalTime } from "./utils.js";

  let { stats } = $props();
</script>

<div class="file-stats">
  <span class="stats-added">+{stats.added}</span>
  <span class="stats-removed">-{stats.removed}</span>
  <span class="size">{(stats.size / 1024).toFixed(1)}KB</span>
  <span
    class="modified"
    title={"Last modified: " + formatLocalTime(stats.modified)}
    >{formatRelativeTime(stats.modified)}</span
  >
</div>

<style>
  .file-stats {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
    white-space: nowrap;
  }


  .file-stats .size {
    color: var(--text-muted);
  }

  .file-stats .modified {
    color: var(--text-muted);
    cursor: help;
  }

  @media (max-width: 768px) {
    .file-stats {
      justify-content: space-between;
    }
  }
</style>
