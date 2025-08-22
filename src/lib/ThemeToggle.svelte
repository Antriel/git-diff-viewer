<script>
  let { theme = $bindable("auto") } = $props();

  function toggleTheme() {
    if (theme === "auto") {
      theme = "light";
    } else if (theme === "light") {
      theme = "dark";
    } else {
      theme = "auto";
    }
  }

  function getThemeIcon() {
    switch (theme) {
      case "light":
        return "☀️";
      case "dark":
        return "🌙";
      default:
        return "🔄";
    }
  }

  function getThemeTitle() {
    switch (theme) {
      case "light":
        return "Switch to dark mode";
      case "dark":
        return "Switch to auto mode";
      default:
        return "Switch to light mode";
    }
  }
</script>

<button
  onclick={toggleTheme}
  class="theme-toggle"
  title={getThemeTitle()}
  aria-label={getThemeTitle()}
>
  {getThemeIcon()}
</button>

<style>
  .theme-toggle {
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 6px;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 1rem;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    color: var(--text-color);
  }

  .theme-toggle:hover {
    background: var(--button-bg-hover);
  }
</style>
