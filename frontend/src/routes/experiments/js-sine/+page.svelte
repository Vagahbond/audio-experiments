<script lang="ts">
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { onMount } from 'svelte';

	let osc: OscillatorNode | undefined;

	let context: AudioContext | undefined = $state();

	let gainNode: GainNode | undefined = $state();

	let playState = $state(false);

	onMount(() => {
		context = new AudioContext();

		osc = context.createOscillator();

		gainNode = context.createGain();

		gainNode.connect(context.destination);

		osc?.start();
	});

	function play() {
		if (gainNode) osc?.connect(gainNode);
		playState = true;
	}

	function stop() {
		if (gainNode) osc?.disconnect(gainNode);
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
		osc.frequency.value = value;
	}
</script>

<div class="container box">
	<h3>Pure JS audio generation</h3>
	<div class="controls">
		<button disabled={playState} onclick={play} class="button">Play</button>
		<button disabled={!playState} onclick={stop} class="button">Stop</button>
		<!-- volume slider -->
		<span>Volume</span>
		<input
			id="volume"
			class="slider"
			type="range"
			min="0.0"
			max="3.0"
			step="0.01"
			defaultvalue="1.0"
			oninput={(event: any) => {
				changeVolume(event.target.valueAsNumber);
			}}
		/>
		<span>Frequency</span>
		<input
			id="frequency"
			class="slider"
			type="range"
			min="20"
			max="16000"
			step="1"
			defaultvalue="440"
			oninput={(event: any) => {
				changeFrequency(event.target.valueAsNumber);
			}}
		/>
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

</style>
