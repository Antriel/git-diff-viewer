<script>
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DiffViewer from "../lib/DiffViewer.svelte";
  import DirectorySelector from "../lib/DirectorySelector.svelte";

  let currentDirectory = $state("");
  let gitDiffResult = $state(null);
  let loading = $state(false);
  let error = $state("");

  async function loadGitDiff(directory) {
    if (!directory) return;
    
    loading = true;
    error = "";
    
    try {
      const result = await invoke("get_git_diff", { directoryPath: directory });
      gitDiffResult = result;
      
      // Save to localStorage for project history
      const savedProjects = JSON.parse(localStorage.getItem('gitDiffProjects') || '[]');
      const existing = savedProjects.find(p => p.path === directory);
      
      if (existing) {
        existing.lastOpened = new Date().toISOString();
      } else {
        savedProjects.unshift({
          path: directory,
          name: directory.split(/[/\\]/).pop() || directory,
          lastOpened: new Date().toISOString()
        });
      }
      
      // Keep only the last 10 projects
      if (savedProjects.length > 10) {
        savedProjects.splice(10);
      }
      
      localStorage.setItem('gitDiffProjects', JSON.stringify(savedProjects));
    } catch (err) {
      error = err.toString();
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

  onMount(() => {
    // Load the last opened project
    const savedProjects = JSON.parse(localStorage.getItem('gitDiffProjects') || '[]');
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
      {currentDirectory}
      on:refresh={handleRefresh}
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
      <p>Select a git repository to view and search through your uncommitted changes.</p>
    </div>
  {/if}
</main>

<style>
  :global(body) {
    font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, "Liberation Mono", monospace;
    background: #f5f5f5;
    margin: 0;
    padding: 0;
    line-height: 1.4;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
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
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
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

  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: #1a1a1a;
      color: #f6f6f6;
    }

    h1 {
      color: #f6f6f6;
    }

    .welcome, .no-changes {
      background: #2a2a2a;
      border-color: #444;
    }

    .welcome h2, .welcome p, .no-changes h3, .no-changes p {
      color: #f6f6f6;
    }

    .error {
      background: #3a1f1f;
      border-color: #ff6b6b;
    }
  }
</style>
