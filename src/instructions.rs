#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instructions {
    Forward,
    Backward,
    Increment,
    Decrement,
    JEZero,
    JNEZero,
    Print,
}

impl Instructions {
    pub fn parser(instruction: char) -> Self {
        match instruction {
            '👉' => Self::Forward,
            '👈' => Self::Backward,
            '👆' => Self::Increment,
            '👇' => Self::Decrement,
            '🤜' => Self::JEZero,
            '🤛' => Self::JNEZero,
            '👊' => Self::Print,
            _ => unimplemented!()
        }
    }
}
