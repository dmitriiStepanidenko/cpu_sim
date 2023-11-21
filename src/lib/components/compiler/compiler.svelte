<script lang="ts">
	import type { Program } from '$lib/pkg/cpu_sim_rs';
	import { compile_program, ParserErrror } from '$lib/pkg/cpu_sim_rs';
	import { createEventDispatcher } from 'svelte';
	import { addToast } from '$lib/Toaster.svelte';

	const dispatch = createEventDispatcher();
	let textarea;
	let error = false;
	$: rows = countNewLines(text);
	let numbers;
	let minHeight = 10;
	let maxHeight = 80;
	let highlitedNumber: undefined | number = undefined;

	$: {
		numbers = [];
		for (let i = 0; i <= rows; i++) {
			numbers.push(i);
		}
	}

	function sendToParent() {
		dispatch('program', program);
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
      create_success("Compiled successfully", "");
			sendToParent();
		} catch (e: unknown) {
			if (e instanceof ParserErrror) {
				error = true;
				highlitedNumber = rows - countNewLines(e.get_error_position());
        create_error("Compilation error", `Line with error: ${highlitedNumber}`);
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
</script>

<div>
	<div class="main_input">
		<div class="numbers">
			{#each numbers as number}
				{#if error && number === highlitedNumber}
          <div class="row" style="background-color: red;color: white;">{number}</div>
				{:else}
					<div class="row">{number}</div>
				{/if}
			{/each}
		</div>
		<div class="container">
			<pre aria-hidden="true" style="min-height: {minHeight}; max-height: {maxHeight}">{text +
					'\n'}</pre>
			<textarea
				cols="80"
				autocomplete="off"
				autocorrect="off"
				autocapitalize="off"
				spellcheck="false"
				bind:value={text}
				bind:this={textarea}
			/>
		</div>
	</div>
	<button on:click={() => compile()}>Compile</button>
</div>

<style>
	.main_input {
		display: flex;
    font-size: 14px;
	}
	.container {
		position: relative;
    font-size: 14px;
    padding-left: 5px;
	}
	.numbers {
		padding-top: 0px;
	}

	pre,
	textarea {
		font-family: inherit;
		/*padding: 0.5em;
		box-sizing: border-box;
		border: 1px solid #eee;
		line-height: 1.2;
    */
		overflow: hidden;
	}

	textarea {
		position: absolute;
		width: 100%;
		height: 100%;
		top: 0;
		resize: none;
	}
</style>
