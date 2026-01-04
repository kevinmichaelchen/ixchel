<script lang="ts">
	import type { PageData } from './$types';
	import { invalidateAll } from '$app/navigation';
	import IssuePanel from '$lib/components/ui/IssuePanel.svelte';
	import DetailPanel from '$lib/components/ui/DetailPanel.svelte';
	import Toolbar from '$lib/components/ui/Toolbar.svelte';
	import CommandPalette from '$lib/components/ui/CommandPalette.svelte';
	import NodeContextMenu from '$lib/components/ui/NodeContextMenu.svelte';
	import type { Issue, LayoutMode, Status, Priority } from '$lib/types/issue';

	let { data }: { data: PageData } = $props();

	let selectedIssue = $state<Issue | null>(null);
	let layoutMode = $state<LayoutMode>('hierarchical');
	let focusedIssueId = $state<string | null>(null);
	let isLoading = $state(false);

	let contextMenuOpen = $state(false);
	let contextMenuPosition = $state({ x: 0, y: 0 });
	let contextMenuIssue = $state<Issue | null>(null);

	const issues = $derived(data.issues);
	const demoMode = $derived(data.demoMode);
	const error = $derived(data.error);

	async function handleRefresh() {
		if (demoMode) return;
		isLoading = true;
		try {
			await invalidateAll();
		} finally {
			isLoading = false;
		}
	}

	function handleSelectIssue(issue: Issue) {
		selectedIssue = issue;
		focusedIssueId = issue.id;
	}

	function handleCloseDetail() {
		selectedIssue = null;
	}

	function handleCameraAnimationComplete() {
		focusedIssueId = null;
	}

	function handleLayoutChange(mode: LayoutMode) {
		layoutMode = mode;
	}

	function handleNodeContextMenu(event: MouseEvent, issue: Issue) {
		contextMenuIssue = issue;
		contextMenuPosition = { x: event.clientX, y: event.clientY };
		contextMenuOpen = true;
	}

	function handleContextMenuClose() {
		contextMenuOpen = false;
		contextMenuIssue = null;
	}

	async function handleStatusChange(issueId: string, status: Status) {
		if (demoMode) return;
		isLoading = true;
		try {
			await fetch(`/api/issues/${issueId}/status`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ status })
			});
			await handleRefresh();
		} finally {
			isLoading = false;
		}
	}

	async function handlePriorityChange(issueId: string, priority: Priority) {
		if (demoMode) return;
		isLoading = true;
		try {
			await fetch(`/api/issues/${issueId}/priority`, {
				method: 'PATCH',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ priority })
			});
			await handleRefresh();
		} finally {
			isLoading = false;
		}
	}

	function handleAddDependency(issueId: string) {
		alert(`Add dependency: Coming soon!\n\nUse command line:\nhbd dep add ${issueId} <target-id> --type blocks`);
	}

	function handleOpenInEditor(issueId: string) {
		alert(`Open in editor: Coming soon!\n\nUse command line:\nhbd edit ${issueId}`);
	}

	// Global keyboard navigation
	function handleKeydown(e: KeyboardEvent) {
		// Ignore if typing in an input or command palette is open
		if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

		switch (e.key) {
			case 'j':
			case 'ArrowDown':
				e.preventDefault();
				// Navigate to next issue
				if (issues.length > 0) {
					const selectedId = selectedIssue?.id;
					const currentIndex = selectedId ? issues.findIndex((i) => i.id === selectedId) : -1;
					const nextIndex = currentIndex < issues.length - 1 ? currentIndex + 1 : 0;
					handleSelectIssue(issues[nextIndex]);
				}
				break;
			case 'k':
			case 'ArrowUp':
				e.preventDefault();
				// Navigate to previous issue
				if (issues.length > 0) {
					const selectedId = selectedIssue?.id;
					const currentIndex = selectedId ? issues.findIndex((i) => i.id === selectedId) : 0;
					const prevIndex = currentIndex > 0 ? currentIndex - 1 : issues.length - 1;
					handleSelectIssue(issues[prevIndex]);
				}
				break;
			case 'Escape':
				// Close detail panel
				if (selectedIssue) {
					handleCloseDetail();
				}
				break;
		}
	}
</script>

<svelte:document onkeydown={handleKeydown} />

<div class="flex h-screen w-screen overflow-hidden">
	<IssuePanel {issues} {selectedIssue} onSelectIssue={handleSelectIssue} />

	<main class="relative flex-1">
		<Toolbar
			{layoutMode}
			onLayoutChange={(mode) => (layoutMode = mode)}
			onRefresh={handleRefresh}
			{isLoading}
			{demoMode}
		/>

		{#if error}
			<div class="absolute left-1/2 top-16 z-10 -translate-x-1/2 rounded bg-yellow-600 px-4 py-2 text-sm">
				{error}
			</div>
		{/if}

		<div class="flex h-full items-center justify-center text-foreground-alt">
			<p class="text-center">
				<span class="text-2xl">Graph view coming soon</span>
				<br />
				<span class="text-sm">Use the issue panel to browse issues</span>
			</p>
		</div>

		{#if selectedIssue}
			<DetailPanel issue={selectedIssue} onClose={handleCloseDetail} />
		{/if}
	</main>
</div>

<CommandPalette
	{issues}
	{selectedIssue}
	{layoutMode}
	onSelectIssue={handleSelectIssue}
	onLayoutChange={handleLayoutChange}
	onRefresh={handleRefresh}
/>

<NodeContextMenu
	bind:open={contextMenuOpen}
	position={contextMenuPosition}
	issue={contextMenuIssue}
	onClose={handleContextMenuClose}
	onStatusChange={handleStatusChange}
	onPriorityChange={handlePriorityChange}
	onAddDependency={handleAddDependency}
	onOpenInEditor={handleOpenInEditor}
/>
