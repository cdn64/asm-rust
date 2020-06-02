use std::str::FromStr;

use crate::register::*;

// Value represents an instruction operand that evaluates to a i32, either by specifiying a literal
// value or a register to read from

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Literal(i32),
    Reference(Register),
}
impl Value {
    pub fn from(token: &str) -> Option<Self> {
        if token.starts_with("#") {
            Some(Self::Literal(i32::from_str(&token[1..]).unwrap()))
        } else {
            Register::from(token).map(|register| Self::Reference(register))
        }
    }
}
