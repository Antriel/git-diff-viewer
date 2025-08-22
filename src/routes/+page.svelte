<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DiffViewer from "../lib/DiffViewer.svelte";
  import DiffHeader from "../lib/DiffHeader.svelte";
  import DirectorySelector from "../lib/DirectorySelector.svelte";
  import AboutModal from "../lib/AboutModal.svelte";

  let currentDirectory = $state("");
  /** @type {any} */
  let gitDiffResult = $state(null);
  let loading = $state(false);
  let error = $state("");
  let currentContextSize = $state(3);
  let searchTerm = $state("");
  let visibleCount = $state(0);
  let totalCount = $state(0);
  let includeUntracked = $state(false);
  let comparisonSource = $state("working");
  let comparisonTarget = $state("HEAD");
  let showAbout = $state(false);

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
  });

  onMount(() => {
    // Load the last opened project
    const savedProjects = getSavedProjects();
    if (savedProjects.length > 0) {
      const lastProject = savedProjects[0];
      currentDirectory = lastProject.path;
      loadGitDiff();
    }
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
    background: #f5f5f5;
    margin: 0;
    padding: 0;
    line-height: 1.4;
    overflow-y: scroll;
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
    background: #f5f5f5;
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
    color: #333;
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
    background: #ffe6e6;
    border: 1px solid #ff6b6b;
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
    background: #fff;
    border-radius: 8px;
    border: 1px solid #ddd;
  }

  .no-changes h3 {
    margin: 0 0 0.5rem 0;
    color: #333;
  }

  .no-changes p {
    margin: 0 0 1.5rem 0;
    color: #666;
  }

  .welcome {
    text-align: center;
    padding: 4rem 2rem;
    background: #fff;
    border-radius: 8px;
    border: 1px solid #ddd;
  }

  .welcome h2 {
    margin: 0 0 1rem 0;
    color: #333;
  }

  .welcome p {
    margin: 0;
    color: #666;
    font-size: 1.1rem;
  }

  button {
    background: #0066cc;
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
    background: #0052a3;
  }

  .diff-header-wrapper {
    position: sticky;
    top: 72px; /* Height of header */
    background: #f5f5f5;
    z-index: 99;
    margin: 0 -1rem 1rem -1rem;
    padding: 1rem;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: #1a1a1a;
      color: #f6f6f6;
    }

    header {
      background: #1a1a1a;
    }

    h1 {
      color: #f6f6f6;
    }

    .diff-header-wrapper {
      background: #1a1a1a;
    }

    .welcome,
    .no-changes {
      background: #2a2a2a;
      border-color: #444;
    }

    .welcome h2,
    .welcome p,
    .no-changes h3,
    .no-changes p {
      color: #f6f6f6;
    }

    .error {
      background: #3a1f1f;
      border-color: #ff6b6b;
    }
  }
</style>
