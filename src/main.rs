pub mod instructions;
mod program_buffer;
use program_buffer::Program;

fn main() {
    let program_path = "./test2.hand";
    let app = Program::from_file(program_path);
    dbg!(app.exec());
}
