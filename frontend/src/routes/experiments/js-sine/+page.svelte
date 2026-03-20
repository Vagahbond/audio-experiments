<script lang="ts">
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
	import { onMount } from 'svelte';

	let audio: HTMLAudioElement;

	let context: AudioContext | undefined = $state();

	let gainNode: GainNode | undefined = $state();

	let playState = $state(false);

	onMount(() => {
		context = new AudioContext();

		const track = context.createMediaElementSource(audio);

		gainNode = context.createGain();

		track.connect(gainNode).connect(context.destination);
	});

	function play() {
		audio.play();
		playState = true;
	}

	function stop() {
		audio.pause();
		playState = false;
	}

	function changeVolume(value: number) {
		if (!gainNode) {
			return;
		}
		gainNode.gain.value = value;
	}
</script>

<div class="container box">
	<h3>Pure JS audio generation</h3>
	<div class="controls">
		<audio bind:this={audio} src="/mycooltrack.mp3"></audio>
		<button disabled={playState} onclick={play} class="button">Play</button>
		<button disabled={!playState} onclick={stop} class="button">Stop</button>
		<!-- volume slider -->
		<input
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
	</div>
</div>

<Snailwisdom
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

	.slider {
		color: var(--text);
		background-color: var(--base);
		border-radius: 0.5em;
	}

	.slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 1em;
		height: 1em;
		border-radius: 0.5em;
		background: var(--text);
		cursor: pointer;
	}

	.slider::-moz-range-thumb {
		width: 1em;
		height: 1em;
		border-radius: 0.5em;
		background: var(--overlay0);
		cursor: pointer;
	}

	.slider::-ms-thumb {
		width: 1em;
		height: 1em;
		border-radius: 0.5em;
		background: var(--overlay0);
		cursor: pointer;
	}
</style>
