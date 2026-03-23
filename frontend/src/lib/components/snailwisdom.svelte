<script lang="ts">
	import { onMount } from 'svelte';

	interface SnailProps {
		title: string;
		message: string;
		noRustCode?: boolean;

		noSvelteCode?: boolean;
	}

	const baseFrontendUrl =
		'https://github.com/Vagahbond/audio-experiments/tree/main/frontend/src/routes/experiments';

	const baseWasmUrl = 'https://github.com/Vagahbond/audio-experiments/tree/main/wasm';

	const { title, message, noRustCode, noSvelteCode }: SnailProps = $props();

	let currentUrl = $state('');

	let rustCode = $derived(`${baseWasmUrl}/${currentUrl}`);

	let svelteCode = $derived(`${baseFrontendUrl}/${currentUrl}`);

	let reduced = $state(false);

	onMount(() => {
		currentUrl = window.location.href.split('/').pop() ?? '';
	});
</script>

<div class={`snail-container ${reduced && 'reduced'}`}>
	<div class="snail-text box">
		<div class="title-bar">
			<h2>{title}</h2>
			<div class="links">
				{#if !noRustCode}
					<a class="button" target="_blank" href={rustCode}>Rust code</a>
				{/if}

				{#if !noSvelteCode}
					<a class="button" target="_blank" href={svelteCode}>Svelte code </a>
				{/if}
				<button class="button close" onclick={(_) => (reduced = true)}> x </button>
			</div>
		</div>
		<p>{message}</p>
	</div>
	<div class="snail">
		<button class="button open" onclick={(_) => (reduced = false)}> ? </button>
		<img src="/snail.png" alt="snail" class="snail-image" />
	</div>
</div>

<style>
	.snail-container {
		display: flex;
		flex-direction: row;

		justify-content: space-between;
		position: fixed;
		bottom: 0.5em;
		right: 8.5em;
		left: 0.5em;

		z-index: 15;
	}

	.open,
	.close {
		padding: 0.5em 1em;
	}

	.open {
		display: none;
		top: 1em;
		left: -1em;
		position: absolute;
		z-index: 10;
	}

	.reduced .open {
		display: block;
	}

	.reduced .snail-text {
		transform: translateY(110%);
	}

	.reduced .snail {
		background-color: rgba(0, 0, 0, 0);
		bottom: 1em;
		right: -5em;
	}

	.snail {
		bottom: 1em;
		right: 0em;
		position: fixed;
		transition: all 0.5s ease-in;
		animation: snail-animation 10s infinite linear;
	}

	.snail-image {
		height: 8em;

		background-color: rgba(0, 0, 0, 0);
		filter: drop-shadow(20px 30px 10px rgba(0, 0, 0, 0.5));
	}

	.snail-text {
		transition: all 0.5s ease-in;
		margin: 0 1em 1em 1em;
		width: 100%;
		position: relative;
	}

	.links {
		display: flex;
		padding-left: 1em;
		flex-grow: 1;
	}

	.links a {
		margin-right: 1em;
		text-decoration: none;
		color: var(--text);
	}

	.links .close {
		margin-left: auto;
	}

	.title-bar {
		display: flex;
	}

	.snail-text h2 {
		margin-top: auto;
		margin-bottom: auto;
	}

	.snail-text:after,
	.snail-text:before {
		content: ' ';
		position: absolute;
		width: 2em;
		height: 0.5em;
		right: -2em;
	}
	.snail-text:after {
		bottom: 5em;
		background: linear-gradient(to left top, transparent 50%, var(--surface0) 50%);
	}

	.snail-text:before {
		bottom: 5.5em;
		background: linear-gradient(to left bottom, transparent 50%, var(--surface0) 50%);
	}

	/*snail slowly goes up and down as an NPC */
	@keyframes snail-animation {
		0% {
			transform: translateY(0);
		}

		20% {
			transform: translateY(-10px);
		}

		40% {
			transform: translateY(0);
		}

		60% {
			transform: translateY(-10px);
		}

		80% {
			transform: translateY(0);
		}

		100% {
			transform: translateY(0);
		}
	}
</style>
