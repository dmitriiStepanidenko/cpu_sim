use crate::parser::Program;
use crate::wasm::CommandWrapper;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::{debug, error, info, warn};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub enum MemoryType {
    Registers,
    Command,
    Data,
}

#[wasm_bindgen]
pub struct Cpu {
    command_memory: Rc<RefCell<Memory>>,
    data_memory: Rc<RefCell<Memory>>,
    registers: Rc<RefCell<Memory>>,
}

#[wasm_bindgen]
impl Cpu {
    #[wasm_bindgen(constructor)]
    pub fn new(
        command_memory_size: Option<usize>,
        data_memory_size: Option<usize>,
        registers_size: Option<usize>,
    ) -> Self {
        cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            console_error_panic_hook::set_once();
            tracing_wasm::set_as_global_default_with_config(
                tracing_wasm::WASMLayerConfigBuilder::new()
                    .set_max_level(tracing::Level::DEBUG)
                    .build(),
            );
        } else {
            tracing_subscriber::fmt::init()
        }
        }

        let command_memory = Rc::new(RefCell::new(Memory::new(command_memory_size)));
        let data_memory = Rc::new(RefCell::new(Memory::new(data_memory_size)));
        let registers = Rc::new(RefCell::new(Memory::new(registers_size)));

        Self {
            command_memory,
            data_memory,
            registers,
        }
    }

    #[wasm_bindgen]
    pub fn encode(&mut self, program: &Program) {
        // 1. Encode data
        let mut ptr: u8 = 0;
        for (_label, values) in program.get_data().iter() {
            for value in values.iter() {
                self.data_memory.borrow_mut().write_u8(ptr as usize, *value);
                ptr += 1;
            }
        }

        // 2. Encode commands
        let mut ptr: u8 = 0;
        for command in program.get_commands().iter() {
            self.command_memory
                .borrow_mut()
                .write_u32((ptr * 4) as usize, command.encode());
            ptr += 1;
        }
    }

    #[wasm_bindgen]
    pub fn get_memory(&self, mem_type: MemoryType) -> SharedMemory {
        let memory = match mem_type {
            MemoryType::Registers => Rc::clone(&self.registers),
            MemoryType::Data => Rc::clone(&self.data_memory),
            MemoryType::Command => Rc::clone(&self.command_memory),
        };

        SharedMemory(memory)
    }

    #[wasm_bindgen]
    pub fn do_op(&mut self) -> Result<(), CpuErrors> {
        debug!("do_op before fetch");
        // 1. Fetch
        self.fetch();
        debug!("do_op after fetch");
        // 2. Decode
        let command = self.decode()?;
        println!("command: {:?}", command);
        debug!("do_op after decode");
        // 3. Execute
        self.execute(command);
        debug!("do_op after execute");
        // 4. Update
        self.update();
        debug!("do_op after update");
        return Ok(());
    }

    fn fetch(&mut self) -> (u8, u32) {
        // Read the next instruction by address pointed by the RegisterAddress::PC
        // and place it's value to IR
        let pc = self.registers.borrow().read_u8(RegisterAddress::PC.addr());
        println!("pc : {}", pc);
        let command = self.command_memory.borrow().read_u32(pc as usize);
        println!("command: {:b}", command);
        self.registers
            .borrow_mut()
            .write_u32(RegisterAddress::IR.addr(), command);
        return (pc, command);
    }
    fn decode(&mut self) -> Result<Command, CpuErrors> {
        // Read current instruction from RegisterAddress::IR and decode it
        return Command::decode(self.registers.borrow().read_u32(RegisterAddress::IR.addr()));
    }
    fn execute(&mut self, command: Command) {
        // Execute decoded command
        println!("{:?}", command);
        command.execute(
            &mut self.data_memory.borrow_mut(),
            &mut self.registers.borrow_mut(),
        );
    }
    fn update(&mut self) {
        let jmp_val = self.registers.borrow().read_u8(RegisterAddress::JMP.addr());
        if jmp_val == 1 {
            self.registers
                .borrow_mut()
                .write_u8(RegisterAddress::JMP.addr(), 0);
            return;
        }
        let counter = self.registers.borrow().read_u8(RegisterAddress::PC.addr());
        self.registers
            .borrow_mut()
            .write_u8(RegisterAddress::PC.addr(), counter + 4);
        //}
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum RegisterAddress {
    /// Program Counter
    PC,
    /// Instruction Register
    IR,
    /// Loop counter
    CX,
    /// JMP register. If jamp was = 1, if not = 0
    JMP,
    /// General-purpose registers, where u8 is the register number (e.g., 0 for R0, 1 for R1, etc.)
    GP(u8),
}

impl RegisterAddress {
    pub const SPECIAL_REGS_LEN: u8 = 7;
    /// Return actual address of register
    pub fn addr(&self) -> usize {
        match self {
            RegisterAddress::PC => 0,  // 0-0
            RegisterAddress::IR => 1,  // 1-4
            RegisterAddress::CX => 5,  // 5-5
            RegisterAddress::JMP => 6, // 6-6
            RegisterAddress::GP(reg_num) => (Self::SPECIAL_REGS_LEN as usize + *reg_num as usize), // Assuming GP registers start at index 2
        }
    }

    pub fn from(value: u32) -> (Self, u32) {
        let reg = match value as u8 {
            0 => Self::PC,
            1 => Self::IR,
            2 => Self::CX,
            3 => Self::JMP,
            4_u8..=u8::MAX => Self::GP(value as u8 - 4),
        };
        return (reg, value >> 8);
    }

    pub fn encode(&self, mut value: u32, position: u32) -> (u32, u32) {
        let reg_value: u8 = match self {
            Self::PC => 0,
            Self::IR => 1,
            Self::CX => 2,
            Self::JMP => 3,
            Self::GP(reg_value) => *reg_value as u8 + 4,
        };

        // Shift the register value back to its original position
        let reg_value: u32 = (reg_value as u32) << position;

        // Combine the value with the register value to reconstruct the original u32
        value |= reg_value;

        (value, 8 + position)
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub enum CpuErrors {
    UnknownAddressingMode,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum AddressingMode {
    /// Number
    Immediate(u8),
    /// Address
    /// MOV R0, \[123\] - mov from addr 123 to R0
    Direct(u8),
    /// Address in register
    /// MOV R0, \[R1\] - mov from addr R1 to R0
    Indirect(RegisterAddress),
    /// Register
    /// MOV R0, R1 - obvious
    Register(RegisterAddress),
    /// MOV R0, [R1+R2]
    RegisterIndirect(RegisterAddress, RegisterAddress),
}

impl AddressingMode {
    pub fn from(value: u32) -> Result<(Self, u32), CpuErrors> {
        let bits_value = (value as u8) & 0b_0000_0111;
        let position = 3;
        println!("value = {:b}", value);
        println!("Addressing mode value = {:b}", bits_value);
        match bits_value {
            0 => return Ok((Self::Immediate((value >> 3) as u8), value >> (3 + 8))),
            1 => return Ok((Self::Direct((value >> 3) as u8), value >> (3 + 8))),
            2 => {
                let (register, value) = RegisterAddress::from(value >> position);
                return Ok((Self::Indirect(register), value));
            }
            3 => {
                let (register, value) = RegisterAddress::from(value >> position);
                return Ok((Self::Register(register), value));
            }
            4 => {
                let (dst_register, value) = RegisterAddress::from(value >> position);
                let (src_register, value) = RegisterAddress::from(value);
                return Ok((Self::RegisterIndirect(dst_register, src_register), value));
            }
            _ => return Err(CpuErrors::UnknownAddressingMode),
        }
    }
    pub fn read_u8(&self, data_memory: &mut Memory, registers: &mut Memory) -> u8 {
        return match self {
            Self::Immediate(number) => *number,
            Self::Direct(address) => data_memory.read_u8(*address as usize),
            Self::Indirect(address) => {
                data_memory.read_u8(registers.read_u8(address.addr()) as usize)
            }
            Self::Register(register) => registers.read_u8(register.addr()),
            Self::RegisterIndirect(reg1, reg2) => data_memory
                .read_u8((registers.read_u8(reg1.addr()) + registers.read_u8(reg2.addr())).into()),
        };
    }
    pub fn write_u8(&self, value: u8, data_memory: &mut Memory, registers: &mut Memory) {
        match self {
            Self::Direct(address) => data_memory.write_u8(*address as usize, value),
            Self::Indirect(address) => {
                data_memory.write_u8(registers.read_u8(address.addr()) as usize, value)
            }
            Self::Register(register) => registers.write_u8(register.addr(), value),
            Self::RegisterIndirect(reg1, reg2) => data_memory.write_u8(
                (registers.read_u8(reg1.addr()) + registers.read_u8(reg2.addr())).into(),
                value,
            ),
            _ => panic!(),
        }
    }
    pub fn encode(&self, mut value: u32, position: u32) -> (u32, u32) {
        let mut reg_value: u32 = 0;

        let mut length = 3; // adressing code length
        match self {
            Self::Immediate(addr) => {
                reg_value = 0;
                reg_value |= (*addr as u32) << length;
                length += 8;
            }
            Self::Direct(addr) => {
                reg_value = 1;
                reg_value |= (*addr as u32) << length;
                length += 8;
            }
            Self::Indirect(addr) => {
                reg_value = 2;
                reg_value |= addr.encode(reg_value, length).0;
                length += 8;
                //value |= reg_value;
            }
            Self::Register(addr) => {
                reg_value = 3;
                reg_value |= addr.encode(reg_value, length).0;
                length += 8;
            }
            //Self::RegisterIndirect(addr, addr) => {}
            _ => panic!("unimplemented!"),
        };
        reg_value = reg_value << position;
        value |= reg_value;

        (value, length + position)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub enum Command {
    /// LOAD reg, addr_mode - load data from memory at addr_mode into reg.
    Load(RegisterAddress, AddressingMode),
    /// STORE addr_mode, reg - store data from reg into memory at addr_mode.
    Store(AddressingMode, RegisterAddress),

    /// ADD Rdest, Rsrc1, Rsrc2 - add values from Rsrc1 and Rsrc2, store result in Rdest.
    Add(RegisterAddress, AddressingMode, AddressingMode),
    /// MUL Rdest, Rsrc1, Rsrc2 - multiply values from Rsrc1 and Rsrc2, store result in Rdest.
    Mul(RegisterAddress, AddressingMode, AddressingMode),

    /// MOV reg1, addr_mode - copy data from addr_mode to reg1.
    Mov(RegisterAddress, AddressingMode),
    /// MOVI reg, val - load immediate value val into reg.
    Movi(RegisterAddress, u8),
    /// INC addr_mode - increment the value at addr_mode by one.
    Inc(AddressingMode),

    /// LOOP addr - decrement loop counter (usually CX) by one, if not zero, jump to addr.
    Loop(u8), // LOOP typically uses a direct jump address, not a register.

    /// JMP addr_mode - unconditional jump to addr_mode.
    Jmp(AddressingMode),
    // INT n - execute interrupt number n.
    // Int(u8), // INT typically takes an immediate interrupt number.
}

impl Command {
    const LOAD: u8 = 0;
    const STORE: u8 = 1;
    const ADD: u8 = 2;
    const MUL: u8 = 3;
    const MOV: u8 = 4;
    const MOVI: u8 = 5;
    const INC: u8 = 6;

    const LOOP: u8 = 7;
    const JMP: u8 = 8;
    const INT: u8 = 9;

    pub fn encode(&self) -> u32 {
        let mut result: u32;
        let mut position = 4;
        match self {
            Self::Store(dst, src) => {
                result = Self::STORE as u32;
                (result, position) = dst.encode(result, position);
                (result, _) = src.encode(result, position);
            }
            Self::Add(dst, src1, src2) => {
                result = Self::ADD as u32;
                (result, position) = dst.encode(result, position);
                (result, position) = src1.encode(result, position);
                (result, _) = src2.encode(result, position);
            }
            Self::Mul(dst, src1, src2) => {
                result = Self::MUL as u32;
                (result, position) = dst.encode(result, position);
                (result, position) = src1.encode(result, position);
                (result, _) = src2.encode(result, position);
            }
            Self::Mov(dst, src) => {
                result = Self::MOV as u32;
                (result, position) = dst.encode(result, position);
                (result, _) = src.encode(result, position);
            }
            Self::Inc(dst) => {
                result = Self::INC as u32;
                (result, _) = dst.encode(result, position);
            }
            Self::Loop(addr) => {
                result = Self::LOOP as u32;
                let reg_val: u32 = (*addr as u32) << position;
                result |= reg_val;
            }
            _ => unimplemented!(),
        };

        return result;
    }

    pub fn decode(encode_command: u32) -> Result<Command, CpuErrors> {
        // Operand  = 4 b
        // AddressingMode = 3 b
        // Address = 8 b
        let operand = (encode_command & 0b0000_1111) as u8;
        let value = encode_command >> 4;
        match operand {
            Self::LOAD => {
                let (dst, value) = RegisterAddress::from(value);
                let (src, _value) = AddressingMode::from(value)?;
                return Ok(Self::Load(dst, src));
            }
            Self::STORE => {
                let (dst, value) = AddressingMode::from(value)?;
                let (src, _value) = RegisterAddress::from(value);
                return Ok(Self::Store(dst, src));
            }
            Self::ADD => {
                let (dst, value) = RegisterAddress::from(value);
                let (src1, value) = AddressingMode::from(value)?;
                let (src2, _value) = AddressingMode::from(value)?;
                return Ok(Self::Add(dst, src1, src2));
            }
            Self::MUL => {
                let (dst, value) = RegisterAddress::from(value);
                let (src1, value) = AddressingMode::from(value)?;
                let (src2, _value) = AddressingMode::from(value)?;
                return Ok(Self::Mul(dst, src1, src2));
            }
            Self::MOV => {
                let (dst, value) = RegisterAddress::from(value);
                let (src, _value) = AddressingMode::from(value)?;
                return Ok(Self::Mov(dst, src));
            }
            Self::MOVI => {
                let (dst, value) = RegisterAddress::from(value);
                let src = value as u8;
                return Ok(Self::Movi(dst, src));
            }
            Self::INC => {
                let (dst, _value) = AddressingMode::from(value)?;
                return Ok(Self::Inc(dst));
            }
            Self::LOOP => {
                let src = value as u8;
                return Ok(Self::Loop(src));
            }
            Self::JMP => {
                let (dst, _value) = AddressingMode::from(value)?;
                return Ok(Self::Jmp(dst));
            }
            _ => unimplemented!(),
        };
    }

    pub fn execute(&self, data_memory: &mut Memory, registers: &mut Memory) {
        match self {
            Self::Load(dst, src) => {
                let src_value = src.read_u8(data_memory, registers);
                registers.write_u8(dst.addr(), src_value);
            }
            Self::Store(dst, src) => {
                let src_value = registers.read_u8(src.addr());
                dst.write_u8(src_value, data_memory, registers);
            }
            Self::Add(dst, src1, src2) => {
                let src1_value = src1.read_u8(data_memory, registers);
                let src2_value = src2.read_u8(data_memory, registers);
                registers.write_u8(dst.addr(), src1_value + src2_value);
            }
            Self::Mul(dst, src1, src2) => {
                let src1_value = src1.read_u8(data_memory, registers);
                let src2_value = src2.read_u8(data_memory, registers);
                registers.write_u8(dst.addr(), src1_value * src2_value);
            }
            Self::Mov(dst, src) => {
                let src_value = src.read_u8(data_memory, registers);
                registers.write_u8(dst.addr(), src_value);
            }
            Self::Movi(dst, src_value) => {
                registers.write_u8(dst.addr(), *src_value);
            }
            Self::Inc(dst) => {
                let value = dst.read_u8(data_memory, registers); //registers.read_u8(dst.addr());
                dst.write_u8(value + 1, data_memory, registers);
                //registers.write_u8(dst.addr(), value + 1);
            }
            Self::Loop(addr) => {
                let counter = registers.read_u8(RegisterAddress::CX.addr()) - 1;
                registers.write_u8(RegisterAddress::CX.addr(), counter);
                if counter != 0 {
                    registers.write_u8(RegisterAddress::PC.addr(), *addr);
                    registers.write_u8(RegisterAddress::JMP.addr(), 1);
                }
            }
            Self::Jmp(addr) => {
                let addr_value = addr.read_u8(data_memory, registers);
                registers.write_u8(RegisterAddress::PC.addr(), addr_value);
                registers.write_u8(RegisterAddress::JMP.addr(), 1);
            }
        };
    }
}

#[wasm_bindgen]
pub struct SharedMemory(Rc<RefCell<Memory>>);

#[wasm_bindgen]
impl SharedMemory {
    #[wasm_bindgen]
    pub fn subscribe(&mut self, callback: js_sys::Function) -> JsValue {
        self.0.borrow_mut().subscribe(callback)
    }
    #[wasm_bindgen]
    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.0.borrow_mut().write_u8(addr, value)
    }
    #[wasm_bindgen]
    pub fn read_u8(&self, addr: usize) -> u8 {
        self.0.borrow().read_u8(addr)
    }
    #[wasm_bindgen]
    pub fn write_u32(&mut self, addr: usize, value: u32) {
        self.0.borrow_mut().write_u32(addr, value)
    }
    #[wasm_bindgen]
    pub fn read_u32(&self, addr: usize) -> u32 {
        self.0.borrow().read_u32(addr)
    }
    #[wasm_bindgen]
    pub fn write_all(&mut self, data: Uint8Array) {
        self.0.borrow_mut().write_all(data)
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    subscribers: Rc<RefCell<HashMap<u64, js_sys::Function>>>,
    next_subscriber: u64,
}

impl Memory {
    pub fn new(value: Option<usize>) -> Self {
        Memory {
            data: vec![0; value.unwrap_or(0)],
            subscribers: Rc::new(RefCell::new(HashMap::new())),
            next_subscriber: 0,
        }
    }

    fn check_out_of_bounds(&self, addr: usize, msg: &str) {
        if addr >= self.data.len() {
            panic!("{msg}");
        }
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.check_out_of_bounds(addr, "read_u8 out of bounds");
        return self.data[addr];
    }

    pub fn read_u32(&self, addr: usize) -> u32 {
        self.check_out_of_bounds(addr + 3, "read_u32 out of bounds");

        (self.data[addr] as u32)
            | ((self.data[addr + 1] as u32) << 8)
            | ((self.data[addr + 2] as u32) << 16)
            | ((self.data[addr + 3] as u32) << 24)
    }

    pub fn write_u32(&mut self, addr: usize, value: u32) {
        self.check_out_of_bounds(addr + 3, "write_u32 out of bounds");
        self.data[addr] = value as u8;
        self.data[addr + 1] = (value >> 8) as u8;
        self.data[addr + 2] = (value >> 16) as u8;
        self.data[addr + 3] = (value >> 24) as u8;
        #[cfg(target_arch = "wasm32")]
        {
            self.notify_subscribers();
        }
    }

    pub fn write_all(&mut self, data: Uint8Array) {
        self.data = data.to_vec();
        #[cfg(target_arch = "wasm32")]
        {
            self.notify_subscribers();
        }
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.check_out_of_bounds(addr, "write_u8 out of bounds");
        self.data[addr] = value;
        #[cfg(target_arch = "wasm32")]
        {
            self.notify_subscribers();
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn notify_subscribers(&mut self) {
        let array: Uint8Array = Uint8Array::from(self.data.as_slice());
        let js_array: JsValue = array.into();
        for (_, func) in self.subscribers.borrow().iter() {
            //let _ = func.call1(&JsValue::null(), &JsValue::from(self.data.clone()));
            let _ = func.call1(&JsValue::null(), &js_array);
        }
    }

    pub fn subscribe(&mut self, callback: js_sys::Function) -> JsValue {
        let array: Uint8Array = Uint8Array::from(self.data.as_slice());
        let js_array: JsValue = array.into();
        let _ = callback.call1(&JsValue::null(), &js_array);
        let key = self.next_subscriber;
        self.next_subscriber += 1;
        self.subscribers.borrow_mut().insert(key, callback);

        let subscribers = self.subscribers.clone();
        let unsubscribe = Closure::once_into_js(move || {
            subscribers.borrow_mut().remove(&key);
        });
        return unsubscribe;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_loop() {
        let cmd = Command::Loop(0b_0101_0101);
        let encode = cmd.encode();
        println!("encoded:{:032b}", encode);
        assert_eq!(Command::decode(encode).unwrap(), cmd);
    }

    #[test]
    fn encode_decode_mov() {
        let cmd = Command::Mov(RegisterAddress::GP(11), AddressingMode::Direct(15));
        let encode = cmd.encode();
        println!("encoded:{:032b}", encode);
        assert_eq!(Command::decode(encode).unwrap(), cmd);
    }

    #[test]
    fn encode_decode_mul() {
        // Проблема с регистровым типом!!!
        let cmd = Command::Mul(
            RegisterAddress::GP(11),
            AddressingMode::Register(RegisterAddress::GP(11)),
            AddressingMode::Register(RegisterAddress::GP(11)),
        );
        let encode = cmd.encode();
        println!("encoded:{:032b}", encode);
        assert_eq!(Command::decode(encode).unwrap(), cmd);
    }

    #[test]
    fn encode_decode_inc() {
        let cmd = Command::Inc(AddressingMode::Register(RegisterAddress::GP(4)));
        let encode = cmd.encode();
        println!("encoded:{:032b}", encode);
        assert_eq!(Command::decode(encode).unwrap(), cmd);
    }

    #[test]
    fn encode_decode_store() {
        let cmd = Command::Store(
            AddressingMode::Indirect(RegisterAddress::IR),
            RegisterAddress::PC,
        );
        let encode = cmd.encode();
        println!("encoded:{:032b}", encode);
        assert_eq!(Command::decode(encode).unwrap(), cmd);
    }

    #[test]
    fn encode_register_address() {
        let address = RegisterAddress::GP(101u8);
        let value: u32 = 0;
        let (result, _) = address.encode(value, 0);
        assert_eq!(0b1101001, result & (u8::MAX as u32));
        let value: u32 = 0;
        let (result, _) = address.encode(value, 2);
        assert_eq!(0b1101001_00 as u32, result);
        let value: u32 = 3;
        let (result, _) = address.encode(value, 2);
        assert_eq!(0b1101001_11 as u32, result);
    }

    #[test]
    fn decode_register_address() {
        let (address, value) = RegisterAddress::from(0b1101001);
        let expected_address = RegisterAddress::GP(101u8);
        assert_eq!(address, expected_address);
        assert_eq!(value, 0);

        let (address, value) = RegisterAddress::from(0b11_01101001);
        let expected_address = RegisterAddress::GP(101u8);
        assert_eq!(address, expected_address);
        assert_eq!(value, 3);
    }

    #[test]
    fn encode_addressing_mode_immediate() {
        let addressing_mode = AddressingMode::Immediate(101u8);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 0);
        assert_eq!(0b1100101000, result);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 2);
        assert_eq!(0b110010100000 as u32, result);
        let value: u32 = 2;
        let (result, _) = addressing_mode.encode(value, 2);
        assert_eq!(0b110010100010 as u32, result);
    }

    #[test]
    fn decode_addressing_mode_immediate() {
        let (addressing_mode, value) = AddressingMode::from(0b1100101000).unwrap();
        let expected_addressing_mode = AddressingMode::Immediate(101u8);
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 0);

        let (addressing_mode, value) = AddressingMode::from(0b10_01100101000).unwrap();
        let expected_addressing_mode = AddressingMode::Immediate(101u8);
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 2);
    }

    #[test]
    fn encode_addressing_mode_direct() {
        let addressing_mode = AddressingMode::Direct(101u8);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 0);
        assert_eq!(0b1100101_001, result);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 2);
        println!("{:b}", result);
        assert_eq!(0b1100101_001_00 as u32, result);
        let value: u32 = 2;
        let (result, _) = addressing_mode.encode(value, 2);
        assert_eq!(0b1100101_001_10 as u32, result);
    }

    #[test]
    fn decode_addressing_mode_direct() {
        let (addressing_mode, value) = AddressingMode::from(0b1100101001).unwrap();
        let expected_addressing_mode = AddressingMode::Direct(101u8);
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 0);

        let (addressing_mode, value) = AddressingMode::from(0b10_01100101001).unwrap();
        let expected_addressing_mode = AddressingMode::Direct(101u8);
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 2);
    }

    #[test]
    fn encode_addressing_mode_indirect() {
        let addressing_mode = AddressingMode::Indirect(RegisterAddress::GP(101u8));
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 0);
        assert_eq!(0b1101001_010, result);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 2);
        println!("{:b}", result);
        assert_eq!(0b1101001_010_00 as u32, result);
        let value: u32 = 2;
        let (result, _) = addressing_mode.encode(value, 2);
        assert_eq!(0b1101001_010_10 as u32, result);
    }

    #[test]
    fn decode_addressing_mode_indirect() {
        let (addressing_mode, value) = AddressingMode::from(0b1101001_010).unwrap();
        let expected_addressing_mode = AddressingMode::Indirect(RegisterAddress::GP(101u8));
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 0);

        let (addressing_mode, value) = AddressingMode::from(0b10_01101001_010).unwrap();
        let expected_addressing_mode = AddressingMode::Indirect(RegisterAddress::GP(101u8));
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 2);
    }

    #[test]
    fn encode_addressing_mode_register() {
        let addressing_mode = AddressingMode::Register(RegisterAddress::GP(101u8));
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 0);
        assert_eq!(0b1101001_011, result);
        let value: u32 = 0;
        let (result, _) = addressing_mode.encode(value, 2);
        println!("{:b}", result);
        assert_eq!(0b1101001_011_00 as u32, result);
        let value: u32 = 2;
        let (result, _) = addressing_mode.encode(value, 2);
        assert_eq!(0b1101001_011_10 as u32, result);
    }

    #[test]
    fn decode_addressing_mode_register() {
        let (addressing_mode, value) = AddressingMode::from(0b1101001011).unwrap();
        let expected_addressing_mode = AddressingMode::Register(RegisterAddress::GP(101u8));
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 0);

        let (addressing_mode, value) = AddressingMode::from(0b10_01101001011).unwrap();
        let expected_addressing_mode = AddressingMode::Register(RegisterAddress::GP(101u8));
        assert_eq!(addressing_mode, expected_addressing_mode);
        assert_eq!(value, 2);
    }

    #[test]
    fn encode_command_store() {
        let command = Command::Store(
            AddressingMode::Register(RegisterAddress::GP(101u8)),
            RegisterAddress::GP(101u8),
        );
        let result = command.encode();
        assert_eq!(0b1101001_01101001_011_0001, result);

        let cmd = Command::Store(
            AddressingMode::Indirect(RegisterAddress::IR),
            RegisterAddress::PC,
        );
        let result = cmd.encode();
        assert_eq!(0b0_00000001_010_0001, result);
    }

    #[test]
    fn decode_command_store() {
        let command = Command::decode(0b1101001_01101001_011_0001).unwrap();
        let expected_command = Command::Store(
            AddressingMode::Register(RegisterAddress::GP(101u8)),
            RegisterAddress::GP(101u8),
        );
        assert_eq!(command, expected_command);

        let command = Command::decode(0b1_00000001_011_0001).unwrap();
        let expected_command = Command::Store(
            AddressingMode::Register(RegisterAddress::IR),
            RegisterAddress::IR,
        );
        assert_eq!(command, expected_command);
    }

    use crate::parser::parse_program;
    #[test]
    fn try_to_run() {
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
                mov R10, [result] 
                add R10, [sum], R10

                inc [result]

            loop sum
            "#;

        let (input, program) = parse_program(test_string, Program::default()).unwrap();
        println!("{:?}", program);
        let mut cpu = Cpu::new(Some(64), Some(64), Some(64));
        cpu.encode(&program);
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        cpu.do_op();
        assert_eq!(0, 1);
    }
}
