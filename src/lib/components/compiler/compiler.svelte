<script lang="ts">
	import type { Program } from '$lib/pkg/cpu_sim_rs';
	import { compile_program, ParserErrror} from '$lib/pkg/cpu_sim_rs';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

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
		console.log(text);
		try {
			program = compile_program(text);
			console.log(program);
      sendToParent();
		} catch (e: unknown) {
			if (e instanceof ParserErrror) {
				console.log('Error!');
				console.log(e.is_known_error());
				console.log(e.get_error_position());
			}
		}
	}
</script>

<div>
	<textarea rows="10" cols="80" bind:value={text} />
	<button on:click={() => compile()}>Compile</button>
</div>
