use crate::label::*;
use crate::register::*;
use crate::value::*;

// Instruction represents, rather unsurprisingly, a single instruction that can be evaluated.
// It holds both the operation (type of instruction) as well as all its operands.

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov(Value, Register),
    Add(Value),
    Jmp(Label),
    Jeq(Value, Label),
}
impl Instruction {
    pub fn from(line: &str) -> Option<Self> {
        let tokens: Vec<&str> = Self::cleanup(line).split_whitespace().collect();
        if tokens.len() < 1 {
            return None;
        }
        let operation = tokens.first().unwrap().to_lowercase();
        let operands_count = tokens.len() - 1;
        match (operation.as_str(), operands_count) {
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
            _ => None,
        }
    }
    fn cleanup(line: &str) -> &str {
        line.split("# ").collect::<Vec<&str>>().first().unwrap()
    }
}
