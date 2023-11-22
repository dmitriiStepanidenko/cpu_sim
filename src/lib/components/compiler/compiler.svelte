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
	<select on:change={handleSelectionCode}>
		<option value="">Empty</option>
		<option value="convolution">Convolution</option>
		<option value="sum">Sum</option>
	</select>
	<div class="main_input" class:opacity={$opacityStore}>
    <CodeMirror bind:value={program_text} tabSize={2} />
	</div>
	<div>
		<button class="ready" on:click={() => compile()}>Compile</button>
		<button disabled={!encodeButtonActive} class:ready={encodeButtonActive} on:click={encode}
			>Encode</button
		>
	</div>
</div>

<style>
	.main_input {
		font-size: 13px;
    min-width: 100px;
    min-height: 200px;
	}

	.opacity {
		opacity: 0.05;
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
	}

	button:hover.ready {
		background-color: #2ee59d;
		box-shadow: 0px 15px 20px rgba(46, 229, 157, 0.4);
		color: #fff;
		transform: translateY(-1px);
	}

	button:not(.ready) {
		background-color: red;
		color: #fff;
		cursor: default;
	}
</style>
