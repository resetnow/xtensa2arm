
pub enum InstructionKind {
    Load,
    Store,
    Other,
}

#[derive(Default)]
pub struct Instruction {
    pub offset: u32,
    pub opcode: String,
    pub kind: InstructionKind,
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
