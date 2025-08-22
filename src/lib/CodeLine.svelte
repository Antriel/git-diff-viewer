<script>
  let { oldNum, newNum, content, cssClasses, isMatch } = $props();
</script>

<div class="code-line {cssClasses.join(' ')}" class:match={isMatch}>
  <span class="line-num old">{oldNum}</span>
  <span class="line-num new">{newNum}</span>
  <span class="line-content">{@html content}</span>
</div>

<style>
  .code-line {
    display: flex;
    align-items: center;
    line-height: 1;
  }

  .line-num {
    flex: 0 0 3em;
    text-align: right;
    padding: 0 0.5em;
    color: var(--text-muted);
    user-select: none;
    background: var(--line-num-bg);
    border-right: 1px solid var(--border-light);
  }

  .line-num.new {
    border-left: 1px solid var(--border-light);
  }

  .line-content {
    flex: 1;
    padding: 0 0.75em;
    white-space: pre;
    word-break: break-all;
  }

  .code-line.context {
    background: var(--context-bg);
  }

  .code-line.added {
    background: var(--added-bg);
  }

  .code-line.removed {
    background: var(--removed-bg);
  }

  .code-line.match {
    position: relative;
  }

  .code-line.match::before {
    content: "";
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: #ffd700;
    z-index: 1;
  }

  @media (max-width: 768px) {
    .line-num {
      flex: 0 0 2.5em;
    }
  }
</style>
