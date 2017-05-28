use translation::xtensa_op::XtensaInstruction;

pub enum InstructionKind {
    Load,
    Store,
    BranchImm { target: u32 },
    Other,
}

pub enum InstructionArch {
    Xtensa,
    Arm,
    Other,
}

pub trait Operand {
    fn get_imm(&self) -> i32;
    fn get_reg(&self) -> u8;
}

pub trait ParseInstruction {
    fn from_str(&mut self, s: &str);
}

#[derive(Default)]
pub struct Instruction {
    pub offset: u32,
    pub opcode: String,
    pub kind: InstructionKind,
    pub arch: InstructionArch,
    pub referenced: bool,
}

impl Instruction {
    pub fn new() -> Instruction {
        Instruction { ..Default::default() }
    }
}

impl Default for InstructionKind {
    fn default() -> InstructionKind {
        InstructionKind::Other
    }
}

impl Default for InstructionArch {
    fn default() -> InstructionArch {
        InstructionArch::Other
    }
}
