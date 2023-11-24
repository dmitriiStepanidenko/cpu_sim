const convolution = `section .data
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

  store 15, R4

  jmp 0
`;

const sum = `section .data
  array1 db 1, 2, 3, 4, 5
  array2 db 6, 7, 8, 9, 10
  result db 5 dup(0)
  sum db 0

section .text
  mov CX, 5 
  mov R0, array1 
  mov R1, array2 
  mov R2, result 
  summ:
    mov R3, [R0] 
    add R3, R3, [R1] 
    store [R2], R3

    inc R0 
    inc R1 
    inc R2 
  loop summ

  jmp 0
`;

export const program_variants: { [key: string]: any } = { 'convolution': convolution, 'sum': sum, '': '\n\n\n\n\n\n\n\n\n' };
