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
  class="theme-toggle {theme}"
  title={getThemeTitle()}
  aria-label={getThemeTitle()}
>
  <div class="icon-container">
    <div class="sun-icon">
      <div class="sun-center"></div>
      <div class="sun-rays">
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
        <div class="ray"></div>
      </div>
    </div>
    <div class="moon-icon">
      <div class="moon-shape"></div>
    </div>
    <div class="auto-icon">
      <div class="auto-circle">
        <div class="auto-half"></div>
      </div>
    </div>
  </div>
</button>

<style>
  .theme-toggle {
    background: var(--button-bg);
    border: 1px solid var(--button-border);
    border-radius: 8px;
    padding: 0;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    position: relative;
    overflow: hidden;
  }

  .theme-toggle:hover {
    background: var(--button-bg-hover);
    transform: scale(1.05);
  }

  .icon-container {
    position: relative;
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .sun-icon,
  .moon-icon,
  .auto-icon {
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    transform: scale(0.8) rotate(180deg);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .theme-toggle.light .sun-icon {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }

  .theme-toggle.dark .moon-icon {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }

  .theme-toggle.auto .auto-icon {
    opacity: 1;
    transform: scale(1) rotate(0deg);
  }

  /* Sun Icon */
  .sun-center {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #ffa500;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    transition: all 0.3s ease;
  }

  .sun-rays {
    position: absolute;
    width: 100%;
    height: 100%;
  }

  .ray {
    position: absolute;
    width: 2px;
    height: 4px;
    background: #ffa500;
    border-radius: 1px;
    top: 1px;
    left: 50%;
    transform: translateX(-50%);
    transform-origin: center 8px;
  }

  .ray:nth-child(1) { transform: translateX(-50%) rotate(0deg); }
  .ray:nth-child(2) { transform: translateX(-50%) rotate(45deg); }
  .ray:nth-child(3) { transform: translateX(-50%) rotate(90deg); }
  .ray:nth-child(4) { transform: translateX(-50%) rotate(135deg); }
  .ray:nth-child(5) { transform: translateX(-50%) rotate(180deg); }
  .ray:nth-child(6) { transform: translateX(-50%) rotate(225deg); }
  .ray:nth-child(7) { transform: translateX(-50%) rotate(270deg); }
  .ray:nth-child(8) { transform: translateX(-50%) rotate(315deg); }

  /* Moon Icon */
  .moon-shape {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #4a90e2;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    transition: all 0.3s ease;
  }

  .moon-shape::after {
    content: '';
    position: absolute;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--button-bg);
    top: 1px;
    right: 2px;
    transition: background 0.3s ease;
  }

  /* Auto Icon - Half Sun Half Moon */
  .auto-circle {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    overflow: hidden;
    background: linear-gradient(90deg, #ffa500 50%, #4a90e2 50%);
    transition: all 0.3s ease;
  }

  .auto-half {
    position: absolute;
    width: 50%;
    height: 100%;
    right: 0;
    background: #4a90e2;
    border-radius: 0 50% 50% 0;
  }

  @keyframes rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .theme-toggle:hover .sun-rays {
    animation: rotate 2s linear infinite;
  }

  .theme-toggle:hover .auto-circle {
    animation: wiggle 0.6s ease-in-out;
  }

  @keyframes wiggle {
    0%, 100% { transform: translate(-50%, -50%) rotate(0deg); }
    25% { transform: translate(-50%, -50%) rotate(-5deg); }
    75% { transform: translate(-50%, -50%) rotate(5deg); }
  }

  .theme-toggle:hover .moon-shape {
    animation: moonGlow 0.6s ease-in-out;
  }

  @keyframes moonGlow {
    0%, 100% { filter: brightness(1); }
    50% { filter: brightness(1.3); }
  }
</style>
