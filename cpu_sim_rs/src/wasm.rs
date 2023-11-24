use crate::cpu::{AddressingMode, Command, CpuErrors};

use js_sys::wasm_bindgen::JsValue;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct CommandWrapper {
    command: Command,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CommandWrapper {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn get_data(&self) -> JsValue {
        let serialized = serde_json::to_string(&self.command).unwrap();
        return JsValue::from_str(&serialized);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn decode(value: u32) -> Result<CommandWrapper, CpuErrors> {
        Ok(Self {
            command: Command::decode(value)?,
        })
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Copy, Clone)]
pub enum WasmAddressingMode {
    Immediate,
    Direct,
    Indirect,
    Register,
    RegisterIndirect,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct AddressingModeWrapper {
    address: AddressingMode,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl AddressingModeWrapper {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn get_type(&self) -> WasmAddressingMode {
        return match self.address {
            AddressingMode::Immediate(_) => WasmAddressingMode::Immediate,
            AddressingMode::Direct(_) => WasmAddressingMode::Direct,
            AddressingMode::Indirect(_) => WasmAddressingMode::Indirect,
            AddressingMode::Register(_) => WasmAddressingMode::Register,
            AddressingMode::RegisterIndirect(..) => WasmAddressingMode::RegisterIndirect,
        };
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn get_data(&self) -> JsValue {
        let serialized = serde_json::to_string(&self.address).unwrap();
        return JsValue::from_str(&serialized);
    }
}

// C-style enum for wasm_bindgen compatibility
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[repr(u8)]
pub enum WasmRegisterAddress {
    PC = 0,
    IR = 1,
    CX = 2,
    GP = 3, // General-purpose register placeholder
}
