#[derive(Debug, Clone)]
pub struct Program {
    program: Vec<Instructions>,
    stack: Vec<u8>,
    eip: usize,
    end_eip: usize,
    esp: usize,
    stack_pointer: usize,
}

use crate::instructions::Instructions;
use std::fs;

impl Program {
    pub fn new(program: String) -> Self {
        let program_parsed = program
            .chars()
            .map(|char| Instructions::parser(char))
            .collect::<Vec<Instructions>>();

        let program_lenght = program_parsed.len();

        Self {
            program: program_parsed,
            stack: Vec::new(),
            eip: 0,
            end_eip: program_lenght,
            esp: 0,
            stack_pointer: 0,
        }
    }

    pub fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path);
        Self::new(content.expect(&format!("An error has ocurred reading the file {}", path)))
    }

    fn next_instruction(&mut self) {
        self.eip += 1;
    }

    pub fn exec(mut self) {
        self.allocate();
        while !self.end_of_program() {
            match self.program[self.eip] {
                Instructions::Forward => self.forward(),
                Instructions::Backward => self.backward(),
                Instructions::Increment => self.increment(),
                Instructions::Decrement => self.decrement(),
                Instructions::JEZero => self.jezero(),
                Instructions::JNEZero => self.jnezero(),
                Instructions::Print => self.print(),
                _ => unimplemented!(),
            };
            self.next_instruction();
        }
    }

    fn no_memory_left(&self) -> bool {
        self.stack_pointer >= self.esp
    }

    fn allocate(&mut self) {
        self.stack.push(0);
        self.esp += 1;
    }

    fn forward(&mut self) {
        self.stack_pointer += 1;
        if self.no_memory_left() {
            self.allocate()
        }
    }

    fn on_base(&self) -> bool {
        self.stack_pointer == 0
    }

    fn backward(&mut self) {
        if !self.on_base() {
            self.stack_pointer -= 1;
        }
    }

    fn increment(&mut self) {
        if self.stack[self.stack_pointer] == u8::MAX {
            self.stack[self.stack_pointer] = 0;
        } else {
            self.stack[self.stack_pointer] += 1;
        }
    }

    fn decrement(&mut self) {
        if self.stack[self.stack_pointer] == u8::MIN {
            self.stack[self.stack_pointer] = 255;
        } else {
            self.stack[self.stack_pointer] -= 1;
        }
    }

    fn end_of_program(&self) -> bool {
        self.eip >= self.end_eip
    }

    fn reached_jnezero(&self) -> bool {
        self.program[self.eip] == Instructions::JNEZero
    }

    fn jezero(&mut self) {
        if self.stack[self.stack_pointer] != 0 {
            return;
        }

        while !self.end_of_program() && !self.reached_jnezero(){
            self.eip += 1;
        }
    }

    fn reached_jezero(&self) -> bool {
        self.program[self.eip] == Instructions::JEZero
    }

    fn jnezero(&mut self) {
        if self.stack[self.stack_pointer] == 0 {
            return;
        }

        while !self.eip >= 0 && !self.reached_jezero() {
            self.eip -= 1;
        }
    }

    fn print(&self) {
        let to_print_char: char = self.stack[self.stack_pointer].try_into().expect("You cannot print a non-ASCII character");
        print!("{}", to_print_char);
    }
}
