<script lang="ts">
	import type { LayoutMode } from '$lib/types/issue';

	interface Props {
		layoutMode: LayoutMode;
		onLayoutChange: (mode: LayoutMode) => void;
		onRefresh: () => void;
		isLoading: boolean;
		demoMode: boolean;
	}

	let { layoutMode, onLayoutChange, onRefresh, isLoading, demoMode }: Props = $props();
</script>

<div class="absolute left-4 top-4 z-10 flex items-center gap-2">
	<div class="flex overflow-hidden rounded-lg border border-gray-700 bg-gray-900">
		<button
			class="px-3 py-2 text-sm transition-colors {layoutMode === 'hierarchical'
				? 'bg-blue-600 text-white'
				: 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
			onclick={() => onLayoutChange('hierarchical')}
		>
			Hierarchical
		</button>
		<button
			class="px-3 py-2 text-sm transition-colors {layoutMode === 'force'
				? 'bg-blue-600 text-white'
				: 'text-gray-400 hover:bg-gray-800 hover:text-white'}"
			onclick={() => onLayoutChange('force')}
		>
			Force
		</button>
	</div>

	<button
		onclick={onRefresh}
		disabled={isLoading || demoMode}
		class="flex items-center gap-2 rounded-lg border border-gray-700 bg-gray-900 px-3 py-2 text-sm text-gray-400 transition-colors hover:bg-gray-800 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
	>
		<svg
			class="h-4 w-4 {isLoading ? 'animate-spin' : ''}"
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
		Refresh
	</button>

	{#if demoMode}
		<span class="rounded-lg border border-yellow-600 bg-yellow-600/20 px-3 py-2 text-sm text-yellow-400">
			Demo Mode
		</span>
	{/if}
</div>

<div class="absolute bottom-4 left-4 z-10 rounded-lg border border-gray-700 bg-gray-900 p-3">
	<h4 class="mb-2 text-xs font-semibold uppercase text-gray-500">Legend</h4>
	<div class="grid grid-cols-2 gap-x-4 gap-y-1 text-xs">
		<div class="flex items-center gap-2">
			<span class="h-3 w-3 rounded-full bg-gray-500"></span>
			<span class="text-gray-400">Open</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="h-3 w-3 rounded-full bg-yellow-500"></span>
			<span class="text-gray-400">In Progress</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="h-3 w-3 rounded-full bg-red-500"></span>
			<span class="text-gray-400">Blocked</span>
		</div>
		<div class="flex items-center gap-2">
			<span class="h-3 w-3 rounded-full bg-green-500"></span>
			<span class="text-gray-400">Closed</span>
		</div>
	</div>
	<div class="mt-2 border-t border-gray-700 pt-2">
		<div class="flex items-center gap-2 text-xs text-gray-500">
			<span class="inline-block h-3 w-3 rounded bg-gray-600"></span>
			<span>Epic = Box</span>
		</div>
		<div class="flex items-center gap-2 text-xs text-gray-500">
			<span class="inline-block h-3 w-3 rounded-full bg-gray-600"></span>
			<span>Other = Sphere</span>
		</div>
	</div>
</div>
