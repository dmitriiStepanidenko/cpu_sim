<script lang="ts">
	import type { Program } from '$lib/pkg/cpu_sim_rs';
	import { compile_program, ParserErrror } from '$lib/pkg/cpu_sim_rs';
	import { createEventDispatcher } from 'svelte';
	import { addToast } from '$lib/Toaster.svelte';
	import CodeMirror from 'svelte-codemirror-editor';
  import {opacityStore} from '$lib/opacity_store'

	export let encodeFunction: Function;
	const dispatch = createEventDispatcher();
	let error = false;
	$: rows = countNewLines(text);
	let numbers;
	let highlitedNumber: undefined | number = undefined;
	let encodeButtonActive = false;

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
	let text: string = `section .data
                array1 db 1, 2, 3, 4, 5
                array2 db 6, 7, 8, 9, 10
                result db 5 dup(0)
                sum db 0

            section .text
                mov CX, 5 
                mov R0, array1 
                mov R1, array2 
                mov R2, result 
            multiply:
                mov R3, [R0] 
                mul R3, R3, [R1] 
                store [R2], R3

                inc R0 
                inc R1 
                inc R2 
            loop multiply

            mov CX, 5 
            mov R0, array1 
            mov R1, array2 
            mov R2, result 
            mov R4, 0
            sum:
                add R4, R4, [R2]

                inc R2

            loop sum
  `;

	function compile() {
		try {
			program = compile_program(text);
			error = false;
			sendToParent();
			create_success('Compiled successfully', '');
		} catch (e: unknown) {
			error = true;
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
  <div class="main_input" class:opacity={$opacityStore}>
		<CodeMirror bind:value={text} />
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
		display: flex;
		font-size: 14px;
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
