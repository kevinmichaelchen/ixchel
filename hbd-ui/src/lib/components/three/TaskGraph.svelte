<script lang="ts">
	import { useTask } from '@threlte/core';
	import {
		forceSimulation,
		forceLink,
		forceManyBody,
		forceCenter,
		forceCollide
	} from 'd3-force-3d';
	import TaskNode from './TaskNode.svelte';
	import DependencyEdge from './DependencyEdge.svelte';
	import type { Issue, LayoutMode, GraphNode, GraphEdge } from '$lib/types/issue';

	interface Props {
		issues: Issue[];
		layoutMode: LayoutMode;
		selectedIssue: Issue | null;
		onSelectIssue: (issue: Issue) => void;
		onPositionsUpdate: (positions: Map<string, { x: number; y: number; z: number }>) => void;
	}

	let { issues, layoutMode, selectedIssue, onSelectIssue, onPositionsUpdate }: Props = $props();

	let nodes = $state<GraphNode[]>([]);
	let edges = $state<GraphEdge[]>([]);
	let simulation = $state<any>(null);

	const issueMap = $derived(new Map(issues.map((i) => [i.id, i])));

	const priorityToNumber: Record<string, number> = {
		'0': 0, 'Critical': 0,
		'1': 1, 'High': 1,
		'2': 2, 'Medium': 2,
		'3': 3, 'Low': 3,
		'4': 4, 'Backlog': 4
	};

	function getPriorityNum(p: number | string): number {
		return priorityToNumber[String(p)] ?? 2;
	}

	function computeHierarchicalLayout(issueList: Issue[]): GraphNode[] {
		const epics = issueList.filter((i) => i.issue_type === 'epic');
		const epicIds = new Set(epics.map((e) => e.id));

		const childrenByParent = new Map<string, Issue[]>();
		const orphans: Issue[] = [];

		for (const issue of issueList) {
			if (epicIds.has(issue.id)) continue;

			if (issue.parent_id && issueMap.has(issue.parent_id)) {
				const children = childrenByParent.get(issue.parent_id) || [];
				children.push(issue);
				childrenByParent.set(issue.parent_id, children);
			} else {
				orphans.push(issue);
			}
		}

		const result: GraphNode[] = [];
		const spacing = { x: 8, y: -6, z: 4 };

		epics.forEach((epic, epicIndex) => {
			const epicX = (epicIndex - (epics.length - 1) / 2) * spacing.x * 3;
			result.push({
				id: epic.id,
				issue: epic,
				x: epicX,
				y: 0,
				z: 0
			});

			const children = childrenByParent.get(epic.id) || [];
			children.sort((a, b) => getPriorityNum(a.priority) - getPriorityNum(b.priority));

			children.forEach((child, childIndex) => {
				const childX = epicX + (childIndex - (children.length - 1) / 2) * spacing.x;
				result.push({
					id: child.id,
					issue: child,
					x: childX,
					y: spacing.y,
					z: (getPriorityNum(child.priority) - 2) * spacing.z
				});
			});
		});

		orphans.sort((a, b) => getPriorityNum(a.priority) - getPriorityNum(b.priority));
		const orphanStartX = epics.length > 0 ? (epics.length / 2 + 1) * spacing.x * 3 : 0;

		orphans.forEach((orphan, index) => {
			result.push({
				id: orphan.id,
				issue: orphan,
				x: orphanStartX + (index % 5) * spacing.x,
				y: spacing.y * 2,
				z: Math.floor(index / 5) * spacing.z
			});
		});

		return result;
	}

	function buildEdges(nodeList: GraphNode[]): GraphEdge[] {
		const nodeIds = new Set(nodeList.map((n) => n.id));
		const result: GraphEdge[] = [];

		for (const node of nodeList) {
			const deps = node.issue.depends_on ?? [];
			for (const dep of deps) {
				if (nodeIds.has(dep.id)) {
					result.push({
						source: dep.id,
						target: node.id,
						dep_type: dep.dep_type
					});
				}
			}
		}

		return result;
	}

	function initForceSimulation(nodeList: GraphNode[], edgeList: GraphEdge[]) {
		const sim = forceSimulation(nodeList, 3)
			.force(
				'link',
				forceLink(edgeList)
					.id((d: any) => d.id)
					.distance(12)
					.strength(0.5)
			)
			.force('charge', forceManyBody().strength(-80))
			.force('center', forceCenter(0, 0, 0))
			.force('collide', forceCollide().radius(3))
			.alphaDecay(0.02)
			.velocityDecay(0.3);

		return sim;
	}

	$effect(() => {
		const hierarchicalNodes = computeHierarchicalLayout(issues);

		if (layoutMode === 'hierarchical') {
			nodes = hierarchicalNodes;
			edges = buildEdges(nodes);
			if (simulation) {
				simulation.stop();
				simulation = null;
			}
		} else {
			const forceNodes = hierarchicalNodes.map((n) => ({
				...n,
				x: n.x + (Math.random() - 0.5) * 2,
				y: n.y + (Math.random() - 0.5) * 2,
				z: n.z + (Math.random() - 0.5) * 2
			}));
			edges = buildEdges(forceNodes);
			simulation = initForceSimulation(forceNodes, edges);
			nodes = forceNodes;
		}
	});

	useTask(() => {
		if (simulation && layoutMode === 'force') {
			simulation.tick();
			nodes = [...simulation.nodes()];
		}

		const positions = new Map<string, { x: number; y: number; z: number }>();
		for (const node of nodes) {
			positions.set(node.id, { x: node.x, y: node.y, z: node.z });
		}
		onPositionsUpdate(positions);
	});

	function getNodePosition(nodeId: string): { x: number; y: number; z: number } | null {
		const node = nodes.find((n) => n.id === nodeId);
		return node ? { x: node.x, y: node.y, z: node.z } : null;
	}
</script>

{#each nodes as node (node.id)}
	<TaskNode
		issue={node.issue}
		position={{ x: node.x, y: node.y, z: node.z }}
		isSelected={selectedIssue?.id === node.id}
		onClick={() => onSelectIssue(node.issue)}
	/>
{/each}

{#each edges as edge, i (i)}
	{@const sourcePos = getNodePosition(typeof edge.source === 'string' ? edge.source : edge.source.id)}
	{@const targetPos = getNodePosition(typeof edge.target === 'string' ? edge.target : edge.target.id)}
	{#if sourcePos && targetPos}
		<DependencyEdge from={sourcePos} to={targetPos} depType={edge.dep_type} />
	{/if}
{/each}
