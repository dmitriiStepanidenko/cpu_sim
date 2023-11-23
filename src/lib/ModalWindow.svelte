<script lang="ts" context="module">
	import type { ModelChildComponent } from './ModalWindow.ts';
	import type { Writable } from 'svelte/store';
	import { writable } from 'svelte/store';

	let component: Writable<ModelChildComponent> = writable();

	let showModal = writable(false);
	function closeModal() {
		showModal.set(false);
	}
	export const openModal = (component_input: ModelChildComponent) => {
		component.set(component_input);
		showModal.set(true);
	};
</script>

<div>
	{#if $showModal}
		<button class="backdrop" on:click={closeModal}>
			<div class="modal">
				<div class="modal-content">
					<svelte:component this={$component} />
				</div>
			</div>
		</button>
	{/if}
</div>

<style>
	.backdrop {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.6);
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.modal {
		position: fixed;
		left: 0;
		top: 0;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 900;
	}

	.modal-content {
		z-index: 900;
		background-color: var(--base-color);
		padding: 20px;
		border-radius: 5px;
		min-width: 40vw;
		min-height: 40vh;
		max-width: 90vw;
		max-height: 90vh;
		overflow-y: scroll;
	}
</style>
