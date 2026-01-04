<script lang="ts">
  import type { Issue, Status, IssueType } from '$lib/types/issue';

  interface Props {
    issues: Issue[];
    selectedIssue: Issue | null;
    onSelectIssue: (issue: Issue) => void;
  }

  let { issues, selectedIssue, onSelectIssue }: Props = $props();

  const statusConfig: Record<Status, { color: string; bg: string; glow: string; label: string }> = {
    open: {
      color: '#64748b',
      bg: 'hsla(215, 16%, 47%, 0.2)',
      glow: '0 0 8px hsla(215, 16%, 47%, 0.5)',
      label: 'Open',
    },
    in_progress: {
      color: '#d97706',
      bg: 'hsla(38, 92%, 50%, 0.2)',
      glow: '0 0 8px hsla(38, 92%, 50%, 0.5)',
      label: 'In Progress',
    },
    blocked: {
      color: '#e11d48',
      bg: 'hsla(350, 89%, 60%, 0.2)',
      glow: '0 0 8px hsla(350, 89%, 60%, 0.5)',
      label: 'Blocked',
    },
    closed: {
      color: '#059669',
      bg: 'hsla(160, 84%, 39%, 0.2)',
      glow: '0 0 8px hsla(160, 84%, 39%, 0.5)',
      label: 'Closed',
    },
  };

  const typeConfig: Record<
    IssueType,
    { icon: string; color: string; bg: string; glow: string; label: string }
  > = {
    epic: {
      icon: '◆',
      color: '#a855f7',
      bg: 'hsla(270, 91%, 65%, 0.2)',
      glow: '0 0 8px hsla(270, 91%, 65%, 0.5)',
      label: 'Epic',
    },
    feature: {
      icon: '★',
      color: '#3b82f6',
      bg: 'hsla(217, 91%, 60%, 0.2)',
      glow: '0 0 8px hsla(217, 91%, 60%, 0.5)',
      label: 'Feature',
    },
    task: {
      icon: '●',
      color: '#6b7280',
      bg: 'hsla(220, 9%, 46%, 0.2)',
      glow: '0 0 8px hsla(220, 9%, 46%, 0.5)',
      label: 'Task',
    },
    bug: {
      icon: '⚠',
      color: '#ef4444',
      bg: 'hsla(0, 84%, 60%, 0.2)',
      glow: '0 0 8px hsla(0, 84%, 60%, 0.5)',
      label: 'Bug',
    },
    chore: {
      icon: '⚙',
      color: '#78716c',
      bg: 'hsla(30, 6%, 45%, 0.2)',
      glow: '0 0 8px hsla(30, 6%, 45%, 0.5)',
      label: 'Chore',
    },
  };

  const priorityLabels: Record<string, string> = {
    '0': 'P0',
    '1': 'P1',
    '2': 'P2',
    '3': 'P3',
    '4': 'P4',
    Critical: 'P0',
    High: 'P1',
    Medium: 'P2',
    Low: 'P3',
    Backlog: 'P4',
  };
</script>

<div class="issue-panel">
  <div class="issue-list">
    {#if issues.length === 0}
      <div class="empty-state">
        <p>No issues found</p>
      </div>
    {:else}
      {#each issues as issue (issue.id)}
        <button
          type="button"
          class="issue-card"
          class:selected={selectedIssue?.id === issue.id}
          onclick={() => onSelectIssue(issue)}
        >
          <div class="issue-header">
            <span class="issue-type" title={issue.issue_type}>
              {typeConfig[issue.issue_type]?.icon || '●'}
            </span>
            <code class="issue-id">{issue.id.slice(0, 8)}</code>
            <span class="issue-priority">{priorityLabels[String(issue.priority)] ?? 'P2'}</span>
          </div>
          <div class="issue-title">{issue.title}</div>
          <div class="issue-footer">
            <span class="status-badge" style:background={statusConfig[issue.status].color}>
              {statusConfig[issue.status].label}
            </span>
            {#if issue.assignee}
              <span class="issue-assignee">@{issue.assignee.slice(0, 10)}</span>
            {/if}
          </div>
        </button>
      {/each}
    {/if}
  </div>

  <footer class="panel-footer">
    <span>{issues.length} issues</span>
  </footer>
</div>

<style>
  .issue-panel {
    position: relative;
    z-index: 20;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    background: hsl(222 47% 7%);
  }

  .issue-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 32px 16px;
    color: hsl(215 20% 45%);
    font-size: 13px;
  }

  .issue-card {
    width: 100%;
    padding: 12px;
    margin-bottom: 6px;
    background: hsl(222 47% 9%);
    border: 1px solid hsl(217 33% 15%);
    border-radius: 8px;
    text-align: left;
    transition: all 0.15s;
  }

  .issue-card:hover {
    background: hsl(217 33% 12%);
    border-color: hsl(217 33% 22%);
  }

  .issue-card.selected {
    background: hsl(200 40% 12%);
    border-color: hsl(200 60% 35%);
  }

  .issue-header {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 6px;
  }

  .issue-type {
    font-size: 12px;
    color: hsl(215 20% 50%);
  }

  .issue-id {
    font-family: ui-monospace, monospace;
    font-size: 10px;
    color: hsl(215 20% 45%);
    background: hsl(217 33% 12%);
    padding: 2px 6px;
    border-radius: 3px;
  }

  .issue-priority {
    margin-left: auto;
    font-size: 10px;
    font-weight: 600;
    color: hsl(215 20% 50%);
  }

  .issue-title {
    font-size: 13px;
    font-weight: 500;
    color: hsl(210 40% 95%);
    line-height: 1.4;
    margin-bottom: 8px;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .issue-footer {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .status-badge {
    font-size: 10px;
    padding: 2px 6px;
    border-radius: 3px;
    color: white;
  }

  .issue-assignee {
    font-family: ui-monospace, monospace;
    font-size: 10px;
    color: hsl(215 20% 45%);
  }

  .panel-footer {
    padding: 12px 16px;
    border-top: 1px solid hsl(217 33% 15%);
    font-size: 11px;
    color: hsl(215 20% 45%);
    text-align: center;
  }
</style>
