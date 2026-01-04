<script lang="ts">
	import type { Issue, Status, Priority } from '$lib/types/issue';

	interface Props {
		open: boolean;
		position: { x: number; y: number };
		issue: Issue | null;
		onClose: () => void;
		onStatusChange: (issueId: string, status: Status) => void;
		onPriorityChange: (issueId: string, priority: Priority) => void;
		onAddDependency: (issueId: string) => void;
		onOpenInEditor: (issueId: string) => void;
	}

	let {
		open = $bindable(false),
		position,
		issue,
		onClose,
		onStatusChange,
		onPriorityChange,
		onAddDependency,
		onOpenInEditor
	}: Props = $props();

	let expandedSection = $state<'status' | 'priority' | null>(null);

	const statuses: { value: Status; label: string; color: string }[] = [
		{ value: 'open', label: 'Open', color: 'bg-gray-500' },
		{ value: 'in_progress', label: 'In Progress', color: 'bg-yellow-500' },
		{ value: 'blocked', label: 'Blocked', color: 'bg-red-500' },
		{ value: 'closed', label: 'Closed', color: 'bg-green-500' }
	];

	const priorities: { value: Priority; label: string }[] = [
		{ value: 0, label: 'P0 - Critical' },
		{ value: 1, label: 'P1 - High' },
		{ value: 2, label: 'P2 - Medium' },
		{ value: 3, label: 'P3 - Low' },
		{ value: 4, label: 'P4 - Backlog' }
	];

	function handleStatusSelect(status: Status) {
		if (issue) {
			onStatusChange(issue.id, status);
		}
		closeMenu();
	}

	function handlePrioritySelect(priority: Priority) {
		if (issue) {
			onPriorityChange(issue.id, priority);
		}
		closeMenu();
	}

	function handleAddDependency() {
		if (issue) {
			onAddDependency(issue.id);
		}
		closeMenu();
	}

	function handleOpenInEditor() {
		if (issue) {
			onOpenInEditor(issue.id);
		}
		closeMenu();
	}

	function closeMenu() {
		expandedSection = null;
		onClose();
	}

	function toggleSection(section: 'status' | 'priority') {
		expandedSection = expandedSection === section ? null : section;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape' && open) {
			closeMenu();
		}
	}

	function handleClickOutside() {
		if (open) {
			closeMenu();
		}
	}

	function getPriorityNum(p: number | string): number {
		const map: Record<string, number> = {
			'0': 0, 'Critical': 0,
			'1': 1, 'High': 1,
			'2': 2, 'Medium': 2,
			'3': 3, 'Low': 3,
			'4': 4, 'Backlog': 4
		};
		return map[String(p)] ?? 2;
	}

	const currentPriority = $derived(issue ? getPriorityNum(issue.priority) : 2);

	$effect(() => {
		if (!open) {
			expandedSection = null;
		}
	});
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && issue}
	<button
		class="fixed inset-0 z-40"
		onclick={handleClickOutside}
		aria-label="Close context menu"
	></button>

	<div
		class="fixed z-50 min-w-[220px] rounded-xl border border-border-card bg-background-alt shadow-popover"
		style="left: {position.x}px; top: {position.y}px;"
		role="menu"
	>
		<div class="border-b border-border-card px-3 py-2">
			<div class="text-xs font-medium text-foreground-alt">{issue.id}</div>
			<div class="max-w-[200px] truncate text-sm font-medium text-foreground">{issue.title}</div>
		</div>

		<div class="p-1">
			<button
				class="flex w-full cursor-pointer items-center justify-between rounded-lg px-3 py-2 text-sm text-foreground outline-none hover:bg-muted"
				onclick={() => toggleSection('status')}
			>
				<span class="flex items-center gap-2">
					<svg class="h-4 w-4 text-foreground-alt" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<circle cx="12" cy="12" r="10" stroke-width="2" />
						<path stroke-linecap="round" stroke-width="2" d="M12 6v6l4 2" />
					</svg>
					Change Status
				</span>
				<svg
					class="h-4 w-4 text-foreground-alt transition-transform {expandedSection === 'status' ? 'rotate-90' : ''}"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
				</svg>
			</button>

			{#if expandedSection === 'status'}
				<div class="ml-6 border-l border-border-card pl-2">
					{#each statuses as status}
						{#if status.value !== issue.status}
							<button
								class="flex w-full cursor-pointer items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-foreground outline-none hover:bg-muted"
								onclick={() => handleStatusSelect(status.value)}
							>
								<span class="h-2 w-2 rounded-full {status.color}"></span>
								{status.label}
							</button>
						{/if}
					{/each}
				</div>
			{/if}

			<button
				class="flex w-full cursor-pointer items-center justify-between rounded-lg px-3 py-2 text-sm text-foreground outline-none hover:bg-muted"
				onclick={() => toggleSection('priority')}
			>
				<span class="flex items-center gap-2">
					<svg class="h-4 w-4 text-foreground-alt" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4h13M3 8h9m-9 4h6m4 0l4-4m0 0l4 4m-4-4v12" />
					</svg>
					Change Priority
				</span>
				<svg
					class="h-4 w-4 text-foreground-alt transition-transform {expandedSection === 'priority' ? 'rotate-90' : ''}"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
				</svg>
			</button>

			{#if expandedSection === 'priority'}
				<div class="ml-6 border-l border-border-card pl-2">
					{#each priorities as priority}
						{#if priority.value !== currentPriority}
							<button
								class="flex w-full cursor-pointer items-center gap-2 rounded-lg px-3 py-1.5 text-sm text-foreground outline-none hover:bg-muted"
								onclick={() => handlePrioritySelect(priority.value)}
							>
								{priority.label}
							</button>
						{/if}
					{/each}
				</div>
			{/if}

			<div class="my-1 h-px bg-border-card"></div>

			<button
				class="flex w-full cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-sm text-foreground outline-none hover:bg-muted"
				onclick={handleAddDependency}
			>
				<svg class="h-4 w-4 text-foreground-alt" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1" />
				</svg>
				Add Dependency
			</button>

			<button
				class="flex w-full cursor-pointer items-center gap-2 rounded-lg px-3 py-2 text-sm text-foreground outline-none hover:bg-muted"
				onclick={handleOpenInEditor}
			>
				<svg class="h-4 w-4 text-foreground-alt" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
				</svg>
				Open in Editor
			</button>
		</div>
	</div>
{/if}
