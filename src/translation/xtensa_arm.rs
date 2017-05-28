use std::vec::Vec;
use std::collections::BTreeSet;

use assembly::{Instruction, InstructionKind, InstructionArch, ParseInstruction, Operand};
use translation::xtensa_op::{XtensaOpcode, XtensaInstruction};
use translation::xtensa_operand::XtensaOperand;
use function::Function;
use r2pipe::R2Pipe;

#[derive(Default)]
pub struct Translator {
    pub referenced_objects: Vec<u32>,
    pub functions: Vec<Function>,
}

macro_rules! branch {
    ($x:expr, $y:ident) => ({
        let (op_str, ref_addr) = $x;
        $y.insert(ref_addr);
        op_str
    })
}

impl Translator {
    fn arm_reg(&self, xtensa_reg: u8) -> String {
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
    fn emit_r3(&self, instruction: &XtensaInstruction) -> String {
        let opcode = match instruction.opcode {
            XtensaOpcode::Add => "add",
            XtensaOpcode::Sub => "sub",
            XtensaOpcode::And => "and",
            XtensaOpcode::Or => "orr",
            _ => panic!()
        };

        let r1 = self.arm_reg(instruction.operands[0].get_reg());
        let r2 = self.arm_reg(instruction.operands[1].get_reg());
        let r3 = self.arm_reg(instruction.operands[2].get_reg());

        format!("{:} {:}, {:}, {:}", opcode, r1, r2, r3)
    }

    /// Emit opcodes: 2 register operands, 1 immediate
    fn emit_r2_i1(&self, instruction: &XtensaInstruction) -> String {
        let opcode = match instruction.opcode {
            XtensaOpcode::Slri => "lsr",
            XtensaOpcode::Slli => "lsl",
            _ => panic!()
        };

        let r1 = self.arm_reg(instruction.operands[0].get_reg());
        let r2 = self.arm_reg(instruction.operands[1].get_reg());
        let i1 = instruction.operands[2].get_imm();

        format!("{:} {:}, {:}, #{:}", opcode, r1, r2, i1)
    }

    /// Emit opcodes: load/store operations
    fn emit_load_store(&self, instruction: &XtensaInstruction) -> String {
        let opcode = match instruction.opcode {
            XtensaOpcode::L32i => "ldr",
            XtensaOpcode::S32i => "str",
            _ => panic!()
        };

        let r1 = self.arm_reg(instruction.operands[0].get_reg());
        let r2 = self.arm_reg(instruction.operands[1].get_reg());
        let i1 = instruction.operands[2].get_imm();

        format!("{:} {:}, [{:}, #0x{:x}]", opcode, r1, r2, i1)
    }

    /// Emit branch bit set/bit clear
    fn emit_branch_bit(&self, instruction: &XtensaInstruction) -> (String, u32) {
        let opcode = match instruction.opcode {
            XtensaOpcode::Bbci => "beq",
            XtensaOpcode::Bbsi => "bne",
            _ => panic!()
        };

        let r1 = self.arm_reg(instruction.operands[0].get_reg());
        let bit = instruction.operands[1].get_imm();
        let jt = instruction.operands[2].get_imm() as u32;

        ( format!("tst {:}, #0x{:x}\n\t{:} loc_{:x}", r1, (1 << bit), opcode, jt), jt )
    }

    /// Emit PC-relative load.
    /// Replaces load address with actual data since
    /// the load is performed from a read-only memory.
    fn emit_load_relative(&self, instruction: &XtensaInstruction, pipe: &mut R2Pipe) -> String {
        let reg = self.arm_reg(instruction.operands[0].get_reg());
        let address = instruction.operands[1].get_imm() as u32;
        let command = format!("pxwj 32 @ 0x{:x}", address);
        let json_array = pipe.cmdj(&command).unwrap();
        let array = json_array.as_array().unwrap();
        let data = array[0].as_u64().unwrap() as u32;

        format!("ldr {:}, =0x{:x}", reg, data)
    }

    /// Emit function return
    fn emit_ret(&self) -> String {
        "bx lr".to_string()
    }

    /// Emit memory barrier
    fn emit_memw(&self) -> String {
        "".to_string()
    }

    fn translate_instruction(&self, i: &mut Instruction, xtensa_i: &XtensaInstruction,
            refs: &mut BTreeSet<u32>, pipe: &mut R2Pipe) {
        let op = match xtensa_i.opcode {
            XtensaOpcode::Add |
            XtensaOpcode::Sub |
            XtensaOpcode::And |
            XtensaOpcode::Or => { self.emit_r3(xtensa_i) }
            XtensaOpcode::L32i |
            XtensaOpcode::S32i => { self.emit_load_store(xtensa_i) }
            XtensaOpcode::Slli |
            XtensaOpcode::Slri => { self.emit_r2_i1(xtensa_i) }
            XtensaOpcode::Bbci |
            XtensaOpcode::Bbsi => { branch!(self.emit_branch_bit(xtensa_i), refs) }
            XtensaOpcode::L32r => { self.emit_load_relative(xtensa_i, pipe) }
            XtensaOpcode::Ret => { self.emit_ret() }
            XtensaOpcode::Memw => { self.emit_memw() }
            _ => { panic!("translate_instruction: opcode not supported: {:?}", xtensa_i.opcode) }
        };

        i.opcode = op;
    }

    pub fn new() -> Translator {
        Default::default()
    }

    pub fn translate(&mut self, function: &mut Function, pipe: &mut R2Pipe) -> Function {
        let mut result = Function::new();
        let mut refs = BTreeSet::<u32>::new();

        for instruction in &function.instructions {
            let mut xtensa_instruction = XtensaInstruction::new();
            let mut result_instruction = Instruction::new();

            xtensa_instruction.from_str(&instruction.opcode);

            self.translate_instruction(&mut result_instruction, &xtensa_instruction, &mut refs, pipe);

            result_instruction.offset = instruction.offset;
            result_instruction.arch = InstructionArch::Arm;
            result.instructions.push(result_instruction);
        }

        for i in 0..function.instructions.len() {
            if refs.contains(&function.instructions[i].offset) {
                result.instructions[i].referenced = true;
            }
        }

        result.name = function.name.clone();
        result
    }
}
