<script lang="ts">
	import type { Issue, Status } from '$lib/types/issue';
	import IssueCard from './IssueCard.svelte';

	interface Props {
		issues: Issue[];
		selectedIssue: Issue | null;
		onSelectIssue: (issue: Issue) => void;
	}

	let { issues, selectedIssue, onSelectIssue }: Props = $props();

	let searchQuery = $state('');
	let statusFilter = $state<Status | null>(null);
	let showEpicsOnly = $state(true);

	const statuses: Status[] = ['open', 'in_progress', 'blocked', 'closed'];

	const filteredIssues = $derived(
		issues
			.filter((i) => {
				if (searchQuery) {
					const query = searchQuery.toLowerCase();
					return i.title.toLowerCase().includes(query) || i.id.toLowerCase().includes(query);
				}
				return true;
			})
			.filter((i) => !statusFilter || i.status === statusFilter)
			.filter((i) => !showEpicsOnly || i.issue_type === 'epic')
			.sort((a, b) => a.priority - b.priority)
	);

	function getStatusColor(status: Status): string {
		const colors: Record<Status, string> = {
			open: 'bg-gray-500',
			in_progress: 'bg-yellow-500',
			blocked: 'bg-red-500',
			closed: 'bg-green-500'
		};
		return colors[status];
	}
</script>

<aside class="flex h-full w-80 flex-col border-r border-gray-700 bg-gray-900">
	<header class="space-y-3 border-b border-gray-700 p-4">
		<h2 class="text-lg font-semibold text-white">Issues</h2>

		<input
			type="search"
			placeholder="Search issues..."
			bind:value={searchQuery}
			class="w-full rounded bg-gray-800 px-3 py-2 text-sm text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
		/>

		<div class="flex flex-wrap gap-1">
			{#each statuses as status}
				<button
					class="rounded px-2 py-1 text-xs transition-colors {statusFilter === status
						? 'bg-blue-600 text-white'
						: 'bg-gray-800 text-gray-400 hover:bg-gray-700'}"
					onclick={() => (statusFilter = statusFilter === status ? null : status)}
				>
					{status.replace('_', ' ')}
				</button>
			{/each}
		</div>

		<label class="flex items-center gap-2 text-sm text-gray-400">
			<input type="checkbox" bind:checked={showEpicsOnly} class="rounded" />
			Epics only
		</label>
	</header>

	<div class="flex-1 overflow-y-auto">
		{#if filteredIssues.length === 0}
			<p class="p-4 text-center text-sm text-gray-500">No issues found</p>
		{:else}
			{#each filteredIssues as issue (issue.id)}
				<IssueCard
					{issue}
					selected={selectedIssue?.id === issue.id}
					onclick={() => onSelectIssue(issue)}
				/>
			{/each}
		{/if}
	</div>

	<footer class="border-t border-gray-700 p-3 text-center text-xs text-gray-500">
		{filteredIssues.length} of {issues.length} issues
	</footer>
</aside>
