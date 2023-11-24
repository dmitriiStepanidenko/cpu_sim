<script lang="ts">
	import MemoryComponent from '$lib/components/memory/memory.svelte';
	import CommandMemoryComponent from '$lib/components/command_memory/CommandMemory.svelte';
	import RegisterMemoryComponent from '$lib/components/register_memory/RegisterMemory.svelte';
	import CompilerComponent from '$lib/components/compiler/compiler.svelte';
	import { onMount } from 'svelte';
	import type { SharedMemory } from '$lib/pkg/cpu_sim_rs';
	import { Program } from '$lib/pkg/cpu_sim_rs';
	import init, { Cpu, MemoryType } from '$lib/pkg/cpu_sim_rs';
	import { Pause, Play, StepForward } from 'lucide-svelte';
	import ModalWindow from '$lib/ModalWindow.svelte';
	import { create_success, create_error } from '$lib/Toaster.svelte';

	let cpu: Cpu;
	let registersMemory: SharedMemory;
	let dataMemory: SharedMemory;
	let cmdMemory: SharedMemory;
	let program: Program;

	let cmd_size = 80;
	let data_size = 16;
	let reg_size = 16;

	onMount(async () => {
		if (cpu === undefined) {
			init().then(() => {
				//set_trace();
				cpu = new Cpu(cmd_size, data_size, reg_size);
				registersMemory = cpu.get_memory(MemoryType.Registers);
				dataMemory = cpu.get_memory(MemoryType.Data);
				cmdMemory = cpu.get_memory(MemoryType.Command);

				program = new Program();
			});
		}
	});

	function handlePause() {
		console.log('Pause clicked!');
		// Handle the click event
	}

	function handlePlay() {
		console.log('Play clicked!');
	}

	function hadleStepForward() {
		cpu?.do_op();
		console.log('StepForward clicked!');
	}

	function encodeProgram(program: Program) {
		if (program.text_len() * 4 > cmd_size) {
			create_error('Encode error', 'Not enough command memory size');
      return;
		}
		cpu?.encode(program);
    create_success('Encode successfull','');
	}

	function handleProgram(event: CustomEvent<Program>) {
		program = event.detail;
		console.log(program); // { key: 'value' }
	}
</script>

<svelte:head>
	<title>Simulator of Harvard-architecture cpu</title>
	<meta
		name="description"
		content="Simulator of imaginary cpu with Harvard architecture, 
    that build for educational purpose with rust and sveltekit"
	/>
</svelte:head>

<div>
	<div class="row_buttons">
		<!--<button disabled on:click={handlePause} class="icon-button"><Pause /></button>
    <button disabled on:click={hadlePlay} class="icon-button"><Play /></button>-->
		<button on:click={hadleStepForward} class="icon-button"><StepForward /></button>
	</div>
	<div class="row_components">
		<CompilerComponent on:program={handleProgram} encodeFunction={encodeProgram} />
		<RegisterMemoryComponent memory={registersMemory} name="Registers" />
		<MemoryComponent memory={dataMemory} name="Data mem" />
		{#if cmdMemory}
			<CommandMemoryComponent memory={cmdMemory} name="Cmd mem" />
		{/if}
	</div>
	<ModalWindow />
</div>

<style>
	.row_buttons {
		display: flex;
		flex-direction: row;
		width: 100%;
	}
	.row_components {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
		width: 100%;
		flex-wrap: wrap;
	}
	.icon-button {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
		color: var(--action-color);
	}
	:disabled:hover {
		cursor: default;
	}

	*,
	::before,
	::after {
		box-sizing: unset;
	}
</style>
