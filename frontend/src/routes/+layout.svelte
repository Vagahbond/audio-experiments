<script lang="ts">
	import './root.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<a href="/"><h1 class="title box crt">Vagahbond's audio experiments stash</h1> </a>
{@render children()}

<style>
	.title {
		text-align: center;
		color: var(--base);
		background-color: var(--text);
		text-decoration: none;
	}

	a {
		text-decoration: none;
	}
</style>
