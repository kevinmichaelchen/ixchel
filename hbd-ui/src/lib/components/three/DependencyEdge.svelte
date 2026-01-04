<script lang="ts">
	import { T } from '@threlte/core';
	import { MeshLineGeometry, MeshLineMaterial } from '@threlte/extras';
	import { Vector3 } from 'three';
	import type { DepType } from '$lib/types/issue';

	interface Props {
		from: { x: number; y: number; z: number };
		to: { x: number; y: number; z: number };
		depType: DepType;
	}

	let { from, to, depType }: Props = $props();

	const depColors: Record<DepType, string> = {
		blocks: '#ef4444',
		waits_for: '#f59e0b',
		related: '#6b7280'
	};

	const depWidths: Record<DepType, number> = {
		blocks: 0.08,
		waits_for: 0.06,
		related: 0.04
	};

	const color = $derived(depColors[depType]);
	const width = $derived(depWidths[depType]);

	const points = $derived([
		new Vector3(from.x, from.y, from.z),
		new Vector3(to.x, to.y, to.z)
	]);
</script>

<T.Mesh>
	<MeshLineGeometry {points} />
	<MeshLineMaterial {width} {color} opacity={0.7} transparent />
</T.Mesh>
