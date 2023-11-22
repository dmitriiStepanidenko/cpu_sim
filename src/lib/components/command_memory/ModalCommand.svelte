<script lang="ts">
	import { CommandWrapper } from '$lib/pkg/cpu_sim_rs';

  export let close: (event: MouseEvent) => void; 
	export let encodeData: number;

  let data: string;
  $: data = CommandWrapper.decode(encodeData).get_data();
</script>

<div class="modal">
	<div class="modal-content">
		<div>
			{#if encodeData != undefined}
        {data}
			{/if}
		</div>
		<button on:click={close}>Закрыть</button>
	</div>
</div>

<style>
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
		background-color: #fff;
		padding: 20px;
		border-radius: 5px;
	}
</style>
