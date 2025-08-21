<script>
  import { formatRelativeTime, formatLocalTime } from "./utils.js";

  let { stats } = $props();
</script>

<div class="file-stats">
  <span class="added">+{stats.added}</span>
  <span class="removed">-{stats.removed}</span>
  <span class="size">{(stats.size / 1024).toFixed(1)}KB</span>
  <span class="modified" title={formatLocalTime(stats.modified)}
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

  .file-stats .added {
    color: #28a745;
    font-weight: bold;
  }

  .file-stats .removed {
    color: #dc3545;
    font-weight: bold;
  }

  .file-stats .size {
    color: #666;
  }

  .file-stats .modified {
    color: #666;
    cursor: help;
  }

  @media (prefers-color-scheme: dark) {
    .file-stats .size {
      color: #ccc;
    }

    .file-stats .modified {
      color: #ccc;
    }
  }

  @media (max-width: 768px) {
    .file-stats {
      justify-content: space-between;
    }
  }
</style>
