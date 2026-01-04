<script lang="ts">
	import { T, useTask } from '@threlte/core';
	import { OrbitControls, interactivity } from '@threlte/extras';
	import { Vector3 } from 'three';
	import TaskGraph from './TaskGraph.svelte';
	import type { Issue, LayoutMode } from '$lib/types/issue';

	interactivity();

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

	let cameraRef = $state<THREE.PerspectiveCamera | null>(null);
	let controlsRef = $state<any>(null);
	let targetPosition = $state<Vector3 | null>(null);

	let nodePositions = $state<Map<string, { x: number; y: number; z: number }>>(new Map());

	function handleNodePositionsUpdate(positions: Map<string, { x: number; y: number; z: number }>) {
		nodePositions = positions;
	}

	$effect(() => {
		if (focusedIssueId && nodePositions.has(focusedIssueId)) {
			const pos = nodePositions.get(focusedIssueId)!;
			targetPosition = new Vector3(pos.x, pos.y, pos.z + 15);
		}
	});

	useTask((delta) => {
		if (!targetPosition || !cameraRef || !controlsRef) return;

		const currentPos = cameraRef.position;
		const distance = currentPos.distanceTo(targetPosition);

		if (distance < 0.1) {
			targetPosition = null;
			onCameraAnimationComplete();
			return;
		}

		const lerpFactor = Math.min(1, delta * 3);
		cameraRef.position.lerp(targetPosition, lerpFactor);

		const nodePos = focusedIssueId ? nodePositions.get(focusedIssueId) : null;
		if (nodePos && controlsRef.target) {
			const targetLookAt = new Vector3(nodePos.x, nodePos.y, nodePos.z);
			controlsRef.target.lerp(targetLookAt, lerpFactor);
		}
	});
</script>

<T.PerspectiveCamera
	makeDefault
	position={[0, 5, 30]}
	fov={50}
	bind:ref={cameraRef}
	oncreate={(ref) => ref.lookAt(0, 0, 0)}
/>

<OrbitControls
	bind:ref={controlsRef}
	enableDamping
	dampingFactor={0.1}
	minDistance={5}
	maxDistance={100}
/>

<T.AmbientLight intensity={0.6} />
<T.DirectionalLight position={[10, 20, 10]} intensity={1} />
<T.DirectionalLight position={[-10, -10, -10]} intensity={0.3} />

<TaskGraph
	{issues}
	{layoutMode}
	{selectedIssue}
	{onSelectIssue}
	onPositionsUpdate={handleNodePositionsUpdate}
/>

<T.GridHelper args={[100, 50, '#374151', '#1f2937']} rotation.x={Math.PI / 2} position.z={-5} />
