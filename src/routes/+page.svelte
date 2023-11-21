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

	let cpu: Cpu | undefined = undefined;
	let registersMemory: SharedMemory | undefined;
	let dataMemory: SharedMemory | undefined;
	let cmdMemory: SharedMemory | undefined;
	let program: Program | undefined;

	let selectedCmdRepresentation = 'cmd';

	onMount(async () => {
		init().then(() => {
			cpu = new Cpu(80, 32, 32);
			registersMemory = cpu.get_memory(MemoryType.Registers);
			dataMemory = cpu.get_memory(MemoryType.Data);
			cmdMemory = cpu.get_memory(MemoryType.Command);
			program = new Program();
		});
	});

	function handlePause() {
		console.log('Pause clicked!');
		// Handle the click event
	}
	function hadlePlay() {
		console.log('Play clicked!');
	}
	function hadleStepForward() {
		cpu?.do_op();
		console.log('StepForward clicked!');
	}
	function encodeProgram() {
		console.log(program);
		cpu?.encode(program);
	}
	function handleProgram(event) {
		program = event.detail;
		console.log(program); // { key: 'value' }
	}
	function handleSelectionCmdView(event) {
		selectedCmdRepresentation = event.target.value;
	}
</script>

<div>
	<div class="row">
		<button on:click={handlePause} class="icon-button"><Pause /></button>
		<button on:click={hadlePlay} class="icon-button"><Play /></button>
		<button on:click={hadleStepForward} class="icon-button"><StepForward /></button>
		<button on:click={encodeProgram}>Encode</button>
		<select on:change={handleSelectionCmdView}>
			<option value="cmd">Cmd</option>
			<option value="data">Data</option>
		</select>
	</div>
	<div class="row">
		<RegisterMemoryComponent memory={registersMemory} name="Registers" />
		<MemoryComponent memory={dataMemory} name="Data mem" />
		{#if cmdMemory}
			{#if selectedCmdRepresentation === 'data'}
				<MemoryComponent memory={cmdMemory} name="Cmd mem" />
			{:else}
				<CommandMemoryComponent memory={cmdMemory} name="Cmd mem" />
			{/if}
		{/if}
		<CompilerComponent on:program={handleProgram} />
	</div>
</div>

<style>
	.row {
		display: flex;
		flex-direction: row;
		width: 100%;
	}
	.icon-button {
		background: none;
		border: none;
		padding: 0;
		cursor: pointer;
	}
</style>
