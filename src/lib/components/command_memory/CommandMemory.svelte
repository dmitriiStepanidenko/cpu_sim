<script lang="ts">
	import type { SharedMemory } from '$lib/pkg/cpu_sim_rs';
	import ModalCommand from './ModalCommand.svelte';
	import { derived } from 'svelte/store';
	import { CommandWrapper } from '$lib/pkg/cpu_sim_rs';
  import {opacityStore} from '$lib/opacity_store'
  import type {Writable} from 'svelte/store';

	export let name: String;

  export let memory: Writable<[number]>;

	let numeral_system_address = 10;

	const groupedMemory = derived(memory, ($memory) => {
		const groups = [];
		let tempValue = 0;

		$memory.forEach((data: number, index: number) => {
			tempValue |= data << ((index % 4) * 8);
			if ((index + 1) % 4 === 0) {
				groups.push(tempValue);
				tempValue = 0;
			}
		});

		if ($memory.length % 4 !== 0) {
			groups.push(tempValue);
		}

		return groups;
	});

	function decode(data: number) {
		try {
			return CommandWrapper.decode(data).get_data();
		} catch (e) {
			console.log('Error!');
			console.log(e);
		}
	}

  function changeNumeralSystemAddress(event: Event & {currentTarget : HTMLInputElement}) {
		numeral_system_address = Number(event.currentTarget.value);
	}

	let showModal = false;
	let modalCommandData: number | undefined = undefined;

	function openModalCommand(_: Event, value: number) {
		modalCommandData = value;
		showModal = true;
    opacityStore.set(true);
	}
	function closeModalCommand() {
		showModal = false;
    opacityStore.set(false);
	}
</script>

<div>
	<h3>{name}</h3>
	<div class="flex-row" style="font-size:12px;">
		<h4>Addr number system:</h4>
		<label>
			<input
				checked={numeral_system_address === 2}
				type="radio"
				on:change={changeNumeralSystemAddress}
				value={2}
			/>binary
		</label>
		<label>
			<input
				checked={numeral_system_address === 10}
				type="radio"
				on:change={changeNumeralSystemAddress}
				value={10}
			/>10
		</label>
	</div>
	<div class="table">
		<div class="row">
			<div class="cell addr">Addr</div>
			<div class="cell value">Value</div>
		</div>
		{#if $memory != undefined}
			{#each $groupedMemory as data, index}
				<div class="row" style="font-size:14px">
					<div class="cell addr">
						{#if numeral_system_address === 2}
							{(index * 4).toString(2).padStart(7, '0')}
						{:else if numeral_system_address === 10}
							{index * 4}
						{/if}
					</div>
					<div
						class="cell value"
						style="cursor: pointer"
						on:click={(event) => openModalCommand(event, data)}
					>
						<div>{data.toString(2)}</div>
						<div>{decode(data)}</div>
					</div>
				</div>
			{/each}
		{/if}
	</div>
	{#if showModal}
		<div class="backdrop" on:click={closeModalCommand}>
			<ModalCommand close={closeModalCommand} encodeData={modalCommandData} />
		</div>
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

	.addr {
		flex: 1;
    min-width: 80px;
	}

	.value {
		flex: 2;
		flex-direction: column;
    min-width: 80px;
	}

	.table {
		display: flex;
		flex-direction: column;
		border: 1px solid #000;
		justify-content: flex-start;
	}

	.row {
		display: flex;
		flex-direction: row;
		justify-content: flex-start;
	}

	.cell {
		border: 1px solid #ccc;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.flex-row {
		display: flex;
		flex-direction: row;
		justify-content: space-around;
	}
</style>
