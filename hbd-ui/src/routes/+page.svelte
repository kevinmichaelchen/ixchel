<script lang="ts">
	import type { PageData } from './$types';
	import Canvas from '$lib/components/Canvas.svelte';
	import IssuePanel from '$lib/components/ui/IssuePanel.svelte';
	import DetailPanel from '$lib/components/ui/DetailPanel.svelte';
	import Toolbar from '$lib/components/ui/Toolbar.svelte';
	import type { Issue, LayoutMode } from '$lib/types/issue';

	let { data }: { data: PageData } = $props();

	let localIssues = $state<Issue[] | null>(null);
	let selectedIssue = $state<Issue | null>(null);
	let layoutMode = $state<LayoutMode>('hierarchical');
	let focusedIssueId = $state<string | null>(null);
	let isLoading = $state(false);

	const issues = $derived(localIssues ?? data.issues);
	const demoMode = $derived(data.demoMode);
	const error = $derived(data.error);

	async function handleRefresh() {
		if (demoMode) return;
		isLoading = true;
		try {
			const response = await fetch('?_data=routes/+page');
			const newData = await response.json();
			localIssues = newData.issues;
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
</script>

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

		<Canvas
			{issues}
			{layoutMode}
			{selectedIssue}
			{focusedIssueId}
			onSelectIssue={handleSelectIssue}
			onCameraAnimationComplete={handleCameraAnimationComplete}
		/>

		{#if selectedIssue}
			<DetailPanel issue={selectedIssue} onClose={handleCloseDetail} />
		{/if}
	</main>
</div>
