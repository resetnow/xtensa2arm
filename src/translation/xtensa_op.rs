use assembly::ParseInstruction;
use translation::xtensa_operand::{XtensaOperand, XtensaOperandKind};

/// xtensa opcodes enum.
/// Narrow versions are assigned same
/// enum values as wide — it doesn't
/// affect translation in any way
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum XtensaOpcode {
    Add, Addi, And, Sub,
    Or, Slli, Slri, Srai,
    Mov, Movi, Addx2, Addx4,
    Addx8,
    // Flow control
    Bbsi, Bbci, Ret, Jmp,
    // Memory sync barrier
    Memw,
    // Load operations
    L32r, L32i, L16ui,
    L16si, L8ui, S8i,
    // Calls
    Call0, Callx0,
    // Store operations
    S32i,
    Other,
}

/// Holds xtensa opcode and operands
#[derive(Default, PartialEq)]
pub struct XtensaInstruction {
    pub opcode: XtensaOpcode,
    pub operands: Vec<XtensaOperand>,
}

/// Helper struct used to parse assembly tokens
struct InstructionBuilder {
    pub opcode: XtensaOpcode,
    pub operand_kind: Vec<XtensaOperandKind>,
}

/// Macro for creating XtensaInstruction entries
macro_rules! op {
    ($x:ident, [ $( $y:ident ),* ]) => (
        InstructionBuilder {
            opcode: XtensaOpcode::$x,
            operand_kind: vec![ $( XtensaOperandKind::$y(0) ),* ]
        }
    )
}

impl XtensaInstruction {
    pub fn new() -> XtensaInstruction {
        Default::default()
    }
}

impl InstructionBuilder {
    pub fn from_opcode_str(opcode: &str) -> InstructionBuilder {
        match opcode {
            "and"  | "and.n"  => op!(And,   [ Reg, Reg, Reg ]),
            "or"   | "or.n"   => op!(Or,    [ Reg, Reg, Reg ]),
            "add"  | "add.n"  => op!(Add,   [ Reg, Reg, Reg ]),
            "sub"  | "sub.n"  => op!(Sub,   [ Reg, Reg, Reg ]),
            "addx2"           => op!(Addx2, [ Reg, Reg, Reg ]),
            "addx4"           => op!(Addx4, [ Reg, Reg, Reg ]),
            "addx8"           => op!(Addx8, [ Reg, Reg, Reg ]),
            "ret"  | "ret.n"  => op!(Ret,   []),
            "l32i" | "l32i.n" => op!(L32i,  [ Reg, Reg, Imm ]),
            "l32r"            => op!(L32r,  [ Reg, Imm ]),
            "l8ui"            => op!(L8ui,  [ Reg, Reg, Imm ]),
            "l16ui"           => op!(L16ui, [ Reg, Reg, Imm ]),
            "l16si"           => op!(L16si, [ Reg, Reg, Imm ]),
            "s32i" | "s32i.n" => op!(S32i,  [ Reg, Reg, Imm ]),
            "s8i"             => op!(S8i,   [ Reg, Reg, Imm ]),
            "slli"            => op!(Slli,  [ Reg, Reg, Imm ]),
            "slri"            => op!(Slri,  [ Reg, Reg, Imm ]),
            "srai"            => op!(Srai,  [ Reg, Reg, Imm ]),
            "bbsi"            => op!(Bbsi,  [ Reg, Imm, Imm ]),
            "bbci"            => op!(Bbci,  [ Reg, Imm, Imm ]),
            "addi"            => op!(Addi,  [ Reg, Reg, Imm ]),
            "mov" | "mov.n"   => op!(Mov,   [ Reg, Reg ]),
            "movi" | "movi.n" => op!(Movi,  [ Reg, Imm ]),
            "call0"           => op!(Call0, [ Imm ]),
            "callx0"          => op!(Callx0,[ Reg ]),
            "j"               => op!(Jmp,   [ Imm ]),
            "memw"            => op!(Memw,  []),
            _ => { panic!("Opcode not supported: {:?}", opcode); }
        }
    }

    pub fn build<'a, I>(&self, tokens: I) -> XtensaInstruction
    where I: Iterator<Item=&'a str> {
        let mut operands = Vec::<XtensaOperand>::new();
        let t: Vec<&str> = tokens.collect();

        for i in 0 .. self.operand_kind.len() {
            let token = t[i];

            operands.push(
                match self.operand_kind[i] {
                    XtensaOperandKind::Reg(_) => XtensaOperand::new(XtensaOperandKind::Reg(0), token),
                    XtensaOperandKind::Imm(_) => XtensaOperand::new(XtensaOperandKind::Imm(0), token),
                    _ => panic!("Unknown operand type")
                }
            )
        }

        XtensaInstruction {
            opcode: self.opcode,
            operands: operands
        }
    }
}

impl ParseInstruction for XtensaInstruction {
    fn from_str(&mut self, s: &str) {
        let mut tokens = s
            .split(|c| c == ' ' || c == ',')
            .filter(|s| !s.is_empty());

        let opcode = tokens.nth(0).unwrap();
        let builder = InstructionBuilder::from_opcode_str(opcode);
        let instruction = builder.build(tokens);

        self.opcode = instruction.opcode;
        self.operands = instruction.operands;
    }
}

impl Default for XtensaOpcode {
    fn default() -> XtensaOpcode {
        XtensaOpcode::Other
    }
}

#[cfg(test)]
mod tests {
    use translation::xtensa_op::{XtensaInstruction, XtensaOpcode, XtensaOperandKind, XtensaOperand};
    use assembly::ParseInstruction;

    #[test]
    #[should_panic]
    fn test_parse_1() {
        let mut i = XtensaInstruction::new();

        i.from_str("");
    }

    #[test]
    fn test_parse_2() {
        let mut i = XtensaInstruction::new();

        i.from_str("and a1, a2, a3");

        assert!(i == XtensaInstruction {
            opcode: XtensaOpcode::And,
            operands: vec![
                XtensaOperand { kind: XtensaOperandKind::Reg(1) },
                XtensaOperand { kind: XtensaOperandKind::Reg(2) },
                XtensaOperand { kind: XtensaOperandKind::Reg(3) },
            ],
        });
    }

    #[test]
    fn test_parse_3() {
        let mut i = XtensaInstruction::new();

        i.from_str("l32r a14, 0xaabbccdd");

        assert!(i == XtensaInstruction {
            opcode: XtensaOpcode::L32r,
            operands: vec![
                XtensaOperand { kind: XtensaOperandKind::Reg(14) },
                XtensaOperand { kind: XtensaOperandKind::Imm(0xaabbccdd) },
            ],
        });
    }
}
