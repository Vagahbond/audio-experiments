<script lang="ts">
	const { name, slug, desc } = $props();
</script>

<a class="card box" href="/experiments/{slug}">
	<h5>{name}</h5>
	<p>{desc}</p>
</a>

<style>
	.card {
		transition: all 0.2s ease-in-out;
		background-color: var(--surface0);
		border-radius: 0.5em;
		color: var(--text);
		text-decoration: none;
		cursor: pointer;
	}

	.card:hover {
		transform: translate(0.2em, 0.2em);
		box-shadow: none;
	}
</style>
