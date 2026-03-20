<script lang="ts">
	import './root.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<h1 class="title box crt">Vagahbond's music experiments stash</h1>
{@render children()}

<style>
	.title {
		text-align: center;
		color: var(--base);
		background-color: var(--text);
	}
</style>
