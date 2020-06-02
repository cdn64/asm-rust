use std::collections::HashMap;

use crate::instruction::*;
use crate::instruction_sequencer::*;
use crate::label::*;
use crate::register::*;
use crate::value::*;

// Computer has a memory which stores values for each Register type and executes Instructions.
// TODO: Extract Memory and reading / writing to own struct

pub struct Computer {
    register_memory: HashMap<Register, i32>,
}
impl Default for Computer {
    fn default() -> Self {
        let mut computer = Self {
            register_memory: HashMap::new(),
        };
        for register in Register::iter() {
            computer.register_memory.insert(*register, 0);
        }
        computer
    }
}
impl Computer {
    pub fn display_ram(&self) {
        for register in Register::iter() {
            print!("{:?}: {:?} ", register, self.register_memory[register]);
        }
        println!();
    }
    pub fn execute(
        &mut self,
        instruction: &Instruction,
        instruction_sequencer: &mut InstructionSequencer,
    ) {
        match instruction {
            Instruction::Add(value) => self.add(value),
            Instruction::Mov(value, register) => self.mov(value, register),
            Instruction::Jmp(label) => self.jmp(label, instruction_sequencer),
            Instruction::Jeq(value, label) => self.jeq(value, label, instruction_sequencer),
        }
    }
    pub fn add(&mut self, value: &Value) {
        let summand_one = self.read(&Register::Acc);
        let summand_two = self.get_value(&value);
        self.write(&Register::Acc, summand_one + summand_two);
    }
    pub fn mov(&mut self, value: &Value, register: &Register) {
        let value = self.get_value(&value);
        self.write(register, value)
    }
    pub fn jmp(&mut self, label: &Label, instruction_sequencer: &mut InstructionSequencer) {
        instruction_sequencer.jump_to_label(label.to_string());
    }
    pub fn jeq(
        &mut self,
        value: &Value,
        label: &Label,
        instruction_sequencer: &mut InstructionSequencer,
    ) {
        let acc_value = self.read(&Register::Acc);
        let value = self.get_value(&value);
        if acc_value == value {
            instruction_sequencer.jump_to_label(label.to_string());
        }
    }
    pub fn get_value(&mut self, value: &Value) -> i32 {
        match value {
            Value::Literal(value) => value.clone(),
            Value::Reference(register) => self.read(register),
        }
    }
    pub fn read(&mut self, register: &Register) -> i32 {
        self.register_memory[register]
    }
    pub fn write(&mut self, register: &Register, value: i32) {
        self.register_memory.insert(*register, value);
    }
}
