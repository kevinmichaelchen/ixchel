<script lang="ts">
	import { T } from '@threlte/core';
	import { Text } from '@threlte/extras';
	import type { Issue } from '$lib/types/issue';

	interface Props {
		issue: Issue;
		position: { x: number; y: number; z: number };
		isSelected: boolean;
		onClick: () => void;
	}

	let { issue, position, isSelected, onClick }: Props = $props();

	let isHovered = $state(false);

	const statusColors: Record<string, string> = {
		open: '#9ca3af',
		in_progress: '#fbbf24',
		blocked: '#ef4444',
		closed: '#22c55e'
	};

	const typeShapes: Record<string, 'sphere' | 'box'> = {
		epic: 'box',
		feature: 'sphere',
		task: 'sphere',
		bug: 'sphere',
		chore: 'sphere'
	};

	const prioritySizes: Record<string, number> = {
		'0': 1.8, 'Critical': 1.8,
		'1': 1.4, 'High': 1.4,
		'2': 1.0, 'Medium': 1.0,
		'3': 0.8, 'Low': 0.8,
		'4': 0.6, 'Backlog': 0.6
	};

	const color = $derived(statusColors[issue.status] || '#9ca3af');
	const size = $derived(prioritySizes[String(issue.priority)] ?? 1.0);
	const shape = $derived(typeShapes[issue.issue_type] || 'sphere');
	const scale = $derived(isSelected ? 1.3 : isHovered ? 1.15 : 1);
	const emissiveIntensity = $derived(isSelected ? 0.4 : isHovered ? 0.2 : 0);

	function truncateTitle(title: string, maxLen: number = 20): string {
		return title.length > maxLen ? title.slice(0, maxLen - 3) + '...' : title;
	}
</script>

<T.Group position={[position.x, position.y, position.z]}>
	{#if shape === 'box'}
		<T.Mesh
			{scale}
			onclick={onClick}
			onpointerenter={() => (isHovered = true)}
			onpointerleave={() => (isHovered = false)}
		>
			<T.BoxGeometry args={[size * 1.5, size, size]} />
			<T.MeshStandardMaterial {color} emissive={color} {emissiveIntensity} roughness={0.4} />
		</T.Mesh>
	{:else}
		<T.Mesh
			{scale}
			onclick={onClick}
			onpointerenter={() => (isHovered = true)}
			onpointerleave={() => (isHovered = false)}
		>
			<T.SphereGeometry args={[size, 32, 32]} />
			<T.MeshStandardMaterial {color} emissive={color} {emissiveIntensity} roughness={0.4} />
		</T.Mesh>
	{/if}

	{#if isHovered || isSelected}
		<Text
			text={truncateTitle(issue.title)}
			fontSize={0.5}
			position={[0, size + 0.8, 0]}
			color="white"
			anchorX="center"
			anchorY="bottom"
		/>
		<Text
			text={issue.id}
			fontSize={0.3}
			position={[0, -size - 0.3, 0]}
			color="#9ca3af"
			anchorX="center"
			anchorY="top"
		/>
	{/if}
</T.Group>
