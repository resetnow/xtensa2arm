use std::vec::Vec;

use assembly::{Instruction, InstructionKind, InstructionArch, ParseInstruction};
use translation::xtensa_op::{XtensaOpcode, XtensaInstruction};
use translation::xtensa_operand::XtensaOperand;
use function::Function;
use r2pipe::R2Pipe;

#[derive(Default)]
pub struct Translator {
    pub referenced_objects: Vec<u32>,
    pub functions: Vec<Function>,
}

impl Translator {
    fn get_arm_reg_str(xtensa_reg: u8) -> String {
        let result = match xtensa_reg {
            0 => "lr",
            1 => "sp",
            2 => "r0",
            3 => "r1",
            4 => "r2",
            5 => "r3",
            6 => "r4",
            7 => "r5",
            8 => "r6",
            9 => "r7",
            10 => "r8",
            11 => "r9",
            12 => "r10",
            13 => "r11",
            14 => "r12",
            _ => { panic!("Unknown/unsupported xtensa register referenced: {:}", xtensa_reg); }
        };
        result.to_string()
    }

    /// Emit opcodes: 3 register operands
    fn emit_r3(instruction: &Instruction, function: &mut Function) {

    }

    /// Emit opcodes: 2 register operands, 1 immediate
    fn emit_r2(instruction: &Instruction, function: &mut Function) {

    }

    pub fn new() -> Translator {
        Default::default()
    }

    pub fn translate(&mut self, function: &mut Function) {
        // let mut result = Function::new();

        for instruction in &mut function.instructions {
            let mut xtensa_instruction = XtensaInstruction::new();
            xtensa_instruction.from_str(&instruction.opcode);
            instruction.arch = InstructionArch::Xtensa(xtensa_instruction);
        }
    }
}
