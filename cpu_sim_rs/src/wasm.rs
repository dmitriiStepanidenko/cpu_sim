use crate::cpu::{AddressingMode, Command, CpuErrors};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum WasmCommandType {
    Load,
    Store,
    Add,
    Mul,
    Mov,
    Movi,
    Inc,
    Loop,
    Jmp,
}

#[wasm_bindgen]
pub struct CommandWrapper {
    command: Command,
}

#[wasm_bindgen]
impl CommandWrapper {
    #[wasm_bindgen]
    pub fn get_data(&self) -> JsValue {
        let serialized = serde_json::to_string(&self.command).unwrap();
        return JsValue::from_str(&serialized);
    }

    #[wasm_bindgen]
    pub fn decode(value: u32) -> Result<CommandWrapper, CpuErrors> {
        Ok(Self {
            command: Command::decode(value)?,
        })
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum WasmAddressingMode {
    Immediate,
    Direct,
    Indirect,
    Register,
    RegisterIndirect,
}

#[wasm_bindgen]
pub struct AddressingModeWrapper {
    address: AddressingMode,
}

#[wasm_bindgen]
impl AddressingModeWrapper {
    #[wasm_bindgen]
    pub fn get_type(&self) -> WasmAddressingMode {
        return match self.address {
            AddressingMode::Immediate(_) => WasmAddressingMode::Immediate,
            AddressingMode::Direct(_) => WasmAddressingMode::Direct,
            AddressingMode::Indirect(_) => WasmAddressingMode::Indirect,
            AddressingMode::Register(_) => WasmAddressingMode::Register,
            AddressingMode::RegisterIndirect(..) => WasmAddressingMode::RegisterIndirect,
        };
    }

    #[wasm_bindgen]
    pub fn get_data(&self) -> JsValue {
        let serialized = serde_json::to_string(&self.address).unwrap();
        return JsValue::from_str(&serialized);
    }
}

// C-style enum for wasm_bindgen compatibility
#[wasm_bindgen]
#[repr(u8)]
pub enum WasmRegisterAddress {
    PC = 0,
    IR = 1,
    CX = 2,
    GP = 3, // General-purpose register placeholder
}

