use crate::program::{count_vars, NodeF64, ProgramError};
use crate::binary_op::{BinaryOpF64};
use crate::ternary_op::{TernaryOpF64};

use core::fmt::{Write};

#[derive(Debug)]
struct Compiler {
    program: String,
    position: usize,
}

const REG_COUNT: usize = 16;

impl Compiler {
    fn new() -> Self {
        Self {
            program: String::new(),
            position: 0,
        }
    }
    fn compile(&mut self, target: usize, program: &[NodeF64]) -> Result<usize, ProgramError> {
        let node = program[self.position];
        // println!("p={}, n={:?} t={}", self.position, node, target);
        self.position += 1;
        match node {
            NodeF64::Local(index) => {
                self.locals.get(index as usize).copied()
                    .ok_or(ProgramError::NonExistentLocal)
            }
            NodeF64::Input(index) => {
                self.inputs.get(index as usize).copied()
                    .ok_or(ProgramError::NonExistentInput)
            }
            NodeF64::Constant(index) => {
                self.emit_load(target, index as usize);
                Ok(target)
            }
            NodeF64::Lettuce => {
                let register = self.alloc(RegState::Local)?;
                // writeln!(&mut self.program, "; local={}", register);
                self.locals.push(register);
                self.compile(register, program)?;
                // writeln!(&mut self.program, "; in...");
                self.compile(target, program)
            }
            NodeF64::BinaryOp(op) => {
                let lhs = self.compile(target, program)?;
                let tmp = self.alloc(RegState::Temp)?;
                let rhs = self.compile(tmp, program)?;
                self.emit_binary(op, target, lhs, rhs);
                self.free(tmp)?;
                Ok(lhs)
            }
            NodeF64::TernaryOp(op) => {
                let mut a = self.compile(target, program)?;
                let mut tmp_a = 0;
                if a != target {
                    tmp_a = self.alloc(RegState::Temp)?;
                    self.emit_mov(tmp_a, a);
                    a = tmp_a;
                }
                let tmp_b = self.alloc(RegState::Temp)?;
                let b = self.compile(tmp_b, program)?;
                let tmp_c = self.alloc(RegState::Temp)?;
                let c = self.compile(tmp_c, program)?;
                self.emit_ternary(op, target, a, b, c);
                if tmp_a != 0 {
                    self.emit_mov(target, a);
                }
                self.free(tmp_a)?;
                self.free(tmp_b)?;
                self.free(tmp_c)?;
                Ok(a)
            }
        }
    }
}

pub fn compile(program: &[NodeF64]) -> Result<String, ProgramError> {
    let mut compiler = Compiler::new();
    compiler.init(program)?;
    let dst = compiler.compile(0, program)?;
    compiler.emit_ret();
    Ok(compiler.program)
}

pub fn to_fn() -> (fn(x: f64, consts: *const f64) -> f64) {
    let code = &[
        0x0F, 0x28, 0xD8, 0xF2, 0x0F, 0x59, 0xDB, 0x0F, 0x28, 0xE3, 0xF2, 0x0F, 0x59, 0xE4, 0xF2, 0x0F, 0x10, 0x52, 0x28, 0x0F, 0x28, 0xCB, 0xF2, 0x0F, 0x59, 0xCA, 0xF2, 0x0F, 0x10, 0x52, 0x20, 0xF2, 0x0F, 0x58, 0xD1, 0x0F, 0x28, 0xCB, 0xF2, 0x0F, 0x59, 0xCC, 0xF2, 0x0F, 0x10, 0x62, 0x18, 0x0F, 0x28, 0xEB, 0xF2, 0x0F, 0x59, 0xEC, 0xF2, 0x0F, 0x10, 0x62, 0x10, 0xF2, 0x0F, 0x58, 0xE5, 0xF2, 0x0F, 0x59, 0xCC, 0xF2, 0x0F, 0x58, 0xD1, 0x0F, 0x28, 0xCB, 0xF2, 0x0F, 0x59, 0xCA, 0xF2, 0x0F, 0x10, 0x52, 0x08, 0xF2, 0x0F, 0x58, 0xD1, 0x0F, 0x28, 0xCB, 0xF2, 0x0F, 0x59, 0xC8, 0xF2, 0x0F, 0x59, 0xDA, 0xF2, 0x0F, 0x10, 0x12, 0xF2, 0x0F, 0x58, 0xD3, 0xF2, 0x0F, 0x59, 0xCA, 0xF2, 0x0F, 0x58, 0xC1, 0xC3 
    ];
    unsafe {
        use winapi::um::memoryapi::{VirtualAlloc};
        use winapi::um::winnt::{self};

        let raw_addr = VirtualAlloc(
            core::ptr::null_mut(),
            code.len(),
            winnt::MEM_RESERVE | winnt::MEM_COMMIT,
            winnt::PAGE_EXECUTE_READWRITE);
        let slice = core::slice::from_raw_parts_mut(raw_addr as *mut u8, code.len());
        slice.copy_from_slice(code);
        core::mem::transmute::<_, _>(raw_addr)
    }
}
