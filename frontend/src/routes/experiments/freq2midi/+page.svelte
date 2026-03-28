<script lang="ts">
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { midi_to_freq } from 'midi2freq';
	import { freq_to_midi } from 'freq2midi';
	import { onMount } from 'svelte';
	import Rack from '$lib/components/control-rack/rack.svelte';
	import Rackslot from '$lib/components/control-rack/rackslot.svelte';
	import { logarithmicInput } from '$lib/input';

	let frequencySliderValue = $state<number>(0);

	let frequency = $derived.by(() => {
		const max = 13289.75;
		const min = 8.18;

		return logarithmicInput(frequencySliderValue, min, max, 100000);
	});

	let midi = $derived(freq_to_midi(frequency));

	let corrected_frequency = $derived(midi_to_freq(midi));

	let context: AudioContext | undefined = $state();
	let osc: OscillatorNode | undefined = $state();

	let playState = $state(false);
	let autotuneState = $state(false);

	$effect(() => {
		if (osc) {
			osc.frequency.value = autotuneState ? corrected_frequency : frequency;
		}
	});

	onMount(() => {
		context = new AudioContext();
		context.suspend();

		osc = context.createOscillator();
		osc.frequency.value = frequency;

		osc.connect(context.destination);

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
</script>

<div class="container box">
	<h3>Frequency to midi</h3>
	<div class="controls-container">
		<Rack label="Sine controls">
			<Rackslot label="Play/Stop">
				<button disabled={playState} onclick={play} class="button">Play</button>
				<button disabled={!playState} onclick={stop} class="button">Stop</button>
			</Rackslot>
			<Rackslot label="Auto-tune">
				<button disabled={autotuneState} onclick={() => (autotuneState = true)} class="button">
					On
				</button>
				<button disabled={!autotuneState} onclick={() => (autotuneState = false)} class="button">
					Off
				</button>
			</Rackslot>
		</Rack>
		<input
			class="slider"
			type="range"
			bind:value={frequencySliderValue}
			min="0"
			max="100000"
			step="1"
		/>
	</div>
	<div class="output box">
		<span> Frequency: {frequency.toFixed(2)}Hz</span> <br />
		<span> Closest midi note is {midi}</span><br />

		<span> Its frequency is {corrected_frequency.toFixed(2)}</span><br />
	</div>
</div>

<Snailwisdom
	title="Frequency to midi note!"
	message={`
    This simple program converts any given frequency to the closest midi note. You could call it an autotuner.
    The technique used is based on natural logarithm :
     log(frequency / c0) / log(semitone_ratio);
    We ratio the log of the frequency divided by the lowest frequency (c0) to the log of the semitone ratio (2^(1/12)).
The result is a value corresponding to the midi note.
  `}
/>

<style>
	.controls-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		margin-top: 1em;
		margin-bottom: 1em;
	}

	.controls-container > .slider {
		width: 100%;
	}
</style>
