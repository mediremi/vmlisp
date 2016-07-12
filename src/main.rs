#[macro_use]
extern crate enum_primitive;

mod compiler;
mod vm;

fn main() {
    let program = "(print (- (+ (/ 200 2) 2) 60))".to_string();
    vm::execute(compiler::compile(program));
}
