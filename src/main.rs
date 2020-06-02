mod computer;
mod instruction;
mod instruction_sequencer;
mod label;
mod program;
mod register;
mod value;

use computer::*;
pub use instruction::*;
use instruction_sequencer::*;
pub use label::*;
use program::*;
pub use register::*;
pub use value::*;

const MAX_EXECUTION_COUNT: i32 = 30;

fn main() {
    let program = Program::from_file("src/test.asm");
    let mut computer: Computer = Default::default();
    let mut instruction_sequencer: InstructionSequencer = Default::default();
    let mut execution_count = 0;

    computer.display_ram();
    instruction_sequencer.load(program);
    loop {
        match instruction_sequencer.next() {
            None => break,
            Some(line) => {
                println!("{:?}: {:?}", instruction_sequencer.program_counter, &line);
                computer.execute(&line.instruction, &mut instruction_sequencer);
                computer.display_ram();
            }
        }

        // Avoid infinite loops
        execution_count += 1;
        if execution_count >= MAX_EXECUTION_COUNT {
            println!(
                "⚠️ Stopped execution after {:?} instructions",
                execution_count
            );
            break;
        }
    }
}
