<script>
	import Toaster from '$lib/Toaster.svelte';
	import Header from './Header.svelte';

	import { theme } from '$lib/components/themeStore';

	import { themes } from '$lib/styles/themes';

	import './styles.css';
	import '../app.css';

	import { base } from '$app/paths';
</script>

<svelte:head>
	<link rel="manifest" href={base + '/manifest.json'} />
</svelte:head>

<Toaster />
<div
	class="app"
	style="--base-color: {themes[$theme]['base-color']}; 
    --primary-color: {themes[$theme]['primary-color']}; 
    --action-color: {themes[$theme]['action-color']}; 
    --secondary-color: {themes[$theme]['secondary-color']};
    --error-color: {themes[$theme]['error-color']};"
>
	<Header />

	<main>
		<slot />
	</main>
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
		background-color: var(--base-color);
		color: var(--primary-color);
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		width: 100%;
		max-width: min(1600px, 95vw);
		margin: 0 auto;
		box-sizing: border-box;
	}
	@media (min-width: 0px) {
		main {
			padding: 0;
			margin-left: 0.25vw;
			margin-right: 0.25vw;
		}
	}
	@media (min-width: 0px) {
		main {
			padding: 0;
			margin-left: 0.25vw;
			margin-right: 0.25vw;
		}
	}
	@media (min-width: 600px) {
		main {
			padding: 1rem;
			margin: 0 auto;
		}
	}
</style>
