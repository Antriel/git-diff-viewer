<script>
  import { open } from "@tauri-apps/plugin-shell";
  import { setupModalHandlers } from "./utils.js";

  let { show = $bindable() } = $props();
  let overlayElement = $state();
  let modalElement = $state();

  function closeModal() {
    show = false;
  }

  $effect(() => {
    if (show && overlayElement) {
      const cleanup = setupModalHandlers({
        onClose: closeModal,
        overlayElement: overlayElement,
        enableEscapeClose: true,
        enableOverlayClose: true,
      });

      return cleanup;
    }
  });
</script>

<div class="about-btn-container">
  <button class="btn-secondary transition-base" onclick={() => (show = true)}>About</button>
</div>

{#if show}
  <div
    bind:this={overlayElement}
    class="modal-overlay"
    tabindex="-1"
    role="button"
  >
    <div class="modal-content">
      <div class="modal-header">
        <h2>About Git Diff Viewer</h2>
        <button class="btn-icon close-btn" onclick={closeModal}>×</button>
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
                class="btn-link"
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
            class="btn-primary repo-btn"
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
    background: var(--modal-bg);
    color: var(--text-color);
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
    color: var(--text-color);
    font-size: 1.4rem;
  }

  .close-btn {
    font-size: 1.5rem;
    color: var(--text-muted);
  }

  .close-btn:hover {
    color: var(--text-color);
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
    color: var(--text-color);
    font-size: 0.85rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .value {
    color: var(--text-muted);
    font-size: 0.95rem;
  }


  .description {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border-light);
  }

  .description-text {
    margin: 0.5rem 0 0 0;
    color: var(--text-muted);
    line-height: 1.5;
    font-size: 0.95rem;
  }

  .links {
    text-align: center;
  }

  .repo-btn {
    padding: 0.75rem 1.5rem;
    border-radius: 8px;
    font-weight: 500;
    font-size: 0.95rem;
  }

  .repo-btn:hover {
    transform: translateY(-1px);
  }

  .btn-link {
    text-align: left;
  }

  @media (max-width: 480px) {
    .info-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }
</style>
