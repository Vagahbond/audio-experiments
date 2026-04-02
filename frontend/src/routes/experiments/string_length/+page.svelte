<script lang="ts">
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { midi_to_freq } from 'midi2freq';
	import { onMount } from 'svelte';
	import Rack from '$lib/components/control-rack/rack.svelte';
	import Rackslot from '$lib/components/control-rack/rackslot.svelte';
	import String from '$lib/components/string.svelte';
	import { string_length2freq, string_length2midi } from 'string_length';

	let length = $state<number>(0);

	let midi = $derived(string_length2midi(length));

	let raw_frequency = $derived(string_length2freq(length));

	let corrected_frequency = $derived(midi_to_freq(midi));

	let context: AudioContext | undefined = $state();
	let osc: OscillatorNode | undefined = $state();

	let playState = $state(false);
	let autotuneState = $state(false);

	$effect(() => {
		if (osc && length) {
			osc.frequency.value = autotuneState ? corrected_frequency : raw_frequency;
		}
	});

	function onLengthChanged(value: number) {
		length = (value * 2120) / 100000 + 15;
	}

	onMount(() => {
		context = new AudioContext();
		context.suspend();

		osc = context.createOscillator();
		osc.frequency.value = raw_frequency;

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
	<h3>String length to midi</h3>
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
	</div>
	<String playing={playState} onValueChanged={onLengthChanged} />
	<div class="output box">
		<span> Length: {length.toFixed(2)}cm</span> <br />
		<span> Raw frequency: {raw_frequency.toFixed(2)} </span> <br />
		<span> Closest midi note is {midi}</span><br />

		<span> Its frequency is {corrected_frequency.toFixed(2)}</span><br />
	</div>
</div>

<Snailwisdom
	title="String length to midi note ! "
	message={`
  Assuming that a string vibrates at A4's frequency if it is 660CM, we can then infer a frequency to any length for this string ! 
  It works oposite to the frequency in relation to the midi value: The higher the midi value, the smaller the string, and the higher the frequency. 
Other than that, nothing changes, so the half-tone coefficient we have been using still applies here, it just has to be applied the other way around. Sweet! This exercise is an excuse for me to create a nice string component for later, and prepare for the bending simulation !
  `}
/>

<style>
	.output {
		margin-top: 1em;
	}

	.controls-container {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		margin-top: 1em;
		margin-bottom: 1em;
	}
</style>
