<script lang="ts">
	import { Canvas } from '@threlte/core';
	import Scene from './three/Scene.svelte';
	import type { Issue, LayoutMode } from '$lib/types/issue';

	interface Props {
		issues: Issue[];
		layoutMode: LayoutMode;
		selectedIssue: Issue | null;
		focusedIssueId: string | null;
		onSelectIssue: (issue: Issue) => void;
		onCameraAnimationComplete: () => void;
	}

	let {
		issues,
		layoutMode,
		selectedIssue,
		focusedIssueId,
		onSelectIssue,
		onCameraAnimationComplete
	}: Props = $props();
</script>

<div class="h-full w-full">
	<Canvas>
		<Scene
			{issues}
			{layoutMode}
			{selectedIssue}
			{focusedIssueId}
			{onSelectIssue}
			{onCameraAnimationComplete}
		/>
	</Canvas>
</div>
