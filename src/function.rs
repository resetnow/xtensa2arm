use std::vec::Vec;
use std::default::Default;

use rustc_serialize::json;
use assembly::{Instruction, InstructionKind};

#[derive(Default)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub name: String,
}

impl Function {
    pub fn new() -> Function {
        Default::default()
    }

    pub fn from_json(&mut self, json: json::Json) {
        let json_object = json.into_object().unwrap();
        let json_ops = json_object.get("ops").cloned().unwrap();
        let array_ops = json_ops.into_array().unwrap();

		for element in &array_ops {
            let json_object = element.as_object().unwrap();
            let mut instruction = Instruction::new();

		    for (key, value) in json_object.iter() {
                match key as &str {
                    "opcode" => { instruction.opcode = String::from(value.as_string().unwrap()) }
                    "offset" => { instruction.offset = value.as_u64().unwrap() as u32 }
                    "type" => {
                        instruction.kind = match value.as_string().unwrap() as &str {
                            "store" => InstructionKind::Store,
                            "load" => InstructionKind::Load,
                            "cjmp" => {
                                let jump = json_object.get("jump").unwrap();
                                let address = jump.as_u64().unwrap() as u32;
                                InstructionKind::BranchImm { target: address }
                            }
                            _ => InstructionKind::Other
                        }
                    }
                    _ => {}
                }
            }

            self.instructions.push(instruction);
        }
    }
}
