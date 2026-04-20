<script lang="ts">
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { midi_to_freq } from 'midi2freq';
	import { freq_to_midi } from 'freq2midi';
	import { bend_amount } from 'bend_amount';
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

	let bend = $derived(bend_amount(frequency) * 100);

	let context: AudioContext | undefined = $state();
	let osc: OscillatorNode | undefined = $state();

	let playState = $state(false);

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
	<h3>Bend amount</h3>
	<div class="controls-container">
		<Rack label="Sine controls">
			<Rackslot label="Play/Stop">
				<button disabled={playState} onclick={play} class="button">Play</button>
				<button disabled={!playState} onclick={stop} class="button">Stop</button>
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
		<span> Its frequency is {corrected_frequency.toFixed(2)}Hz</span><br />
		<span> The bend rate compared to them is {bend.toFixed(2)}%</span><br />
	</div>
</div>

<Snailwisdom
	title="Frequency to midi note!"
	message={`
    This simple program converts any given frequency to the closest midi note.\n 
    More than that, it also calculates a percentage value that represents the "bend", which is the difference between the closest midi note and the given frequency. The fomula is basically "(current note) - (closest note) / abs(closest note) - (second closest note).`}
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
