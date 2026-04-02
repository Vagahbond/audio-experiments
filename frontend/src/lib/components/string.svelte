<script lang="ts">
	const { playing, onValueChanged } = $props();

	let length = $state(0);

	$effect(() => {
		onValueChanged(100000 - length + 2);
	});
</script>

<div class="container">
	<div class="graduations"></div>
	<div class="graduations-small"></div>
	<div class={`shape ${playing && 'playing'}`} style:--curve-start={`${length / 1000}%`}></div>
	<input
		class="slider"
		type="range"
		bind:value={length}
		min="0"
		max="100000"
		defaultValue="1"
		step="0"
	/>
</div>

<style>
	:root {
		--curve-target: 0;
		--curve-start: 0;
	}

	@keyframes vibrate {
		0% {
			clip-path: shape(
				from 0 50%,
				line to var(--curve-start) 50%,
				curve to 100% 50% with calc(50% + var(--curve-start) / 2) calc(100% - 5px),
				line to 100% calc(50% + 5px),
				curve to var(--curve-start) calc(50% + 5px) with calc(50% + var(--curve-start) / 2) 100%,
				line to 0 calc(50% + 5px),
				line to 0 50%
			);
		}

		50% {
			clip-path: shape(
				from 0 50%,
				line to var(--curve-start) 50%,
				curve to 100% 50% with calc(50% + var(--curve-start) / 2) 0%,
				line to 100% calc(50% + 5px),
				curve to var(--curve-start) calc(50% + 5px) with calc(50% + var(--curve-start) / 2) 5px,
				line to 0 calc(50% + 5px),
				line to 0 50%
			);
		}

		100% {
			clip-path: shape(
				from 0 50%,
				line to var(--curve-start) 50%,
				curve to 100% 50% with calc(50% + var(--curve-start) / 2) calc(100% - 5px),
				line to 100% calc(50% + 5px),
				curve to var(--curve-start) calc(50% + 5px) with calc(50% + var(--curve-start) / 2) 100%,
				line to 0 calc(50% + 5px),
				line to 0 50%
			);
		}
	}

	.shape.playing {
		animation: vibrate 0.1s infinite;
	}

	.shape {
		clip-path: shape(
			from 0 50%,
			line to 100% 50%,
			line to 100% calc(50% + 5px),
			line to 0 calc(50% + 5px),
			line to 0 50%
		);

		background: repeating-linear-gradient(
			70deg,
			var(--overlay2),
			var(--overlay2) 5px,
			/* Width of the grey stripe */ var(--overlay0) 5px,
			/* Start of the dark stripe */ var(--overlay0) 8px /* Width of the dark stripe (5px) */
		);
		position: absolute;
		inset: 0;
		width: 100%;
		height: 5em;
		transition: all 0.1s ease-in;
		z-index: 10;
	}

	.graduations {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 2em;
		background: repeating-linear-gradient(
			90deg,
			var(--overlay2),
			var(--overlay2) 0.1em,
			transparent 0.1em,
			transparent 9.9em,
			var(--overlay2) 9.9em,
			var(--overlay2) 10em
		);
		z-index: 1;
	}

	.graduations-small {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		height: 1em;
		background: repeating-linear-gradient(
			90deg,
			transparent,
			transparent 4.9em,
			var(--overlay2) 4.9em,
			var(--overlay2) 5.1em,
			transparent 5.1em,
			transparent 10em
		);
		z-index: 1;
	}

	.container {
		height: 5em;
		position: relative;
		margin: 0;
		padding: 0;
		background-color: var(--surface1);

		border-radius: 0.5em;
		color: var(--text);
		box-shadow: rgba(0, 0, 0, 0.1) 2px 2px 0px inset;

		overflow: hidden;
	}

	.slider {
		padding: 0;
		margin: 0;
		width: 100%;
		height: 100%;
		position: absolute;
		top: 0;
		background-color: transparent;
		z-index: 10;
	}
</style>
