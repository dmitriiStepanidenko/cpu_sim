<script lang="ts">
	import { derived } from 'svelte/store';
	import { CommandWrapper } from '$lib/pkg/cpu_sim_rs';
	import { opacityStore } from '$lib/opacity_store';
	import type { Readable } from 'svelte/store';
	import CommandMemoryHelp from '$lib/components/help/CommandMemoryHelp.svelte';
	import { openModal } from '$lib/ModalWindow.svelte';
	import { HelpCircle } from 'lucide-svelte';
	import type { SharedMemory } from '$lib/pkg/cpu_sim_rs';

	export let name: String;

	export let memory: Readable<[number]> & SharedMemory;

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

	function changeNumeralSystemAddress(event: Event & { currentTarget: HTMLInputElement }) {
		numeral_system_address = Number(event.currentTarget.value);
	}

</script>

<div>
	<h3>
		{name}
		<button class="icon-button" on:click={() => openModal(CommandMemoryHelp)}><HelpCircle /></button
		>
	</h3>
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
			<div class="cell addr header header-left">Addr</div>
			<div class="cell value header header-right">Value</div>
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
					<button class="cell value" style="cursor: pointer">
						<div>{data.toString(2)}</div>
						<div>{decode(data)}</div>
					</button>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	@import '../memoryStyles.css' scoped;

	.addr {
		flex: 1;
	}
	.value {
		flex: 4;
	}
	@media (min-width: 0px) {
		.value {
			max-width: 300px;
		}
	}

	select {
		background-color: var(--base-color);
		color: var(--primary-color);
		border: 0.2px solid var(--secondary-color);
		border-radius: 7px;
		margin-left: 5px;
		text-align-last: center;
	}
	option {
		margin: 5px;
	}
</style>
