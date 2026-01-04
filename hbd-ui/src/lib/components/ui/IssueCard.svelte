<script lang="ts">
	import type { Issue } from '$lib/types/issue';

	interface Props {
		issue: Issue;
		selected: boolean;
		onclick: () => void;
	}

	let { issue, selected, onclick }: Props = $props();

	const statusColors: Record<string, string> = {
		open: 'bg-gray-500',
		in_progress: 'bg-yellow-500',
		blocked: 'bg-red-500',
		closed: 'bg-green-500'
	};

	const typeIcons: Record<string, string> = {
		epic: '📦',
		feature: '✨',
		task: '📋',
		bug: '🐛',
		chore: '🔧'
	};

	const priorityLabels: Record<string, string> = {
		'0': 'P0', 'Critical': 'P0',
		'1': 'P1', 'High': 'P1',
		'2': 'P2', 'Medium': 'P2',
		'3': 'P3', 'Low': 'P3',
		'4': 'P4', 'Backlog': 'P4'
	};

	const labels = $derived(issue.labels ?? []);
</script>

<button
	class="w-full border-b border-gray-800 p-3 text-left transition-colors hover:bg-gray-800 {selected
		? 'bg-gray-800'
		: ''}"
	{onclick}
>
	<div class="flex items-start gap-2">
		<span class="text-lg" title={issue.issue_type}>{typeIcons[issue.issue_type] || '📋'}</span>

		<div class="min-w-0 flex-1">
			<p class="truncate text-sm font-medium text-white">{issue.title}</p>

			<div class="mt-1 flex items-center gap-2 text-xs text-gray-500">
				<code class="rounded bg-gray-800 px-1">{issue.id}</code>
				<span class="rounded px-1.5 py-0.5 text-white {statusColors[issue.status]}">
					{issue.status.replace('_', ' ')}
				</span>
				<span class="text-gray-600">{priorityLabels[String(issue.priority)] ?? issue.priority}</span>
			</div>

			{#if labels.length > 0}
				<div class="mt-1.5 flex flex-wrap gap-1">
					{#each labels.slice(0, 3) as label}
						<span class="rounded bg-gray-700 px-1.5 py-0.5 text-xs text-gray-300">{label}</span>
					{/each}
					{#if labels.length > 3}
						<span class="text-xs text-gray-500">+{labels.length - 3}</span>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</button>
