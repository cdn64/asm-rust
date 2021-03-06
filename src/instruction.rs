use crate::label::*;
use crate::register::*;
use crate::value::*;

// Instruction represents, rather unsurprisingly, a single instruction that can be evaluated.
// It holds both the operation (type of instruction) as well as all its operands.

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    Mov(Value, Register),
    Add(Value),
    Jmp(Label),
    Jeq(Value, Label),
    Jlt(Value, Label),
}
impl Instruction {
    pub fn from(tokens: Vec<&str>) -> Option<Self> {
        if tokens.len() < 1 {
            return None;
        }
        let operation = tokens.first().unwrap().to_lowercase();
        let operands_count = tokens.len() - 1;
        match (operation.as_str(), operands_count) {
            ("nop", 0) => Some(Self::Nop),
            ("mov", 2) => {
                if let Some(value) = Value::from(tokens[1]) {
                    if let Some(register) = Register::from(tokens[2]) {
                        return Some(Self::Mov(value, register));
                    }
                }
                None
            }
            ("add", 1) => {
                if let Some(value) = Value::from(tokens[1]) {
                    return Some(Self::Add(value));
                }
                None
            }
            ("jmp", 1) => {
                if let Some(label) = Label::from(tokens[1]) {
                    return Some(Self::Jmp(label));
                }
                None
            }
            ("jeq", 2) => {
                if let Some(value) = Value::from(tokens[1]) {
                    if let Some(label) = Label::from(tokens[2]) {
                        return Some(Self::Jeq(value, label));
                    }
                }
                None
            }
            ("jlt", 2) => {
                if let Some(value) = Value::from(tokens[1]) {
                    if let Some(label) = Label::from(tokens[2]) {
                        return Some(Self::Jlt(value, label));
                    }
                }
                None
            }
            _ => None,
        }
    }
}
