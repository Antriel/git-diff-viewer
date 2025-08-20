<script>
  import { open } from "@tauri-apps/plugin-shell";

  let { show = $bindable() } = $props();
</script>

<div class="about-btn-container">
  <button class="about-btn" onclick={() => (show = true)}>About</button>
</div>

{#if show}
  <div class="modal-overlay" onclick={() => (show = false)}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>About Git Diff Viewer</h2>
        <button class="close-btn" onclick={() => (show = false)}>×</button>
      </div>
      <div class="modal-body">
        <div class="description">
          <span class="label">Description</span>
          <p class="description-text">{__APP_DESCRIPTION__}</p>
        </div>
        <div class="info-grid">
          <div class="info-section">
            <div class="info-item">
              <span class="label">Version</span>
              <span class="value">{__APP_VERSION__}</span>
            </div>
            <div class="info-item">
              <span class="label">License</span>
              <span class="value">{__APP_LICENSE__}</span>
            </div>
          </div>

          <div class="info-section">
            <div class="info-item">
              <span class="label">Author</span>
              <button
                class="link-btn"
                onclick={() => open("https://antriel.com")}
              >
                {__APP_AUTHOR__}
              </button>
            </div>
            <div class="info-item">
              <span class="label">Collaborator</span>
              <span class="value">Claude (AI Assistant)</span>
            </div>
          </div>
        </div>

        <div class="links">
          <button
            class="repo-btn"
            onclick={() =>
              open(__APP_REPOSITORY__.replace("git+", "").replace(".git", ""))}
          >
            📦 GitHub Repository
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .about-btn-container {
    display: contents;
  }

  .about-btn {
    background: #666;
    color: white;
    border: none;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    font-size: 0.8rem;
    transition: background-color 0.2s;
  }

  .about-btn:hover {
    background: #555;
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal-content {
    background: white;
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 1.5rem 0 1.5rem;
  }

  .modal-header h2 {
    margin: 0;
    color: #333;
    font-size: 1.4rem;
  }

  .close-btn {
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: #666;
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
  }

  .close-btn:hover {
    background: #f0f0f0;
    color: #333;
  }

  .modal-body {
    padding: 1.5rem;
  }

  .info-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .info-section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .label {
    font-weight: 600;
    color: #333;
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .value {
    color: #555;
    font-size: 0.95rem;
  }

  .link-btn {
    background: none;
    border: none;
    color: #0066cc;
    cursor: pointer;
    padding: 0;
    text-align: left;
    font-size: 0.95rem;
    text-decoration: underline;
    font-family: inherit;
  }

  .link-btn:hover {
    color: #0052a3;
  }

  .description {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid #eee;
  }

  .description-text {
    margin: 0.5rem 0 0 0;
    color: #555;
    line-height: 1.5;
    font-size: 0.95rem;
  }

  .links {
    text-align: center;
  }

  .repo-btn {
    background: #0066cc;
    color: white;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 500;
    font-size: 0.95rem;
    transition: all 0.2s;
    font-family: inherit;
  }

  .repo-btn:hover {
    background: #0052a3;
    transform: translateY(-1px);
  }

  @media (max-width: 480px) {
    .info-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }

  @media (prefers-color-scheme: dark) {
    .modal-content {
      background: #2a2a2a;
      color: #f6f6f6;
    }

    .modal-header h2 {
      color: #f6f6f6;
    }

    .close-btn {
      color: #ccc;
    }

    .close-btn:hover {
      background: #3a3a3a;
      color: #f6f6f6;
    }

    .label {
      color: #f6f6f6;
    }

    .value,
    .description-text {
      color: #ccc;
    }

    .link-btn {
      color: #66b3ff;
    }

    .link-btn:hover {
      color: #99ccff;
    }

    .description {
      border-color: #444;
    }

    .repo-btn {
      background: #66b3ff;
      color: #1a1a1a;
    }

    .repo-btn:hover {
      background: #99ccff;
    }
  }
</style>
