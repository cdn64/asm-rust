use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::instruction::*;

#[derive(Debug, Clone)]
pub struct Line {
    pub instruction: Instruction,
    pub label: Option<String>,
}

// Program expresses a series of optionally labeled instructions

#[derive(Default)]
pub struct Program {
    pub lines: Vec<Line>,
}
impl Program {
    pub fn from_file(filename: &str) -> Program {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut program: Program = Default::default();
        let mut label: Option<String> = None;
        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() == 1 {
                let new_label = tokens.first().unwrap();
                if new_label.ends_with(":") {
                    label = Some(String::from(new_label.trim_end_matches(":")));
                }
            } else if tokens.len() >= 2 {
                if let Some(instruction) = Instruction::from(&line) {
                    program.lines.push(Line { instruction, label });
                    label = None;
                }
            }
        }
        program
    }
}
