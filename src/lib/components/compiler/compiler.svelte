<script lang="ts">
	import type { Program } from '$lib/pkg/cpu_sim_rs';
	import { compile_program, ParserErrror } from '$lib/pkg/cpu_sim_rs';
	import { createEventDispatcher } from 'svelte';
	import { addToast } from '$lib/Toaster.svelte';
	import CodeMirror from 'svelte-codemirror-editor';
	import { opacityStore } from '$lib/opacity_store';
	import { program_variants } from './programs';

	export let encodeFunction: Function;
	const dispatch = createEventDispatcher();

	$: rows = countNewLines(program_text);
	let numbers;
	let highlitedNumber: undefined | number = undefined;
	let encodeButtonActive = false;
	let program_text: string = program_variants[''];

	$: {
		numbers = [];
		for (let i = 0; i <= rows; i++) {
			numbers.push(i);
		}
	}

	function sendToParent() {
		dispatch('program', program);
	}

	$: {
		if (program != undefined) {
			encodeButtonActive = true;
		} else {
			encodeButtonActive = false;
		}
	}

	let program: Program | undefined;
	let selectedProgram: string;

	function handleSelectionCode(event: Event & { currentTarget: HTMLSelectElement }) {
		selectedProgram = event.currentTarget.value;
		console.log(selectedProgram);
		console.log(program_variants[selectedProgram]);
		program_text = program_variants[selectedProgram];
		console.log(program_text);
	}

	function compile() {
		try {
			program = compile_program(program_text);
			sendToParent();
			create_success('Compiled successfully', '');
		} catch (e: unknown) {
			program = undefined;
			if (e instanceof ParserErrror) {
				highlitedNumber = rows - countNewLines(e.get_error_position());
				create_error('Compilation error', `Line with error: ${highlitedNumber + 1}`);
			} else {
				console.log('uknown error!');
				console.log(e);
			}
		}
	}

	function countNewLines(inputStr: string): number {
		return inputStr.split('\n').length - 1;
	}

	function create_success(title: string, description: string) {
		addToast({
			data: {
				title,
				description,
				color: 'green'
			}
		});
	}
	function create_error(title: string, description: string) {
		addToast({
			data: {
				title,
				description,
				color: 'red'
			}
		});
	}

	function encode() {
		encodeFunction(program);
	}
</script>

<div class="main">
	<div class="select">
		<p>Select a program template:</p>
		<select on:change={handleSelectionCode}>
			<option value="">Empty</option>
			<option value="convolution">Convolution</option>
			<option value="sum">Sum</option>
		</select>
	</div>
	<div class="main_input" class:opacity={$opacityStore}>
		<CodeMirror bind:value={program_text} tabSize={2} />
	</div>
	<div class="buttons">
		<button class="ready" on:click={() => compile()}>Compile</button>
		<button disabled={!encodeButtonActive} class:ready={encodeButtonActive} on:click={encode}
			>Encode</button
		>
	</div>
</div>

<style>
  .select {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }
	select {
		background-color: var(--base-color);
		color: var(--primary-color);
		border: 0.2px solid var(--secondary-color);
		border-radius: 15px;
    margin-left: 5px;
    text-align-last: center;
	}
	option {
		margin: 5px;
	}

	.main_input {
		font-size: 13px;
		min-width: 100px;
		min-height: 200px;

		border-radius: 15px;
		border: 1px solid var(--secondary-color);
	}

	:global(.ͼ2 .cm-gutters) {
		color: var(--primary-color);
		border-right: 1px solid var(--secondary-color);
	}

	:global(.ͼ2 .cm-gutters :not(.cm-activeLineGutter)) {
		background-color: var(--base-color);
	}

	:global(.cm-activeLineGutter) {
		background-color: var(--secondary-color) !important;
		caret-color: transparent !important;
	}

	.opacity {
		opacity: 0.01;
	}

	.buttons {
		margin-top: 50px;
	}

	button {
		border: none;
		padding: 10px 15px;
		text-align: center;
		text-decoration: none;
		display: inline-block;
		text-transform: uppercase;
		box-shadow: 0px 8px 15px rgba(0, 0, 0, 0.1);
		font-weight: 500;
		cursor: pointer;
		border-radius: 30px;
		background-color: var(--base-color);
	}

	button.ready {
		background-color: var(--action-color);
	}

	button:hover.ready {
		background-color: #2ee59d;
		box-shadow: 0px 15px 20px rgba(46, 229, 157, 0.4);
		transform: translateY(-1px);
	}

	button:not(.ready) {
		background-color: var(--secondary-color);
		cursor: default;
	}
</style>
