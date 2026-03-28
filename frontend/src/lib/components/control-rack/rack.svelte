<script lang="ts">
	interface RackProps {
		label?: string;
		children: any;
	}

	const { children, label } = $props();
</script>

<div class="rack box">
	{#if label}
		<h3>{label}</h3>
	{/if}

	{@render children()}
</div>

<style>
	h3 {
		background-color: var(--surface1);
		border-style: none;
		border-width: 1px;
		border-color: var(--overlay2);
		padding: 0.5em;
		border-radius: 0.5em;
		color: var(--text);
		box-shadow: rgba(0, 0, 0, 0.1) 2px 2px 0px inset;
	}

	.rack {
		min-width: 10em;
		margin: 1em;
		width: min-content;
		background-color: var(--surface0);
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
