<script lang="ts">
	import type { Issue, Status, IssueType } from '$lib/types/issue';

	interface Props {
		issues: Issue[];
		selectedIssue: Issue | null;
		onSelectIssue: (issue: Issue) => void;
	}

	let { issues, selectedIssue, onSelectIssue }: Props = $props();

	let searchQuery = $state('');
	let statusFilter = $state<Status | null>(null);
	let typeFilter = $state<IssueType | null>(null);

	const statuses: Status[] = ['open', 'in_progress', 'blocked', 'closed'];
	const issueTypes: IssueType[] = ['epic', 'feature', 'task', 'bug', 'chore'];

	const statusConfig: Record<Status, { color: string; bg: string; glow: string; label: string }> = {
		open: { color: '#64748b', bg: 'hsla(215, 16%, 47%, 0.2)', glow: '0 0 8px hsla(215, 16%, 47%, 0.5)', label: 'Open' },
		in_progress: { color: '#d97706', bg: 'hsla(38, 92%, 50%, 0.2)', glow: '0 0 8px hsla(38, 92%, 50%, 0.5)', label: 'In Progress' },
		blocked: { color: '#e11d48', bg: 'hsla(350, 89%, 60%, 0.2)', glow: '0 0 8px hsla(350, 89%, 60%, 0.5)', label: 'Blocked' },
		closed: { color: '#059669', bg: 'hsla(160, 84%, 39%, 0.2)', glow: '0 0 8px hsla(160, 84%, 39%, 0.5)', label: 'Closed' }
	};

	const typeConfig: Record<IssueType, { icon: string; color: string; bg: string; glow: string; label: string }> = {
		epic: { icon: '◆', color: '#a855f7', bg: 'hsla(270, 91%, 65%, 0.2)', glow: '0 0 8px hsla(270, 91%, 65%, 0.5)', label: 'Epic' },
		feature: { icon: '★', color: '#3b82f6', bg: 'hsla(217, 91%, 60%, 0.2)', glow: '0 0 8px hsla(217, 91%, 60%, 0.5)', label: 'Feature' },
		task: { icon: '●', color: '#6b7280', bg: 'hsla(220, 9%, 46%, 0.2)', glow: '0 0 8px hsla(220, 9%, 46%, 0.5)', label: 'Task' },
		bug: { icon: '⚠', color: '#ef4444', bg: 'hsla(0, 84%, 60%, 0.2)', glow: '0 0 8px hsla(0, 84%, 60%, 0.5)', label: 'Bug' },
		chore: { icon: '⚙', color: '#78716c', bg: 'hsla(30, 6%, 45%, 0.2)', glow: '0 0 8px hsla(30, 6%, 45%, 0.5)', label: 'Chore' }
	};

	const priorityLabels: Record<string, string> = {
		'0': 'P0', '1': 'P1', '2': 'P2', '3': 'P3', '4': 'P4',
		Critical: 'P0', High: 'P1', Medium: 'P2', Low: 'P3', Backlog: 'P4'
	};

	const priorityToNumber: Record<string, number> = {
		'0': 0, 'Critical': 0,
		'1': 1, 'High': 1,
		'2': 2, 'Medium': 2,
		'3': 3, 'Low': 3,
		'4': 4, 'Backlog': 4
	};

	function getPriorityNum(p: number | string): number {
		return priorityToNumber[String(p)] ?? 2;
	}

	const filteredIssues = $derived(
		issues
			.filter((i) => {
				if (searchQuery) {
					const query = searchQuery.toLowerCase();
					return (
						i.title.toLowerCase().includes(query) ||
						i.id.toLowerCase().includes(query) ||
						i.labels.some((l) => l.toLowerCase().includes(query))
					);
				}
				return true;
			})
			.filter((i) => !statusFilter || i.status === statusFilter)
			.filter((i) => !typeFilter || i.issue_type === typeFilter)
			.sort((a, b) => {
				// Sort epics first, then by priority
				if (a.issue_type === 'epic' && b.issue_type !== 'epic') return -1;
				if (b.issue_type === 'epic' && a.issue_type !== 'epic') return 1;
				return getPriorityNum(a.priority) - getPriorityNum(b.priority);
			})
	);

	function toggleStatusFilter(status: Status) {
		statusFilter = statusFilter === status ? null : status;
	}

	function toggleTypeFilter(type: IssueType) {
		typeFilter = typeFilter === type ? null : type;
	}

	function clearFilters() {
		statusFilter = null;
		typeFilter = null;
		searchQuery = '';
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			clearFilters();
		}
	}
</script>

<aside class="issue-panel" onkeydown={handleKeydown}>
	<header class="panel-header">
		<div class="header-row">
			<h2 class="panel-title">Issues</h2>
			{#if statusFilter || typeFilter || searchQuery}
				<button class="clear-btn" onclick={clearFilters} title="Clear filters">
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
					</svg>
				</button>
			{/if}
		</div>

		<input
			type="search"
			placeholder="Search issues..."
			bind:value={searchQuery}
			class="search-input"
		/>

		<!-- Status filters -->
		<div class="filter-group">
			<span class="filter-label">Status</span>
			<div class="filter-chips">
				{#each statuses as status}
					{@const config = statusConfig[status]}
					<button
						type="button"
						class="filter-chip"
						class:active={statusFilter === status}
						onclick={() => toggleStatusFilter(status)}
						style="--chip-color: {config.color}; --chip-bg: {config.bg}; --chip-glow: {config.glow};"
					>
						<span class="status-dot" style="background: {config.color};"></span>
						{config.label}
					</button>
				{/each}
			</div>
		</div>

		<!-- Type filters -->
		<div class="filter-group">
			<span class="filter-label">Type</span>
			<div class="filter-chips">
				{#each issueTypes as type}
					{@const config = typeConfig[type]}
					<button
						type="button"
						class="filter-chip"
						class:active={typeFilter === type}
						onclick={() => toggleTypeFilter(type)}
						style="--chip-color: {config.color}; --chip-bg: {config.bg}; --chip-glow: {config.glow};"
					>
						<span class="type-icon">{config.icon}</span>
						{config.label}
					</button>
				{/each}
			</div>
		</div>
	</header>

	<div class="issue-list">
		{#if filteredIssues.length === 0}
			<div class="empty-state">
				<p>No issues found</p>
				{#if statusFilter || typeFilter || searchQuery}
					<button class="reset-btn" onclick={clearFilters}>Clear filters</button>
				{/if}
			</div>
		{:else}
			{#each filteredIssues as issue (issue.id)}
				<button
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
						<span class="status-badge" style="background: {statusConfig[issue.status].color};">
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
		<span>{filteredIssues.length} of {issues.length} issues</span>
	</footer>
</aside>

<style>
	.issue-panel {
		position: relative;
		z-index: 20;
		display: flex;
		flex-direction: column;
		width: 320px;
		height: 100%;
		background: hsl(222 47% 7%);
		border-right: 1px solid hsl(217 33% 18%);
	}

	.panel-header {
		padding: 16px;
		border-bottom: 1px solid hsl(217 33% 15%);
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.header-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.panel-title {
		font-size: 16px;
		font-weight: 600;
		color: hsl(210 40% 98%);
	}

	.clear-btn {
		padding: 4px;
		color: hsl(215 20% 50%);
		border-radius: 4px;
		transition: all 0.15s;
	}

	.clear-btn:hover {
		color: hsl(210 40% 98%);
		background: hsl(217 33% 15%);
	}

	.search-input {
		width: 100%;
		padding: 8px 12px;
		background: hsl(222 47% 10%);
		border: 1px solid hsl(217 33% 18%);
		border-radius: 6px;
		color: hsl(210 40% 98%);
		font-size: 13px;
	}

	.search-input::placeholder {
		color: hsl(215 20% 40%);
	}

	.search-input:focus {
		outline: none;
		border-color: hsl(200 80% 50%);
	}

	.filter-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.filter-label {
		font-size: 11px;
		font-weight: 500;
		color: hsl(215 20% 45%);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.filter-chips {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
	}

	.filter-chip {
		display: flex;
		align-items: center;
		gap: 4px;
		padding: 4px 8px;
		font-size: 11px;
		color: hsl(215 20% 55%);
		background: hsl(222 47% 10%);
		border: 1px solid hsl(217 33% 18%);
		border-radius: 4px;
		transition: all 0.15s;
	}

	.filter-chip:hover {
		background: hsl(217 33% 15%);
		color: hsl(210 40% 98%);
	}

	.filter-chip.active {
		background: var(--chip-bg);
		border-color: var(--chip-color);
		box-shadow: var(--chip-glow);
		color: var(--chip-color);
	}

	.filter-chip.active .type-icon {
		color: var(--chip-color);
		opacity: 1;
	}

	.status-dot {
		width: 6px;
		height: 6px;
		border-radius: 50%;
	}

	.type-icon {
		font-size: 10px;
		opacity: 0.7;
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

	.reset-btn {
		font-size: 12px;
		color: hsl(200 80% 60%);
	}

	.reset-btn:hover {
		text-decoration: underline;
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
