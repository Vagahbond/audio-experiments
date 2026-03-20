<script lang="ts">
	import type { MidiEvent } from '$lib/midi';

	interface PianoProps {
		onKeyDown: (event: MidiEvent) => void;
	}

	// 2 octaves on the keyboard
	const { onKeyDown }: PianoProps = $props();

	const maxOctave = 9;
	const minOctave = 0;
	let octave = $state(5);

	function keyDown(index: number) {
		const key = index + 12 * octave;

		if (key >= 0 && key <= 127) {
			onKeyDown({ key });
		}
	}
</script>

<div class="piano box">
	<div class="controls">
		<div class="octave">
			<span class="octave-number">Octave: {octave}</span>

			<button class="octave-up button" disabled={octave >= maxOctave} onclick={() => (octave += 1)}
				>↑
			</button>

			<button
				class="octave-down button"
				disabled={octave <= minOctave}
				onclick={() => (octave -= 1)}
				>↓
			</button>
		</div>
	</div>
	<div class="keys">
		<button onmousedown={(_) => keyDown(0)} class="key white">Do</button>
		<button onmousedown={(_) => keyDown(1)} aria-label="Do#" class="key black"></button>
		<button onmousedown={(_) => keyDown(2)} class="key white">Re</button>
		<button onmousedown={(_) => keyDown(3)} aria-label="Re#" class="key black"></button>
		<button onmousedown={(_) => keyDown(4)} class="key white">Mi</button>
		<button onmousedown={(_) => keyDown(5)} class="key white">Fa</button>
		<button onmousedown={(_) => keyDown(6)} aria-label="Fa#" class="key black"></button>
		<button onmousedown={(_) => keyDown(7)} class="key white">Sol</button>
		<button onmousedown={(_) => keyDown(8)} aria-label="Sol#" class="key black"></button>
		<button onmousedown={(_) => keyDown(9)} class="key white">La</button>
		<button onmousedown={(_) => keyDown(10)} aria-label="la#" class="key black"></button>
		<button onmousedown={(_) => keyDown(11)} class="key white">Si</button>
		<button onmousedown={(_) => keyDown(12)} class="key white">Do</button>
		<button onmousedown={(_) => keyDown(13)} aria-label="Do#" class="key black"></button>
		<button onmousedown={(_) => keyDown(14)} class="key white">Re</button>
		<button onmousedown={(_) => keyDown(15)} aria-label="Re#" class="key black"></button>
		<button onmousedown={(_) => keyDown(16)} class="key white">Mi</button>
		<button onmousedown={(_) => keyDown(17)} class="key white">Fa</button>
		<button onmousedown={(_) => keyDown(18)} aria-label="Fa#" class="key black"></button>
		<button onmousedown={(_) => keyDown(19)} class="key white">Sol</button>
		<button onmousedown={(_) => keyDown(20)} aria-label="Sol#" class="key black"></button>
		<button onmousedown={(_) => keyDown(21)} class="key white">La</button>
		<button onmousedown={(_) => keyDown(22)} aria-label="La#" class="key black"></button>
		<button onmousedown={(_) => keyDown(23)} class="key white">Si</button>
		<button onmousedown={(_) => keyDown(24)} class="key white">Do</button>
	</div>
</div>

<style>
	.piano {
		margin: 1em;
		width: min-content;
		background-color: var(--overlay0);
	}

	.controls {
		width: 100%;
		padding-bottom: 1em;
	}

	.keys {
		height: 10em;
		background-color: var(--surface2);
		border-radius: 0.5em 0.5em 0 0;
		overflow: hidden;
		border-top: 3px solid var(--surface2);

		display: flex;
		flex-direction: row;
	}

	.key {
		border-radius: 0 0 0.3em 0.3em;
		text-align: center;
		display: flex;
		justify-content: center;
		align-items: end;
		padding-bottom: 2em;
		color: var(--surface1);
	}

	.white:active {
		box-shadow: rgba(0, 0, 0, 0.1) 5px 1px 5px inset;
	}

	.key:not(:active) {
		transition: all 400ms ease-in-out;
	}

	.black:active {
		box-shadow: rgba(0, 0, 0, 0.1) 0px 0px 0px;
	}

	.white {
		width: 2em;
		height: 100%;
		background-color: var(--base);
		border-width: 0 1px;
		border-style: solid;
		box-shadow: rgba(0, 0, 0, 0.1) 0px -10px 0px inset;
	}

	.black {
		background-color: var(--text);
		width: 1em;
		height: 60%;
		margin-left: -0.5em;
		margin-right: -0.5em;
		z-index: 10;
		border: none;
		box-shadow: rgba(0, 0, 0, 0.1) 5px 0px 0px;
	}
</style>
