<script>
  import { open } from "@tauri-apps/plugin-dialog";
  import { addClickOutsideListener } from "./utils.js";

  let { currentDirectory = "", onDirectorySelected = () => {} } = $props();

  let recentProjects = $state([]);
  let showRecent = $state(false);
  let dropdownRef = $state();

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
    closeDropdown();
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

  function closeDropdown() {
    showRecent = false;
  }

  $effect(() => {
    // Load recent projects when component mounts
    loadRecentProjects();
  });

  $effect(() => {
    if (showRecent && dropdownRef) {
      const cleanup = addClickOutsideListener(dropdownRef, closeDropdown);
      return cleanup;
    }
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
    <button onclick={selectDirectory} class="btn-primary">
      Select Directory
    </button>

    {#if recentProjects.length > 0}
      <div class="recent-dropdown" bind:this={dropdownRef}>
        <button onclick={toggleRecent} class="btn-secondary"> Recent ▼ </button>

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
    color: var(--text-muted);
    white-space: nowrap;
  }

  .path {
    font-family: monospace;
    background: var(--component-bg-hover);
    color: var(--text-color);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.9rem;
    word-break: break-all;
  }

  .no-path {
    color: var(--text-muted);
    font-style: italic;
  }

  .controls {
    display: flex;
    gap: 0.5rem;
    position: relative;
  }

  button {
    white-space: nowrap;
  }

  .recent-dropdown {
    position: relative;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    right: 0;
    background: var(--modal-bg);
    border: 1px solid var(--border-light);
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
    border-bottom: 1px solid var(--border-light);
    cursor: pointer;
    font-size: 0.85rem;
  }

  .recent-item:last-child {
    border-bottom: none;
  }

  .recent-item:hover {
    background: var(--component-bg-hover);
  }

  .project-name {
    font-weight: 600;
    color: var(--text-color);
    margin-bottom: 0.25rem;
  }

  .project-path {
    font-family: monospace;
    color: var(--text-muted);
    font-size: 0.8rem;
    margin-bottom: 0.25rem;
  }

  .project-date {
    color: var(--text-muted);
    font-size: 0.75rem;
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
