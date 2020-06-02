use std::slice::Iter;

// Register represents a register from which can be read and to which can be written
// This struct additionally provides Register::iter() for looping through all possible registers

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    Acc,
    B,
}
impl Register {
    pub fn iter() -> Iter<'static, Register> {
        [Self::Acc, Self::B].iter()
    }
    pub fn from(token: &str) -> Option<Register> {
        match token.to_lowercase().as_str() {
            "acc" => Some(Register::Acc),
            "b" => Some(Register::B),
            _ => None,
        }
    }
}
