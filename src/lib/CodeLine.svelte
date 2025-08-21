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
    color: #999;
    user-select: none;
    background: #f8f9fa;
    border-right: 1px solid #eee;
  }

  .line-num.new {
    border-left: 1px solid #eee;
  }

  .line-content {
    flex: 1;
    padding: 0 0.75em;
    white-space: pre;
    word-break: break-all;
  }

  .code-line.context {
    background: #fff;
  }

  .code-line.added {
    background: #e6ffed;
  }

  .code-line.removed {
    background: #ffeef0;
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

  @media (prefers-color-scheme: dark) {
    .line-num {
      background: #333;
      border-color: #555;
      color: #999;
    }

    .code-line.context {
      background: #2a2a2a;
    }

    .code-line.added {
      background: #1f3a2e;
    }

    .code-line.removed {
      background: #3a1f1f;
    }
  }

  @media (max-width: 768px) {
    .line-num {
      flex: 0 0 2.5em;
    }
  }
</style>
