pub mod cpu;
pub mod parser;
pub mod wasm;

#[cfg(test)]
mod tests {
    use super::cpu::*;
    use super::parser::*;
    use super::*;

    fn test() {
        let test_string = r#"section .data
                array1 db 1, 2, 3, 4, 5
                array2 db 6, 7, 8, 9, 10
                result db 5 dup(0)
                sum db 0
            section .text
                mov CX, 5 
                mov R1, array1 
                mov R2, array2 
                mov R3, result 
            multiply:
                mov R4, [R1] 
                mul R5, R5, [R2] 
                store [R3], R4 

                inc R1 
                inc R2 
                inc R3 
            loop multiply

            mov CX, 5 

            sum:
                mov R10, result 
                add R10, sum, R10

                inc result 

            loop sum
            "#;

        let (input, program) = parse_program(test_string, Program::default()).unwrap();
        let mut cpu = Cpu::new(Some(32), Some(32), Some(8));
        cpu.encode(&program);
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
    }
}
