<script lang="ts">
	import './root.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="title box crt">
	<h1>Vagahbond's music experiments stash</h1>
	<!-- URL to that repo on github   -->
	<a class="gh-link" href="https://github.com/vagahbond/audio-experiments">
		<img
			src="https://img.shields.io/badge/GitHub-vagahbond%2Faudio--experiments-black?style=for-the-badge&logo=github&logoColor=#df8e1d"
			alt="GitHub badge"
		/>
	</a>
</div>
{@render children()}

<style>
	.title {
		text-align: center;
		color: var(--base);
		background-color: var(--text);
	}

	.gh-link {
		display: flex;
		justify-content: center;
		align-items: center;
		margin: 1em;
	}
</style>
