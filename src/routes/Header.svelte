<script>
	import { theme } from '$lib/components/themeStore';

	import Logo from '$lib/images/android-chrome-512x512.png?enhanced';
	import { ToggleLeft, ToggleRight } from 'lucide-svelte';
</script>

<div class="main">
	<div style="display: flex;">
		<enhanced:img src={Logo} alt="NN generated cpu" class="logo" sizes="64px 64px" />
		<h1 class="main_title">Simulator of imaginary Harvard-architecture cpu</h1>
	</div>
	<div class="toggle_main">
		<h3>Grey</h3>
		{#if $theme === 'grey'}
			<button on:click={() => ($theme = 'white')}><ToggleLeft /></button>
		{:else}
			<button on:click={() => ($theme = 'grey')}><ToggleRight /></button>
		{/if}
		<h3>White</h3>
	</div>
</div>

<style>
	.toggle_main {
		display: flex;
		flex-direction: row;
		align-items: center;
	}
  .toggle_main > h3 {
		padding: 10px;
	}
	.main {
		padding-top: 10px;
		padding-bottom: 10px;
		padding-left: 350px;
		padding-right: 50px;
		display: flex;
		justify-content: space-between;
		flex-direction: row;
		align-items: center;
		flex-wrap: wrap;
	}
	.main_title {
		font-size: 32px;
		padding-left: 50px;
	}

	.logo {
		padding-right: 0px;
		padding-left: 0px;
		width: 64px;
		height: 64px;
		min-width: 64px;
		min-height: 64px;
		left: 0px;
	}

	@media (min-width: 200px) {
		.main {
			padding-left: 5px;
		}
	}
	@media (min-width: 300px) {
		.main {
			padding-left: 5px;
		}
	}
	@media (min-width: 400px) {
		.main {
			padding-left: 20px;
		}
	}
	@media (min-width: 500px) {
		.main {
			padding-left: 30px;
		}
	}
	@media (min-width: 700px) {
		.main {
			padding-left: 50px;
		}
	}
	@media (min-width: 900px) {
		.main {
			padding-left: 70px;
		}
	}
	@media (min-width: 1100px) {
		.main {
			padding-left: 100px;
		}
	}
	@media (min-width: 1200px) {
		.main {
			padding-left: 150px;
		}
	}
	@media (min-width: 1500px) {
		.main {
			padding-left: 200px;
		}
	}
	@media (min-width: 1300px) {
		.main {
			padding-left: 250px;
		}
	}
	@media (min-width: 1600px) {
		.main {
			padding-left: 350px;
		}
	}
</style>
