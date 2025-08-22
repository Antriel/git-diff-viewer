<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DiffViewer from "../lib/DiffViewer.svelte";
  import DiffHeader from "../lib/DiffHeader.svelte";
  import DirectorySelector from "../lib/DirectorySelector.svelte";
  import AboutModal from "../lib/AboutModal.svelte";
  import ThemeToggle from "../lib/ThemeToggle.svelte";

  // Import highlight.js CSS as URLs - Vite will handle bundling
  import githubLightCss from "highlight.js/styles/github.css?url";
  import githubDarkCss from "highlight.js/styles/github-dark.css?url";

  let currentDirectory = $state("");
  /** @type {any} */
  let gitDiffResult = $state(null);
  let loading = $state(false);
  let error = $state("");
  let currentContextSize = $state(
    parseInt(localStorage.getItem("gitDiffContextSize") || "3", 10)
  );
  let searchTerm = $state("");
  let searchMode = $state(localStorage.getItem("gitDiffSearchMode") || "both");
  let visibleCount = $state(0);
  let totalCount = $state(0);
  let includeUntracked = $state(
    localStorage.getItem("gitDiffIncludeUntracked") === "true"
  );
  let comparisonSource = $state("working");
  let comparisonTarget = $state("HEAD");
  let showAbout = $state(false);
  let theme = $state(localStorage.getItem("gitDiffTheme") || "auto");
  let appliedTheme = $state("light");

  function getSavedProjects() {
    return JSON.parse(localStorage.getItem("gitDiffProjects") || "[]");
  }

  async function loadGitDiff() {
    if (!currentDirectory) return;

    loading = true;
    error = "";

    try {
      const result = await invoke("get_git_diff", {
        directoryPath: currentDirectory,
        contextLines: currentContextSize,
        includeUntracked: includeUntracked,
        comparisonSource: comparisonSource,
        comparisonTarget: comparisonTarget,
      });
      gitDiffResult = result;

      saveProjectToHistory(currentDirectory);
    } catch (err) {
      error = err instanceof Error ? err.message : String(err);
      gitDiffResult = null;
    } finally {
      loading = false;
    }
  }

  function handleDirectorySelected(obj) {
    currentDirectory = obj.directory;
    comparisonSource = "working";
    comparisonTarget = "HEAD";
    loadGitDiff();
  }

  function handleRefresh() {
    loadGitDiff();
  }

  // Watch for changes to comparison settings, untracked toggle, and context size
  $effect(() => {
    loadGitDiff();
    localStorage.setItem("gitDiffContextSize", currentContextSize.toString());
    localStorage.setItem(
      "gitDiffIncludeUntracked",
      includeUntracked.toString()
    );
  });

  function updateAppliedTheme() {
    if (theme === "auto") {
      const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
      appliedTheme = mediaQuery.matches ? "dark" : "light";
    } else {
      appliedTheme = theme;
    }
  }

  function updateHighlightTheme(theme) {
    // Remove existing highlight.js stylesheet
    const existingLink = document.querySelector("link[data-highlight-theme]");
    if (existingLink) {
      existingLink.remove();
    }

    // Add new highlight.js stylesheet using imported URLs
    const link = document.createElement("link");
    link.rel = "stylesheet";
    // Use Vite-processed URLs - works in both dev and production
    link.href = theme === "dark" ? githubDarkCss : githubLightCss;
    link.setAttribute("data-highlight-theme", theme);
    document.head.appendChild(link);
  }

  function applyThemeToBody() {
    // Remove existing theme classes
    document.body.classList.remove("dark-theme", "light-theme");
    // Apply current theme
    if (appliedTheme === "dark") {
      document.body.classList.add("dark-theme");
    } else {
      document.body.classList.add("light-theme");
    }

    // Update highlight.js theme
    updateHighlightTheme(appliedTheme);
  }

  onMount(() => {
    // Enable transitions, postponed until now to prevent initial flash
    document.body.classList.add("transitions-enabled");

    // Set up theme detection
    updateAppliedTheme();
    applyThemeToBody();

    // Listen for system theme changes
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const handleThemeChange = () => {
      if (theme === "auto") {
        updateAppliedTheme();
      }
    };
    mediaQuery.addEventListener("change", handleThemeChange);

    // Load the last opened project
    const savedProjects = getSavedProjects();
    if (savedProjects.length > 0) {
      const lastProject = savedProjects[0];
      currentDirectory = lastProject.path;
      loadGitDiff();
    }

    // Cleanup listener on unmount
    return () => {
      mediaQuery.removeEventListener("change", handleThemeChange);
    };
  });

  // Update applied theme when theme selection changes
  $effect(() => {
    updateAppliedTheme();
    applyThemeToBody();
    localStorage.setItem("gitDiffTheme", theme);
  });

  // Save search mode preference
  $effect(() => {
    localStorage.setItem("gitDiffSearchMode", searchMode);
  });

  function saveProjectToHistory(directory) {
    const savedProjects = getSavedProjects();
    const existing = savedProjects.find((p) => p.path === directory);

    if (existing) {
      existing.lastOpened = new Date().toISOString();
    } else {
      savedProjects.unshift({
        path: directory,
        name: directory.split(/[/\\]/).pop() || directory,
        lastOpened: new Date().toISOString(),
      });
    }

    // Sort by lastOpened date (most recent first)
    savedProjects.sort(
      (a, b) =>
        new Date(b.lastOpened).getTime() - new Date(a.lastOpened).getTime()
    );

    // Keep only the last 10 projects
    if (savedProjects.length > 10) {
      savedProjects.splice(10);
    }

    localStorage.setItem("gitDiffProjects", JSON.stringify(savedProjects));
  }
</script>

<main class="container">
  <header>
    <div class="header-left">
      <h1>Git Diff Viewer</h1>
      <AboutModal bind:show={showAbout} />
      <ThemeToggle bind:theme />
    </div>
    <DirectorySelector
      {currentDirectory}
      onDirectorySelected={handleDirectorySelected}
    />
  </header>

  {#if currentDirectory}
    <div class="diff-header-wrapper">
      <DiffHeader
        {gitDiffResult}
        bind:contextSize={currentContextSize}
        bind:searchTerm
        bind:searchMode
        bind:visibleCount
        bind:totalCount
        bind:includeUntracked
        {currentDirectory}
        bind:comparisonSource
        bind:comparisonTarget
        onRefresh={handleRefresh}
      />
    </div>
  {/if}

  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading git diff...</p>
    </div>
  {:else if error}
    <div class="error">
      <h3>Error</h3>
      <p>{error}</p>
      {#if currentDirectory}
        <button onclick={handleRefresh}>Try Again</button>
      {/if}
    </div>
  {:else if gitDiffResult}
    <DiffViewer
      {gitDiffResult}
      bind:searchTerm
      {searchMode}
      {currentDirectory}
      bind:visibleCount
      bind:totalCount
    />
  {:else if currentDirectory}
    <div class="no-changes">
      <h3>No Changes</h3>
      <p>No uncommitted changes found in this repository.</p>
      <button onclick={handleRefresh}>Refresh</button>
    </div>
  {:else}
    <div class="welcome">
      <h2>Welcome to Git Diff Viewer</h2>
      <p>
        Select a git repository to view and search through your uncommitted
        changes.
      </p>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas,
      "Liberation Mono", monospace;
    background: var(--bg-color);
    color: var(--text-color);
    margin: 0;
    padding: 0;
    line-height: 1.4;
    overflow-y: scroll;
  }

  :global(transitions-enabled *) {
    transition:
      background-color 0.5s ease,
      color 0.5s ease,
      border-color 0.5s ease,
      box-shadow 0.5s ease;
  }

  :global(body.dark-theme) {
    /* Base colors */
    --bg-color: #1a1a1a;
    --text-color: #f6f6f6;
    --header-bg: #1a1a1a;
    --welcome-bg: #2a2a2a;
    --border-color: #444;
    --error-bg: #3a1f1f;
    --error-border: #ff6b6b;

    /* Component backgrounds */
    --component-bg: #333;
    --component-bg-hover: #555;
    --input-bg: #3a3a3a;
    --modal-bg: #2a2a2a;

    /* Text colors */
    --text-secondary: #ccc;
    --text-muted: #999;
    --link-color: #66b3ff;

    /* Accent colors */
    --accent-color: #66b3ff;
    --accent-hover: #4da3ff;
    --accent-text: #0a0a0a;

    /* Button colors */
    --button-bg: #444;
    --button-bg-hover: #555;
    --button-border: #666;

    /* Diff colors */
    --line-num-bg: #333;
    --context-bg: #2a2a2a;
    --added-bg: #1f3a2e;
    --removed-bg: #3a1f1f;

    /* Border colors */
    --border-light: #555;
    --border-medium: #666;

    /* Highlight colors */
    --highlight-bg: #3a3a1a;
    --highlight-text: #ffeb3b;
    --word-highlight-bg: rgba(255, 255, 0, 0.2);
  }

  :global(body.light-theme) {
    /* Base colors */
    --bg-color: #f5f5f5;
    --text-color: #333;
    --header-bg: #f5f5f5;
    --welcome-bg: #fff;
    --border-color: #ddd;
    --error-bg: #ffe6e6;
    --error-border: #ff6b6b;

    /* Component backgrounds */
    --component-bg: #f8f9fa;
    --component-bg-hover: #e9ecef;
    --input-bg: #fff;
    --modal-bg: #fff;

    /* Text colors */
    --text-secondary: #666;
    --text-muted: #999;
    --link-color: #0066cc;

    /* Accent colors */
    --accent-color: #0066cc;
    --accent-hover: #0052a3;
    --accent-text: #ffffff;

    /* Button colors */
    --button-bg: #f8f9fa;
    --button-bg-hover: #e9ecef;
    --button-border: #dee2e6;

    /* Diff colors */
    --line-num-bg: #f8f9fa;
    --context-bg: #fff;
    --added-bg: #e8f5e8;
    --removed-bg: #ffe6e6;

    /* Border colors */
    --border-light: #dee2e6;
    --border-medium: #adb5bd;

    /* Highlight colors */
    --highlight-bg: #fff3cd;
    --highlight-text: #856404;
    --word-highlight-bg: rgba(255, 255, 0, 0.15);
  }

  .container {
    width: auto;
    margin: 0;
    padding: 1rem;
    height: auto;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 1rem;
    position: sticky;
    top: 0;
    background: var(--header-bg);
    z-index: 100;
    padding: 1rem;
    margin: -1rem -1rem 0 -1rem;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    color: var(--text-color);
    font-size: 1.8rem;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 3rem;
    gap: 1rem;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid #333;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .error {
    background: var(--error-bg);
    border: 1px solid var(--error-border);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 2rem 0;
  }

  .error h3 {
    margin: 0 0 0.5rem 0;
    color: #d63031;
  }

  .error p {
    margin: 0 0 1rem 0;
    color: #636e72;
  }

  .no-changes {
    text-align: center;
    padding: 3rem;
    background: var(--welcome-bg);
    border-radius: 8px;
    border: 1px solid var(--border-color);
  }

  .no-changes h3 {
    margin: 0 0 0.5rem 0;
    color: var(--text-color);
  }

  .no-changes p {
    margin: 0 0 1.5rem 0;
    color: #666;
  }

  .welcome {
    text-align: center;
    padding: 4rem 2rem;
    background: var(--welcome-bg);
    border-radius: 8px;
    border: 1px solid var(--border-color);
  }

  .welcome h2 {
    margin: 0 0 1rem 0;
    color: var(--text-color);
  }

  .welcome p {
    margin: 0;
    color: #666;
    font-size: 1.1rem;
  }

  button {
    background: var(--accent-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    font-size: 0.9rem;
    transition: background-color 0.2s;
  }

  button:hover {
    background: var(--accent-hover);
  }

  .diff-header-wrapper {
    position: sticky;
    top: 72px; /* Height of header */
    background: var(--header-bg);
    z-index: 99;
    margin: 0 -1rem 1rem -1rem;
    padding: 1rem;
  }
</style>
