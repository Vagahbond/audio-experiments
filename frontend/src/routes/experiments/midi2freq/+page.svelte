<script lang="ts">
	import Piano from '$lib/components/piano.svelte';
	import Snailwisdom from '$lib/components/snailwisdom.svelte';
</script>

<Piano onKeyDown={(k) => alert(k.key)}></Piano>

<Snailwisdom
	title="Midi to frequency !"
	message={`A midi note is a number between 0 and 127 that corresponds to a real note on the standard system.
    We know that the frequency of A0 is 440 Hz, but how to find the frequency of other notes ?
    An octave consists of 12 semi-tones.
    To step up an octave, we multiply the frequency by 2.
    To step down an octave, we divide the frequency by 2.
    This means that a single step up is multiplying the current frequency by the twelfth root of two.
    The twelfth root of two is approximately 1.0594630943592953.
    So, to step up an octave, we multiply the current frequency by 1.0594630943592953.
    We can use the same logic to step down an octave.
    Press a key on the piano to see the frequency of the note !
  `}
/>
