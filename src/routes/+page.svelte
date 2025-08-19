<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DiffViewer from "../lib/DiffViewer.svelte";
  import DiffHeader from "../lib/DiffHeader.svelte";
  import DirectorySelector from "../lib/DirectorySelector.svelte";

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

  async function loadGitDiff(
    directory,
    contextLines = null,
    source = null,
    target = null
  ) {
    if (!directory) return;

    // Use current context size if not explicitly provided
    const actualContextLines =
      contextLines !== null ? contextLines : currentContextSize;

    // Use current comparison values if not explicitly provided
    const actualSource = source !== null ? source : comparisonSource;
    const actualTarget = target !== null ? target : comparisonTarget;

    loading = true;
    error = "";

    try {
      const result = await invoke("get_git_diff", {
        directoryPath: directory,
        contextLines: actualContextLines,
        includeUntracked: includeUntracked,
        comparisonSource: actualSource,
        comparisonTarget: actualTarget,
      });
      gitDiffResult = result;

      // Save to localStorage for project history
      const savedProjects = JSON.parse(
        localStorage.getItem("gitDiffProjects") || "[]"
      );
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

      // Keep only the last 10 projects
      if (savedProjects.length > 10) {
        savedProjects.splice(10);
      }

      localStorage.setItem("gitDiffProjects", JSON.stringify(savedProjects));
    } catch (err) {
      if (err instanceof Error) {
        error = err.message;
      } else {
        error = String(err);
      }
      gitDiffResult = null;
    } finally {
      loading = false;
    }
  }

  function handleDirectorySelected(event) {
    currentDirectory = event.detail.directory;
    loadGitDiff(currentDirectory);
  }

  function handleRefresh() {
    if (currentDirectory) {
      loadGitDiff(currentDirectory);
    }
  }

  function handleContextChange(event) {
    currentContextSize = event.detail.contextLines;
    if (currentDirectory) {
      loadGitDiff(currentDirectory, event.detail.contextLines);
    }
  }

  function handleSearch(event) {
    searchTerm = event.detail.term;
  }

  function handleVisibleCountChange(event) {
    visibleCount = event.detail.count;
    totalCount = event.detail.total;
  }

  function handleUntrackedToggle(event) {
    console.log(event.detail);
    includeUntracked = event.detail.includeUntracked;
    if (currentDirectory) {
      loadGitDiff(currentDirectory);
    }
  }

  function handleComparisonChange(event) {
    comparisonSource = event.detail.source;
    comparisonTarget = event.detail.target;
    if (currentDirectory) {
      loadGitDiff(currentDirectory, null, comparisonSource, comparisonTarget);
    }
  }

  onMount(() => {
    // Load the last opened project
    const savedProjects = JSON.parse(
      localStorage.getItem("gitDiffProjects") || "[]"
    );
    if (savedProjects.length > 0) {
      const lastProject = savedProjects[0];
      currentDirectory = lastProject.path;
      loadGitDiff(currentDirectory);
    }
  });
</script>

<main class="container">
  <header>
    <h1>Git Diff Viewer</h1>
    <DirectorySelector
      {currentDirectory}
      on:directorySelected={handleDirectorySelected}
    />
  </header>

  {#if currentDirectory}
    <div class="diff-header-wrapper">
      <DiffHeader
        {gitDiffResult}
        contextSize={currentContextSize}
        {searchTerm}
        {visibleCount}
        totalCount={gitDiffResult?.hunks?.length || 0}
        {includeUntracked}
        {currentDirectory}
        on:search={handleSearch}
        on:refresh={handleRefresh}
        on:contextChange={handleContextChange}
        on:untrackedToggle={handleUntrackedToggle}
        on:comparisonChange={handleComparisonChange}
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
      {searchTerm}
      {currentDirectory}
      on:visibleCountChange={handleVisibleCountChange}
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
