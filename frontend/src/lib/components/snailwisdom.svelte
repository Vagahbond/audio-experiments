<script lang="ts">
	const { title, message } = $props();
</script>

<div class="snail-container">
	<div class="snail">
		<img src="/snail.png" alt="snail" class="snail-image" />
	</div>
	<div class="snail-text">
		<h1>{title}</h1>
		<p>{message}</p>
	</div>
</div>

<style>
	.snail-container {
		display: flex;
		flex-direction: row;

		justify-content: space-between;
		align-items: center;
		position: fixed;
		bottom: 0;
		right: 0;
		left: 0;
		width: 100%;
	}

	.snail {
		height: 8em;
		margin: 1em;
	}

	.snail-image {
		animation: snail-animation 10s infinite linear;
		height: 8em;
		transform: scale(-0.5 0.5);
	}

	.snail-text {
		margin: 0 1em 1em 1em;
		width: 100%;
		padding: 1em;
		background-color: var(--surface0);
		border-radius: 0.5em;
		position: relative;
	}

	.snail-text:after,
	.snail-text:before {
		content: ' ';
		position: absolute;
		width: 2em;
		height: 0.5em;
		left: -2em;
	}
	.snail-text:after {
		bottom: 2em;
		background: linear-gradient(to right top, transparent 50%, var(--surface0) 50%);
	}
	.snail-text:before {
		bottom: 2.5em;
		background: linear-gradient(to right bottom, transparent 50%, var(--surface0) 50%);
	}

	.snail-text h1 {
		font-size: 1.5em;
	}

	.snail-text p {
		font-size: 1em;
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
