<script lang="ts">
	import type { SharedMemory } from '$lib/pkg/cpu_sim_rs';
	import { addToast } from '$lib/Toaster.svelte';

	export let name: String;

	export let memory: SharedMemory | undefined;

	function isBinaryString(str: string) {
		var binaryPattern = /^[01]+$/;
		return binaryPattern.test(str);
	}

	let numeral_system_value = 10;
	let numeral_system_address = 10;

	function handleCellChange(event: FocusEvent) {
		const cellId = event.target?.id;
		let newValue = event.target.innerText;
		console.log(newValue);
		if (isBinaryString(newValue) && numeral_system_value === 2) {
			newValue = parseInt(newValue, 2);
			console.log(newValue);
			memory?.write_u8(cellId, newValue);
			create_success();
			return;
		}
		if (numeral_system_value === 10) {
			newValue = parseInt(newValue, 10);
			console.log(newValue);
			memory?.write_u8(cellId, newValue);
			create_success();
			return;
		}
		create_error();
		console.log($memory[cellId]);
		if (numeral_system_value === 2) {
			event.target.innerText = $memory[cellId].toString(2).padStart(8, '0');
		} else {
			event.target.innerText = $memory[cellId].toString();
		}
	}

	function create_success() {
		addToast({
			data: {
				title: 'Успешный успех!',
				description: 'Новое значение ячейки памяти было записано!',
				color: 'green'
			}
		});
	}
	function create_error() {
		addToast({
			data: {
				title: 'Неуспешный неуспех!',
				description: 'Новое значение ячейки памяти НЕ было записано!',
				color: 'red'
			}
		});
	}

	function changeNumeralSystemValue(event) {
		numeral_system_value = Number(event.currentTarget.value);
	}
	function changeNumeralSystemAddress(event) {
		numeral_system_address = Number(event.currentTarget.value);
	}
</script>

<div>
	{name}
	<div>
		<label>
			<input
				checked={numeral_system_value === 2}
				type="radio"
				on:change={changeNumeralSystemValue}
				value={2}
			/>binary
		</label>
		<label>
			<input
				checked={numeral_system_value === 10}
				type="radio"
				on:change={changeNumeralSystemValue}
				value={10}
			/>10
		</label>
	</div>
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
			{#each $memory as data, index}
				<div class="row" style="font-size:13px">
					<div class="cell">
						{#if numeral_system_address === 2}
							{index.toString(2).padStart(7, '0')}
						{:else if numeral_system_address === 10}
							{index}
						{/if}
					</div>
					{#if numeral_system_value == 2}
						<div id={index} class="cell" contenteditable on:blur={handleCellChange}>
							{data.toString(2).padStart(8, '0')}
						</div>
					{/if}
					{#if numeral_system_value == 10}
						<div id={index} class="cell" contenteditable on:blur={handleCellChange}>
							{data}
						</div>
					{/if}
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
