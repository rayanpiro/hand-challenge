#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instructions {
    Forward,
    Backward,
    Increment,
    Decrement,
    JEZero(usize),
    JNEZero(usize),
    Print,
}

impl Instructions {
    fn parse_char(instruction: char) -> Self {
        match instruction {
            '👉' => Self::Forward,
            '👈' => Self::Backward,
            '👆' => Self::Increment,
            '👇' => Self::Decrement,
            '🤜' => Self::JEZero(0),
            '🤛' => Self::JNEZero(0),
            '👊' => Self::Print,
            _ => unimplemented!(),
        }
    }

    pub fn parser(program: String) -> Vec<Self> {
        let mut loops = Loops::new();

        // First parse emojis to instructions
        let mut parsed_program: Vec<Self> = program
            .chars()
            .map(|instruction| Self::parse_char(instruction))
            .collect();
        
        // Now filter by the loops and update jump addresses
        let loop_instructions = parsed_program.iter().enumerate().filter(|&(_, instruction)|{
            match instruction {
                Instructions::JEZero(0) | Instructions::JNEZero(0) => true,
                _ => false,
            }
        });

        let mut parsed_program = parsed_program.clone();

        for (index, _) in loop_instructions {
            match parsed_program[index] {
                Instructions::JEZero(0) => loops.init_loop(index),
                Instructions::JNEZero(0) => {
                    let loop_opening = loops.get_open();

                    parsed_program[loop_opening] = Instructions::JEZero(index);
                    parsed_program[index] = Instructions::JNEZero(loop_opening);
                }
                _ => unimplemented!(),
            };
        }

        parsed_program
    }
}

pub struct Loops {
    open: Vec<usize>,
}

impl Loops {
    pub fn new() -> Self {
        Self { open: Vec::new() }
    }

    pub fn init_loop(&mut self, eip: usize) {
        self.open.push(eip);
    }

    pub fn get_open(&mut self) -> usize {
        self.open.pop().unwrap()
    }
}
