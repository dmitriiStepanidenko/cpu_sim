use crate::cpu::{AddressingMode, Command, RegisterAddress};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, char, multispace0, multispace1},
    combinator::map_res,
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;

type Data = HashMap<String, Vec<u8>>;
type Loop = HashMap<String, u8>;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct Program {
    data: Data,
    text: Vec<Command>,
    loop_: Loop,
}

impl Program {
    pub fn get_commands(&self) -> &Vec<Command> {
        &self.text
    }
    pub fn get_data(&self) -> &Data {
        &self.data
    }
    pub fn get_direct_address(&self, v: &str) -> Option<u8> {
        let mut ptr: u8 = 0;
        for (label, values) in &self.data {
            if label.as_str() == v {
                return Some(ptr);
            }
            ptr += values.len() as u8;
        }
        return None;
    }
    pub fn set_loop(&mut self, label: &str) {
        self.loop_
            .insert(label.to_string(), self.text.len() as u8 * 4);
    }
    pub fn get_loop(&mut self, label: &str) -> Option<u8> {
        self.loop_.get(label).copied()
    }

    pub fn push_text(&mut self, cmd: Command) {
        self.text.push(cmd)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Program {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn text_len(&self) -> usize {
        self.text.len()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct ParserErrror {
    data: ParserErrorsTypes,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl ParserErrror {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn is_known_error(&self) -> bool {
        match self.data {
            ParserErrorsTypes::UnknownError => return false,
            _ => return true,
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn get_error_position(&self) -> String {
        match &self.data {
            ParserErrorsTypes::UnknownError => return "".to_string(),
            ParserErrorsTypes::ParsingError(position) => return position.clone(),
        }
    }
}

pub enum ParserErrorsTypes {
    ParsingError(String),
    UnknownError,
}

#[wasm_bindgen]
pub fn compile_program(text: String) -> Result<Program, ParserErrror> {
    let program = Program::default();
    match parse_program(&text, program) {
        Err(e) => match e {
            nom::Err::Failure(fail) => {
                return Err(ParserErrror {
                    data: ParserErrorsTypes::ParsingError(fail.input.to_string()),
                })
            }
            nom::Err::Error(fail) => {
                return Err(ParserErrror {
                    data: ParserErrorsTypes::ParsingError(fail.input.to_string()),
                })
            }
            _ => {
                return Err(ParserErrror {
                    data: ParserErrorsTypes::UnknownError,
                })
            }
        },
        Ok((_, program)) => return Ok(program),
    }
}

pub fn parse_program<'a>(input: &'a str, program: Program) -> IResult<&'a str, Program> {
    let (input, program) = parse_section(input, program)?;
    let mut result = program;
    let mut current_input = input;
    loop {
        if current_input.len() == 0 {
            return Ok((current_input, result));
        }
        (current_input, result) = parse_section(current_input, result.clone())?;
    }
}

// Helper function to parse an identifier (alphanumeric string)
fn identifier(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

// Helper function to parse a number
fn number(input: &str) -> IResult<&str, u8> {
    map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| s.parse())(input)
}

// Define a parser for the "X dup(Y)" construct
fn dup(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, times) = preceded(multispace0, number)(input)?;
    let (input, _) = preceded(multispace0, tag("dup"))(input)?;
    let (input, _) = preceded(multispace0, tag("("))(input)?;
    let (input, value) = preceded(multispace0, number)(input)?;
    let (input, _) = preceded(multispace0, tag(")"))(input)?;

    // Create a vector with `times` copies of `value`
    let values = vec![value as u8; times.into()];

    Ok((input, values))
}

fn parse_comma_number(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, result) = separated_list0(
        preceded(multispace0, tag(",")),
        preceded(multispace0, number),
    )(input)?;
    return Ok((input, result));
}

fn parse_define_byte(input: &str) -> IResult<&str, (&str, Vec<u8>)> {
    // parse the label
    let (input, label) = terminated(preceded(multispace0, alphanumeric1), multispace1)(input)?;

    // parse the "db" keyword
    let (input, _) = tag("db")(input)?;

    let (input, result) = alt((dup, parse_comma_number))(input)?;

    Ok((input, (label, result)))
}

fn parse_section<'a>(input: &'a str, mut program: Program) -> IResult<&'a str, Program> {
    let (input, _) = preceded(multispace0, tag("section"))(input)?;
    let (input, section) = preceded(preceded(multispace0, tag(".")), identifier)(input)?;
    let mut input_out = input;
    match section {
        "data" => {
            let (input, data) = parse_data_section(input_out)?;
            program.data = data.into_iter().collect();
            input_out = input;
        }
        "text" => {
            let (input, text) = parse_text_section(input_out, &mut program)?;
            program.text = text.into_iter().collect();
            input_out = input;
        }
        _ => panic!("Unknown section!"),
    }
    let (input_out, _) = multispace0(input_out)?;
    return Ok((input_out, program));
}

fn parse_text_section<'a>(
    input: &'a str,
    program_input: &mut Program,
) -> IResult<&'a str, Vec<Command>> {
    let program = Rc::new(RefCell::new(program_input.clone()));
    let (input, _) = multispace0(input)?;
    let mut current_input = input;

    loop {
        let parse_result = alt((
            |input| parse_loop(input, program.clone()),
            |input| parse_mov(input, program.clone()),
            |input| parse_store(input, program.clone()),
            |input| parse_inc(input, program.clone()),
            |input| parse_mul(input, program.clone()),
            |input| parse_add(input, program.clone()),
            |input| parse_jmp(input, program.clone()),
        ))(current_input);

        match parse_result {
            Ok((next_input, res)) => {
                program.as_ref().borrow_mut().text.push(res);
                current_input = next_input;
            }

            Err(_) => {
                if let Ok((next_input, _)) = parse_loop_label(current_input, program.clone()) {
                    current_input = next_input;
                } else {
                    break;
                }
            }
        }
    }

    *program_input = (program.borrow()).clone();
    let res = program.as_ref().borrow().text.clone();
    Ok((current_input, res))
}

/// CX - loop counter
fn parse_register_pc(input: &str) -> IResult<&str, RegisterAddress> {
    let (input, _) = preceded(multispace0, tag("CX"))(input)?;
    return Ok((input, RegisterAddress::CX));
}

/// GP - geneal purpose
fn parse_register_gp(input: &str) -> IResult<&str, RegisterAddress> {
    let (input, _) = preceded(multispace0, tag("R"))(input)?;
    let (input, number) = number(input)?;
    return Ok((input, RegisterAddress::GP(number)));
}

fn parse_register_address(input: &str) -> IResult<&str, RegisterAddress> {
    alt((parse_register_gp, parse_register_pc))(input)
}

fn parse_addressing_mode_immediate<'a>(
    input: &'a str,
    _: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    let (input, number) = preceded(multispace0, number)(input)?;
    return Ok((input, AddressingMode::Immediate(number)));
}

fn parse_addressing_mode_immediate_direct_address<'a>(
    input: &'a str,
    program: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    let (input, label) = identifier(input)?;
    let number = match program.borrow().get_direct_address(label) {
        None => {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::Fail,
            )))
        }
        Some(x) => x,
    };
    return Ok((input, AddressingMode::Immediate(number)));
}

fn parse_addressing_mode_direct<'a>(
    input: &'a str,
    program: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    let (input, _) = multispace0(input)?;
    let (input, label) = terminated(preceded(tag("["), identifier), tag("]"))(input)?;
    let number = program.borrow().get_direct_address(label).unwrap();
    return Ok((input, AddressingMode::Direct(number)));
}

fn parse_addressing_mode_indirect<'a>(
    input: &'a str,
    _program: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    let (input, _) = multispace0(input)?;
    let (input, register) =
        terminated(preceded(tag("["), parse_register_address), tag("]"))(input)?;
    return Ok((input, AddressingMode::Indirect(register)));
}

fn parse_addressing_mode_register<'a>(
    input: &'a str,
    _program: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    let (input, _) = multispace0(input)?;
    let (input, register) = parse_register_address(input)?;
    return Ok((input, AddressingMode::Register(register)));
}

fn parse_addressing_mode<'a>(
    input: &'a str,
    program: Rc<RefCell<Program>>,
) -> IResult<&'a str, AddressingMode> {
    alt((
        |input| parse_addressing_mode_immediate(input, program.clone()),
        |input| parse_addressing_mode_immediate_direct_address(input, program.clone()),
        |input| parse_addressing_mode_indirect(input, program.clone()),
        |input| parse_addressing_mode_register(input, program.clone()),
        |input| parse_addressing_mode_direct(input, program.clone()),
    ))(input)
}
fn parse_inc<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("inc"))(input)?;
    let (input, dst) = parse_addressing_mode(input, program)?;

    return Ok((input, Command::Inc(dst)));
}

fn parse_mov<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("mov"))(input)?;
    let (input, dst) = parse_register_address(input)?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src) = parse_addressing_mode(input, program.clone())?;

    return Ok((input, Command::Mov(dst, src)));
}

fn parse_store<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("store"))(input)?;
    let (input, dst) = parse_addressing_mode(input, program.clone())?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src) = parse_register_address(input)?;

    return Ok((input, Command::Store(dst, src)));
}

fn parse_mul<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("mul"))(input)?;
    let (input, dst) = parse_register_address(input)?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src1) = parse_addressing_mode(input, program.clone())?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src2) = parse_addressing_mode(input, program.clone())?;

    return Ok((input, Command::Mul(dst, src1, src2)));
}

fn parse_add<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("add"))(input)?;
    let (input, dst) = parse_register_address(input)?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src1) = parse_addressing_mode(input, program.clone())?;
    let (input, _) = terminated(char(','), multispace0)(input)?;
    let (input, src2) = parse_addressing_mode(input, program.clone())?;

    return Ok((input, Command::Add(dst, src1, src2)));
}

fn parse_jmp<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = preceded(multispace0, tag("jmp"))(input)?;
    let (input, src) = parse_addressing_mode(input, program.clone())?;

    return Ok((input, Command::Jmp(src)));
}

fn parse_data_section(input: &str) -> IResult<&str, Vec<(String, Vec<u8>)>> {
    let mut result = vec![];
    let mut new_input = input;
    while let Ok((input, res)) = parse_define_byte(new_input) {
        result.push((String::from(res.0), res.1));
        new_input = input;
    }
    return Ok((new_input, result));
}

fn parse_loop<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, Command> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("loop")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, label) = identifier(input)?;
    let address = match program.as_ref().borrow_mut().get_loop(label) {
        None => {
            return Err(nom::Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::Fail,
            )))
        }
        Some(x) => x,
    };
    return Ok((input, Command::Loop(address)));
}

fn parse_loop_label<'a>(input: &'a str, program: Rc<RefCell<Program>>) -> IResult<&'a str, ()> {
    let (input, _) = multispace0(input)?;
    let (input, label) = terminated(identifier, char(':'))(input)?;
    program.as_ref().borrow_mut().set_loop(label);
    return Ok((input, ()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_data() {
        let test_string = r#"
            section .data
            array1 db 1, 2, 3, 4, 5
            array2 db 6, 7, 8, 9, 10
            result db 5 dup(0)
            sum db 0"#;
        assert_eq!(
            Ok((
                "",
                Program {
                    data: vec![
                        ("array1".to_string(), vec![1u8, 2u8, 3u8, 4u8, 5u8]),
                        ("array2".to_string(), vec![6u8, 7u8, 8u8, 9u8, 10u8]),
                        ("result".to_string(), vec![0u8, 0u8, 0u8, 0u8, 0u8]),
                        ("sum".to_string(), vec![0u8]),
                    ]
                    .into_iter()
                    .collect(),
                    text: vec![],
                    ..Default::default()
                }
            )),
            parse_section(test_string, Program::default())
        );
    }

    #[test]
    fn parse_text() {
        let test_string = r#"section .data
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
                mov R4, [R0] 
                mul R4, R4, [R1] 
                store [R2], R4 

                inc R0 
                inc R1 
                inc R2 
            loop multiply

            mov CX, 5 

            sum:
                mov R10, result 
                add R10, sum, R10
                inc [result]
            loop summ
            "#;

        let program = Program::default();
        let (_, v) = parse_program(test_string, program).unwrap();
        assert_eq!(
            v,
            Program {
                data: vec![
                    ("array1".to_string(), vec![1u8, 2u8, 3u8, 4u8, 5u8]),
                    ("array2".to_string(), vec![6u8, 7u8, 8u8, 9u8, 10u8]),
                    ("result".to_string(), vec![0u8, 0u8, 0u8, 0u8, 0u8]),
                    ("sum".to_string(), vec![0u8]),
                ]
                .into_iter()
                .collect(),
                text: vec![
                    Command::Mov(RegisterAddress::CX, AddressingMode::Immediate(5)),
                    Command::Mov(
                        RegisterAddress::GP(0),
                        AddressingMode::Immediate(v.get_direct_address("array1").unwrap())
                    ),
                    Command::Mov(
                        RegisterAddress::GP(1),
                        AddressingMode::Immediate(v.get_direct_address("array2").unwrap())
                    ),
                    Command::Mov(
                        RegisterAddress::GP(2),
                        AddressingMode::Immediate(v.get_direct_address("result").unwrap())
                    ),
                    Command::Mov(RegisterAddress::GP(4), AddressingMode::Direct(0)),
                ],
                ..Default::default()
            }
        )
    }

    #[test]
    fn test_define_byte() {
        let test_string = r#"array1 db 1, 2, 3, 4, 5"#;
        assert_eq!(
            Ok(("", ("array1", vec![1u8, 2u8, 3u8, 4u8, 5u8],))),
            parse_define_byte(test_string),
        );
        let test_string = r#"
            array2 db 5 dup(0)"#;
        assert_eq!(
            Ok(("", ("array2", vec![0u8, 0u8, 0u8, 0u8, 0u8],))),
            parse_define_byte(test_string),
        );
        let test_string = r#"sum db 0"#;
        assert_eq!(
            Ok(("", ("sum", vec![0u8],))),
            parse_define_byte(test_string),
        );
    }

    #[test]
    fn test_parse_comma_number() {
        let test_string = r#"1, 2, 3, 4, 5"#;
        assert_eq!(
            Ok(("", vec![1u8, 2u8, 3u8, 4u8, 5u8],)),
            parse_comma_number(test_string),
        );
    }

    #[test]
    fn test_parse_inc() {
        let program = Program::default();
        let test_string = r#"inc R0"#;
        assert_eq!(
            Ok((
                "",
                Command::Inc(AddressingMode::Register(RegisterAddress::GP(0))),
            )),
            parse_inc(test_string, Rc::new(RefCell::new(program))),
        );
    }

    #[test]
    fn test_parse_loop() {
        let mut program = Program::default();
        let test_string = r#"
            label:
                mov R0, 5
            loop label"#;
        let _o = parse_text_section(test_string, &mut program).unwrap();
        assert_eq!(
            program.text,
            vec![
                Command::Mov(RegisterAddress::GP(0), AddressingMode::Immediate(5)),
                Command::Loop(0),
            ]
        );
    }

    #[test]
    fn test_parse_mov() {
        let program = Rc::new(RefCell::new(Program::default()));
        let test_string = r#"mov R0, 5"#;
        assert_eq!(
            Ok((
                "",
                Command::Mov(RegisterAddress::GP(0), AddressingMode::Immediate(5)),
            )),
            parse_mov(test_string, program.clone()),
        );
        {
            let mut prog = program.as_ref().borrow_mut();
            prog.data.insert(String::from("array0"), vec![0, 1, 2]);
            prog.data.insert(String::from("array1"), vec![0, 1]);
        }
        let test_string = r#"mov R1, array1"#;
        let addr = program.borrow().get_direct_address("array1").unwrap();
        assert_eq!(
            Ok((
                "",
                Command::Mov(RegisterAddress::GP(1), AddressingMode::Immediate(addr)),
            )),
            parse_mov(test_string, program.clone()),
        );
        let test_string = r#"mov R2, [R0]"#;
        assert_eq!(
            Ok((
                "",
                Command::Mov(
                    RegisterAddress::GP(2),
                    AddressingMode::Indirect(RegisterAddress::GP(0))
                ),
            )),
            parse_mov(test_string, program.clone()),
        );
    }

    #[test]
    fn test_parse_dup() {
        let test_string = r#"5 dup(1)"#;
        assert_eq!(Ok(("", vec![1u8, 1u8, 1u8, 1u8, 1u8],)), dup(test_string));
    }
}
