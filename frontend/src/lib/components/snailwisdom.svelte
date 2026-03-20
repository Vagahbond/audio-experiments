<script lang="ts">
	const { title, message } = $props();
</script>

<div class="snail-container">
	<div class="snail-text box">
		<h2>{title}</h2>
		<p>{message}</p>
	</div>
	<div class="snail">
		<img src="/snail.png" alt="snail" class="snail-image" />
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

		background-color: rgba(0, 0, 0, 0);
	}

	.snail-image {
		animation: snail-animation 10s infinite linear;
		height: 8em;

		background-color: rgba(0, 0, 0, 0);
		filter: drop-shadow(20px 30px 10px rgba(0, 0, 0, 0.5));
	}

	.snail-text {
		margin: 0 1em 1em 1em;
		width: 100%;
		position: relative;
	}

	.snail-text h2 {
		margin: 0;
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
		bottom: 6em;
		background: linear-gradient(to left top, transparent 50%, var(--surface0) 50%);
	}

	.snail-text:before {
		bottom: 6.5em;
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
