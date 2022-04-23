pub mod instructions;
mod program_buffer;
use program_buffer::Program;

fn main() {
    let program: String = String::from("👇🤜👇👇👇👇👇👇👇👉👆👈🤛👉👇👊👇🤜👇👉👆👆👆👆👆👈🤛👉👆👆👊👆👆👆👆👆👆👆👊👊👆👆👆👊");
    let app = Program::new(program);
    dbg!(app.exec());
}
