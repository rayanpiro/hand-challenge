#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<char>,
    registries: Vec<u8>,
    EIP: usize,
    End_EIP: usize,
}

use std::fs;

use crate::instructions::Instructions;

impl Program {
    pub fn new(program: String) -> Self {
        Self {
            program: program.chars().collect::<Vec<char>>(),
            registries: Vec::new(),
            EIP: 0,
            End_EIP: program.len(),
        }
    }

    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        Self::new(content.expect(&format!("An error has ocurred reading the file {}", path)))
    }

    pub fn exec(self) {
        while self.EIP != self.End_EIP {
            match Instructions::parser(self.program[self.EIP]) {
                Instructions::Forward => self.forward(),
                Instructions::Backward => self.backward(),
                Instructions::Increment => self.increment(),
                Instructions::Decrement => self.decrement(),
                Instructions::JEZero => self.jezero(),
                Instructions::JNEZero => self.jnezero(),
                Instructions::Print => self.print(),
                _ => unimplemented!(),
            }
        }
    }

    fn forward(&self) {
        todo!()
    }

    fn backward(&self) {
        todo!()
    }

    fn increment(&self) {
        todo!()
    }

    fn decrement(&self) {
        todo!()
    }

    fn jezero(&self) {
        todo!()
    }

    fn jnezero(&self) {
        todo!()
    }

    fn print(&self) {
        todo!()
    }
}
