use std::default::Default;
use assembly::Operand;

#[derive(PartialEq)]
pub enum XtensaOperandKind {
    Reg(u8),
    Imm(u32),
    Unknown,
}

#[derive(PartialEq)]
pub struct XtensaOperand {
    pub kind: XtensaOperandKind,
}

impl XtensaOperand {
    fn get_reg_number(s: &str) -> u8 {
        let number_string: String = s.chars().skip(1).collect();
        number_string.parse::<u8>().unwrap()
    }

    pub fn new(k: XtensaOperandKind, s: &str) -> XtensaOperand {
        XtensaOperand { kind: match k {
            XtensaOperandKind::Reg(_) => {
                let reg = XtensaOperand::get_reg_number(s);
                XtensaOperandKind::Reg(reg)
            }
            XtensaOperandKind::Imm(_) => {
                let value: u32;

                if s.starts_with("0x") {
                    value = u32::from_str_radix(&s[2..], 16).unwrap();
                } else {
                    value = s.parse::<i32>().unwrap() as u32;
                }

                XtensaOperandKind::Imm(value)
            },
            XtensaOperandKind::Unknown => panic!("Unknown operand kind")
        }}
    }
}

impl Default for XtensaOperand {
    fn default() -> XtensaOperand {
        XtensaOperand { kind: XtensaOperandKind::Unknown }
    }
}

impl Operand for XtensaOperand {
    fn get_imm(&self) -> i32 {
        match self.kind {
            XtensaOperandKind::Imm(i) => i as i32,
            _ => panic!("called get_imm on non-immediate operand")
        }
    }

    fn get_reg(&self) -> u8 {
        match self.kind {
            XtensaOperandKind::Reg(i) => i,
            _ => panic!("called get_imm on non-immediate operand")
        }
    }
}

#[cfg(test)]
mod tests {
    use translation::xtensa_operand::{XtensaOperand, XtensaOperandKind};
    use assembly::Operand;

    #[test]
    #[should_panic]
    fn test_operand_imm_1() {
        XtensaOperand::new(XtensaOperandKind::Imm(0), "");
    }

    #[test]
    fn test_operand_imm_2() {
        let o = XtensaOperand::new(XtensaOperandKind::Imm(0), "-333");
        assert_eq!(o.get_imm(), -333);
    }

    #[test]
    #[should_panic]
    fn test_operand_reg_1() {
        XtensaOperand::new(XtensaOperandKind::Reg(0), "");
    }

    #[test]
    fn test_operand_reg_2() {
        let o = XtensaOperand::new(XtensaOperandKind::Reg(0), "a15");
        assert_eq!(o.get_reg(), 15);
    }
}
