<script lang="ts">
	import type { SharedMemory } from '$lib/pkg/cpu_sim_rs';
	import { derived } from 'svelte/store';
	import { addToast } from '$lib/Toaster.svelte';
	import { CommandWrapper } from '$lib/pkg/cpu_sim_rs';
	import { onMount } from 'svelte';

	export let name: String;

	export let memory: SharedMemory | undefined;

	let tmp_value = 0;

	let numeral_system_address = 10;

	const groupedMemory = derived(memory, ($memory) => {
		const groups = [];
		let tempValue = 0;

		$memory.forEach((data, index) => {
			tempValue |= data << ((index % 4) * 8);
			if ((index + 1) % 4 === 0) {
				groups.push(tempValue);
				tempValue = 0; // Reset the value for the next group
			}
		});

		// Handle the case where there are not enough cells to form a group of 4 at the end
		if ($memory.length % 4 !== 0) {
			groups.push(tempValue);
		}

		return groups;
	});

	function decode(data) {
		try {
			return CommandWrapper.decode(data).get_data();
		} catch (e) {
			console.log('Error!');
			console.log(e);
		}
	}

	function changeNumeralSystemAddress(event) {
		numeral_system_address = Number(event.currentTarget.value);
	}
</script>

<div>
	{name} Commad memory view!
	<div>
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
			<div class="cell">Addr</div>
			<div class="cell">Value</div>
		</div>
		{#if $memory != undefined}
			{#each $groupedMemory as data, index}
				<div class="row" style="font-size:14px">
					<div class="cell">
						{#if numeral_system_address === 2}
							{(index * 4).toString(2).padStart(7, '0')}
						{:else if numeral_system_address === 10}
							{index * 4}
						{/if}
					</div>
					<div class="cell">
						{data.toString(2)}
						{decode(data)}
					</div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.table {
		display: flex;
		flex-direction: column;
		border: 1px solid #000;
	}

	.row {
		display: flex;
		flex-direction: row;
		width: 100%;
	}

	.cell {
		flex: 1;
		padding: 10px;
		border: 1px solid #ccc;
	}
</style>
