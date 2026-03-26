<script lang="ts">
	interface SlotProps {
		label?: string;
		children: any;
	}
	const { children, label }: SlotProps = $props();
</script>

<div class="slot box">
	{#if label}
		<div class="label">
			<span>{label}</span>
		</div>
	{/if}
	<div class="slot-controls">
		{@render children()}
	</div>

	<div id="screw-1" class="screw"></div>
	<div id="screw-2" class="screw"></div>
	<div id="screw-3" class="screw"></div>
	<div id="screw-4" class="screw"></div>
</div>

<style>
	.screw {
		width: 0.25em;
		height: 0.25em;
		background-color: var(--overlay2);
		border-radius: 0.5em;
		position: absolute;
	}

	#screw-1 {
		top: 0.2em;
		left: 0.2em;
	}

	#screw-2 {
		top: 0.2em;
		right: calc(0.2em + 2px);
	}

	#screw-3 {
		bottom: calc(0.2em + 2px);
		left: 0.2em;
	}

	#screw-4 {
		bottom: calc(0.2em + 2px);
		right: calc(0.2em + 2px);
	}

	.slot {
		background-color: var(--surface1);
		display: flex;
		padding: 0.5em;
		display: flex;
		flex-direction: column;
		width: 100%;
		/*
		border: 1px groove var(--surface2);
*/
		border-radius: 0.5em;
		margin-top: 0.5em;
		margin-bottom: 0.5em;
		position: relative;
	}

	.label {
		margin-bottom: 0.5em;
		font-size: 0.8em;
		width: 100%;
		align-items: start;
	}

	.slot-controls {
		width: 100%;
		display: flex;
		flex-direction: row;
		justify-content: space-around;
	}
</style>
