<script lang="ts">
	import Rack from '$lib/components/control-rack/rack.svelte';
	import Rackslot from '$lib/components/control-rack/rackslot.svelte';
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { onMount } from 'svelte';

	let osc: OscillatorNode | undefined = $state();

	let context: AudioContext | undefined = $state();

	let gainNode: GainNode | undefined = $state();

	let playState = $state(false);

	let oscShape: OscillatorType = $state('sine');

	onMount(() => {
		context = new AudioContext();
		context.suspend();

		osc = context.createOscillator();
		osc.frequency.value = 20;

		gainNode = context.createGain();
		gainNode.gain.value = 0.5;

		osc.connect(gainNode);

		gainNode.connect(context.destination);

		osc?.start();
	});

	function play() {
		context?.resume();
		playState = true;
	}

	function stop() {
		context?.suspend();
		playState = false;
	}

	function changeVolume(value: number) {
		if (!gainNode) {
			return;
		}
		gainNode.gain.value = value;
	}

	function changeFrequency(value: number) {
		if (!osc) {
			return;
		}
		// exponential input
		osc.frequency.value = 20 + Math.pow(15980, Math.pow(value / 100, 0.5));
		console.log(osc.frequency.value);
	}

	function setShape(shape: OscillatorType) {
		if (!osc) {
			return;
		}
		osc.type = shape;
		oscShape = shape;
	}
</script>

<div class="container box">
	<h3>Pure JS audio generation</h3>
	<div class="controls">
		<Rack label="Sine controls">
			<Rackslot label="Play/Stop">
				<button disabled={playState} onclick={play} class="button">Play</button>
				<button disabled={!playState} onclick={stop} class="button">Stop</button>
			</Rackslot>
			<Rackslot label="Volume">
				<input
					id="volume"
					class="slider"
					type="range"
					min="0.0"
					max="3.0"
					step="0.01"
					defaultValue="0.5"
					oninput={(event: any) => {
						changeVolume(event.target.valueAsNumber);
					}}
				/>
			</Rackslot>
			<Rackslot label="Frequency">
				<input
					id="frequency"
					class="slider"
					type="range"
					min="0"
					max="100"
					step="1"
					defaultValue="1"
					oninput={(event: any) => {
						changeFrequency(event.target.valueAsNumber);
					}}
				/>
			</Rackslot>
			<Rackslot label="Shape">
				<button onclick={() => setShape('sine')} class="button" disabled={oscShape === 'sine'}>
					Sine
				</button>
				<button
					onclick={() => setShape('triangle')}
					class="button"
					disabled={oscShape === 'triangle'}
				>
					Triangle
				</button>
				<button
					onclick={() => setShape('sawtooth')}
					class="button"
					disabled={oscShape === 'sawtooth'}
				>
					Sawtooth
				</button>
				<button onclick={() => setShape('square')} class="button" disabled={oscShape === 'square'}>
					Square
				</button>
			</Rackslot>
		</Rack>
	</div>
</div>

<Snailwisdom
	noRustCode
	title="Pure JS audio generation !"
	message={`This is a simple example of how to generate audio using pure JS. 
    It uses the Web Audio API to create a GainNode that can be used to control the volume of the audio.
    The audio is played using the MediaElementSource, which is a wrapper around the HTMLAudioElement.
    The audio is connected to the GainNode, which is then connected to the destination (the speakers).
    The volume of the audio can be controlled using the slider.
    Press the play button to start playing the audio, and the stop button to stop it.
  `}
/>

<style>
	.controls {
		margin: 1em;
	}

	.controls {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: space-around;
	}
</style>
