<script lang="ts">
	import '../app.css';

	let { children } = $props();
</script>

<svelte:head>
	<title>hbd-ui | Task Graph Visualizer</title>
</svelte:head>

{@render children()}
