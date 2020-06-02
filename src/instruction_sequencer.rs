use crate::program::*;

// InstructionSequencer holds lines of a Program and can iterate through them.

#[derive(Default)]
pub struct InstructionSequencer {
    pub program_counter: usize,
    lines: Vec<Line>,
}
impl InstructionSequencer {
    pub fn load(&mut self, program: Program) {
        self.program_counter = 0;
        self.lines = program.lines;
    }
    pub fn jump_to_label(&mut self, label: String) {
        match self.find_label(&label) {
            None => panic!("Label {:?} not found", label),
            Some(index) => self.program_counter = index,
        }
    }
    fn find_label(&self, label: &String) -> Option<usize> {
        for (index, line) in self.lines.iter().enumerate() {
            if let Some(line_label) = &line.label {
                if *line_label == label.to_string() {
                    return Some(index);
                }
            }
        }
        return None;
    }
    pub fn next(&mut self) -> Option<Line> {
        let line = match self.lines.get(self.program_counter) {
            None => None,
            Some(line) => Some(line.clone()),
        };
        self.program_counter += 1;
        line
    }
}
