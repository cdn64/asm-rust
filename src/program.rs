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
            let tokens = Self::tokenize(Self::uncomment(&line));
            if tokens.len() >= 1 {
                let first_token = tokens.first().unwrap();
                if first_token.ends_with(":") {
                    label = Some(String::from(first_token.trim_end_matches(":")));
                } else {
                    if let Some(instruction) = Instruction::from(tokens) {
                        program.lines.push(Line { instruction, label });
                        label = None;
                    }
                }
            }
        }
        program
    }
    fn uncomment(line: &str) -> &str {
        line.split(";").collect::<Vec<&str>>().first().unwrap()
    }
    fn tokenize(line: &str) -> Vec<&str> {
        line.split_whitespace().collect()
    }
}
