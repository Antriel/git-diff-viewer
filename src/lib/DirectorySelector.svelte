<script>
  import { open } from "@tauri-apps/plugin-dialog";

  let { currentDirectory = "", onDirectorySelected = () => {} } = $props();

  let recentProjects = $state([]);
  let showRecent = $state(false);

  async function selectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Git Repository Directory",
      });

      if (selected) {
        onDirectorySelected({ directory: selected });
      }
    } catch (error) {
      console.error("Failed to select directory:", error);
    }
  }

  function loadRecentProjects() {
    const saved = JSON.parse(localStorage.getItem("gitDiffProjects") || "[]");
    recentProjects = saved.slice(0, 5); // Show only the 5 most recent
  }

  function selectRecentProject(project) {
    onDirectorySelected({ directory: project.path });
    showRecent = false;
  }

  function toggleRecent() {
    if (!showRecent) {
      loadRecentProjects();
    }
    showRecent = !showRecent;
  }

  function formatPath(path) {
    const maxLength = 50;
    if (path.length <= maxLength) return path;
    return "..." + path.slice(-(maxLength - 3));
  }

  $effect(() => {
    // Load recent projects when component mounts
    loadRecentProjects();
  });
</script>

<div class="directory-selector">
  <div class="current-path">
    {#if currentDirectory}
      <span class="path-label">Current:</span>
      <span class="path" title={currentDirectory}
        >{formatPath(currentDirectory)}</span
      >
    {:else}
      <span class="no-path">No directory selected</span>
    {/if}
  </div>

  <div class="controls">
    <button onclick={selectDirectory} class="primary">
      Select Directory
    </button>

    {#if recentProjects.length > 0}
      <div class="recent-dropdown">
        <button onclick={toggleRecent} class="secondary"> Recent ▼ </button>

        {#if showRecent}
          <div class="dropdown-menu">
            {#each recentProjects as project}
              <button
                onclick={() => selectRecentProject(project)}
                class="recent-item"
                title={project.path}
              >
                <div class="project-name">{project.name}</div>
                <div class="project-path">{formatPath(project.path)}</div>
                <div class="project-date">
                  {new Date(project.lastOpened).toLocaleDateString()}
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .directory-selector {
    display: flex;
    align-items: center;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .current-path {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    flex: 1;
  }

  .path-label {
    font-weight: 600;
    color: #666;
    white-space: nowrap;
  }

  .path {
    font-family: monospace;
    background: #e8e8e8;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.9rem;
    word-break: break-all;
  }

  .no-path {
    color: #999;
    font-style: italic;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
    position: relative;
  }

  button {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    font-size: 0.9rem;
    transition: all 0.2s;
    white-space: nowrap;
  }

  .primary {
    background: #0066cc;
    color: white;
  }

  .primary:hover {
    background: #0052a3;
  }

  .secondary {
    background: #f0f0f0;
    color: #333;
    border: 1px solid #ccc;
  }

  .secondary:hover {
    background: #e0e0e0;
  }

  .recent-dropdown {
    position: relative;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    right: 0;
    background: white;
    border: 1px solid #ccc;
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 10;
    min-width: 300px;
    max-width: 400px;
  }

  .recent-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 0.75rem;
    background: none;
    border: none;
    border-bottom: 1px solid #eee;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .recent-item:last-child {
    border-bottom: none;
  }

  .recent-item:hover {
    background: #f8f9fa;
  }

  .project-name {
    font-weight: 600;
    color: #333;
    margin-bottom: 0.25rem;
  }

  .project-path {
    font-family: monospace;
    color: #666;
    font-size: 0.8rem;
    margin-bottom: 0.25rem;
  }

  .project-date {
    color: #999;
    font-size: 0.75rem;
  }

  @media (prefers-color-scheme: dark) {
    .path {
      background: #3a3a3a;
      color: #f6f6f6;
    }

    .path-label {
      color: #ccc;
    }

    .no-path {
      color: #888;
    }

    .secondary {
      background: #3a3a3a;
      color: #f6f6f6;
      border-color: #555;
    }

    .secondary:hover {
      background: #4a4a4a;
    }

    .dropdown-menu {
      background: #2a2a2a;
      border-color: #555;
    }

    .recent-item {
      border-color: #444;
    }

    .recent-item:hover {
      background: #3a3a3a;
    }

    .project-name {
      color: #f6f6f6;
    }

    .project-path {
      color: #ccc;
    }

    .project-date {
      color: #999;
    }
  }

  @media (max-width: 768px) {
    .directory-selector {
      flex-direction: column;
      align-items: stretch;
    }

    .current-path {
      order: 2;
    }

    .controls {
      order: 1;
      justify-content: center;
    }

    .dropdown-menu {
      left: 0;
      right: 0;
    }
  }
</style>
