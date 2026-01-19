<script lang="ts">
  import type { LayoutMode } from '$lib/types/issue';

  let showKeyboardShortcuts = $state(false);
  let showMoreMenu = $state(false);

  function handleExportGraph() {
    showMoreMenu = false;
    alert(
      'Export Graph: Coming soon!\n\nThis will export the current view as an image or data file.'
    );
  }

  function handleSettings() {
    showMoreMenu = false;
    alert('Settings: Coming soon!\n\nConfigure display options, colors, and behavior.');
  }

  function handleKeyboardShortcuts() {
    showMoreMenu = false;
    showKeyboardShortcuts = !showKeyboardShortcuts;
  }

  interface Props {
    layoutMode: LayoutMode;
    onLayoutChange: (mode: LayoutMode) => void;
    onRefresh: () => void;
    isLoading: boolean;
    demoMode: boolean;
  }

  let { layoutMode, onLayoutChange, onRefresh, isLoading, demoMode }: Props = $props();

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.more-menu-container')) {
      showMoreMenu = false;
    }
    if (!target.closest('.shortcuts-panel') && !target.closest('.shortcuts-trigger')) {
      showKeyboardShortcuts = false;
    }
  }

  // Global keyboard shortcuts
  function handleGlobalKeydown(e: KeyboardEvent) {
    // Ignore if typing in an input
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    switch (e.key) {
      case 'l':
      case 'L':
        onLayoutChange(layoutMode === 'hierarchical' ? 'force' : 'hierarchical');
        break;
      case 'r':
      case 'R':
        if (!demoMode && !isLoading) onRefresh();
        break;
      case '?':
        showKeyboardShortcuts = !showKeyboardShortcuts;
        break;
      case 'Escape':
        showKeyboardShortcuts = false;
        showMoreMenu = false;
        break;
    }
  }
</script>

<svelte:document onclick={handleClickOutside} onkeydown={handleGlobalKeydown} />

<div class="toolbar">
  <!-- Layout Toggle -->
  <div class="toggle-group">
    <button
      type="button"
      onclick={() => onLayoutChange('hierarchical')}
      class="toggle-btn"
      class:active={layoutMode === 'hierarchical'}
      title="Hierarchical layout (L)"
    >
      <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M4 5h16M4 12h16M4 19h16"
        />
      </svg>
      <span>Hierarchy</span>
    </button>
    <button
      type="button"
      onclick={() => onLayoutChange('force')}
      class="toggle-btn"
      class:active={layoutMode === 'force'}
      title="Force-directed layout (L)"
    >
      <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <circle cx="12" cy="12" r="3" stroke-width="2" />
        <circle cx="6" cy="6" r="2" stroke-width="2" />
        <circle cx="18" cy="6" r="2" stroke-width="2" />
        <circle cx="6" cy="18" r="2" stroke-width="2" />
        <circle cx="18" cy="18" r="2" stroke-width="2" />
        <path stroke-width="1.5" d="M9 10l-2-2M15 10l2-2M9 14l-2 2M15 14l2 2" />
      </svg>
      <span>Force</span>
    </button>
  </div>

  <!-- Refresh Button -->
  <button
    type="button"
    onclick={onRefresh}
    disabled={isLoading || demoMode}
    class="action-btn"
    title={demoMode ? 'Disabled in demo mode' : 'Refresh (R)'}
  >
    <svg
      class="icon"
      class:animate-spin={isLoading}
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
      />
    </svg>
    <span>Refresh</span>
  </button>

  <!-- More Menu -->
  <div class="more-menu-container">
    <button
      type="button"
      onclick={() => (showMoreMenu = !showMoreMenu)}
      class="icon-btn shortcuts-trigger"
      aria-label="More options"
    >
      <svg class="icon" fill="currentColor" viewBox="0 0 24 24">
        <circle cx="12" cy="5" r="2" />
        <circle cx="12" cy="12" r="2" />
        <circle cx="12" cy="19" r="2" />
      </svg>
    </button>

    {#if showMoreMenu}
      <div class="dropdown-menu">
        <button type="button" class="menu-item" onclick={handleExportGraph}>
          <svg class="menu-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
            />
          </svg>
          Export Graph
        </button>

        <button type="button" class="menu-item" onclick={handleSettings}>
          <svg class="menu-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
            />
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
          </svg>
          Settings
        </button>

        <div class="menu-separator"></div>

        <button type="button" class="menu-item" onclick={handleKeyboardShortcuts}>
          <svg class="menu-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"
            />
          </svg>
          <span>Keyboard Shortcuts</span>
          <kbd class="kbd">?</kbd>
        </button>
      </div>
    {/if}
  </div>

  {#if demoMode}
    <span class="demo-badge">Demo Mode</span>
  {/if}
</div>

<!-- Keyboard Shortcuts Panel -->
{#if showKeyboardShortcuts}
  <div class="shortcuts-panel">
    <div class="shortcuts-header">
      <h3>Keyboard Shortcuts</h3>
      <button
        type="button"
        onclick={() => (showKeyboardShortcuts = false)}
        class="close-btn"
        aria-label="Close"
      >
        <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          />
        </svg>
      </button>
    </div>
    <div class="shortcuts-list">
      <div class="shortcut-row">
        <span>Command palette</span>
        <kbd class="kbd">⌘K</kbd>
      </div>
      <div class="shortcut-row">
        <span>Next issue</span>
        <kbd class="kbd">J</kbd> / <kbd class="kbd">↓</kbd>
      </div>
      <div class="shortcut-row">
        <span>Previous issue</span>
        <kbd class="kbd">K</kbd> / <kbd class="kbd">↑</kbd>
      </div>
      <div class="shortcut-row">
        <span>Toggle layout</span>
        <kbd class="kbd">L</kbd>
      </div>
      <div class="shortcut-row">
        <span>Refresh issues</span>
        <kbd class="kbd">R</kbd>
      </div>
      <div class="shortcut-row">
        <span>Fit to view</span>
        <kbd class="kbd">0</kbd>
      </div>
      <div class="shortcut-row">
        <span>Close / Cancel</span>
        <kbd class="kbd">Esc</kbd>
      </div>
      <div class="shortcut-row">
        <span>Show shortcuts</span>
        <kbd class="kbd">?</kbd>
      </div>
    </div>
  </div>
{/if}

<style>
  .toolbar {
    position: absolute;
    left: 336px; /* Account for IssuePanel width */
    top: 16px;
    z-index: 10;
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toggle-group {
    display: flex;
    overflow: hidden;
    border-radius: 8px;
    border: 1px solid hsl(217 33% 20%);
    background: hsl(222 47% 9%);
  }

  .toggle-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    font-size: 12px;
    font-weight: 500;
    color: hsl(215 20% 55%);
    transition: all 0.15s;
  }

  .toggle-btn:hover {
    background: hsla(210, 40%, 98%, 0.05);
    color: hsl(210 40% 90%);
  }

  .toggle-btn.active {
    background: hsl(200 60% 20%);
    color: hsl(200 80% 90%);
  }

  .toggle-btn:first-child {
    border-right: 1px solid hsl(217 33% 18%);
  }

  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    font-size: 12px;
    font-weight: 500;
    color: hsl(215 20% 55%);
    background: hsl(222 47% 9%);
    border: 1px solid hsl(217 33% 20%);
    border-radius: 8px;
    transition: all 0.15s;
  }

  .action-btn:hover:not(:disabled) {
    background: hsl(217 33% 12%);
    color: hsl(210 40% 90%);
  }

  .action-btn:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  .icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    color: hsl(215 20% 55%);
    background: hsl(222 47% 9%);
    border: 1px solid hsl(217 33% 20%);
    border-radius: 8px;
    transition: all 0.15s;
  }

  .icon-btn:hover {
    background: hsl(217 33% 12%);
    color: hsl(210 40% 90%);
  }

  .icon {
    width: 16px;
    height: 16px;
  }

  .more-menu-container {
    position: relative;
  }

  .dropdown-menu {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    min-width: 200px;
    background: hsl(222 47% 9%);
    border: 1px solid hsl(217 33% 22%);
    border-radius: 10px;
    padding: 4px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    z-index: 50;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 8px 10px;
    font-size: 13px;
    color: hsl(210 40% 90%);
    border-radius: 6px;
    transition: background 0.1s ease;
  }

  .menu-item:hover {
    background: hsl(217 33% 15%);
  }

  .menu-icon {
    width: 16px;
    height: 16px;
    color: hsl(215 20% 50%);
  }

  .menu-separator {
    height: 1px;
    margin: 4px 0;
    background: hsl(217 33% 18%);
  }

  .kbd {
    margin-left: auto;
    padding: 2px 6px;
    font-size: 11px;
    font-family: ui-monospace, monospace;
    color: hsl(215 20% 55%);
    background: hsl(217 33% 12%);
    border: 1px solid hsl(217 33% 22%);
    border-radius: 4px;
  }

  .demo-badge {
    padding: 6px 12px;
    font-size: 11px;
    font-weight: 600;
    color: hsl(45 90% 60%);
    background: hsla(45, 90%, 50%, 0.15);
    border: 1px solid hsla(45, 80%, 50%, 0.3);
    border-radius: 6px;
  }

  .shortcuts-panel {
    position: absolute;
    top: 60px;
    right: 16px;
    z-index: 20;
    min-width: 260px;
    background: hsl(222 47% 9%);
    border: 1px solid hsl(217 33% 22%);
    border-radius: 12px;
    padding: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
  }

  .shortcuts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .shortcuts-header h3 {
    font-size: 14px;
    font-weight: 600;
    color: hsl(210 40% 98%);
  }

  .close-btn {
    padding: 4px;
    color: hsl(215 20% 50%);
    border-radius: 4px;
    transition: all 0.15s;
  }

  .close-btn:hover {
    color: hsl(210 40% 98%);
    background: hsl(217 33% 15%);
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 12px;
    color: hsl(215 20% 60%);
  }

  @keyframes spin {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .animate-spin {
    animation: spin 1s linear infinite;
  }
</style>
