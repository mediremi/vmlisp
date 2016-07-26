use compiler::Instruction;

enum_from_primitive! {
#[derive(Debug)]
pub enum Bytecode {
    PUSH = 0,
    POP,
    ADD,
    SUB,
    MULT,
    DIV,
    PRINT,
    TOSTRING
}
}

macro_rules! check_args {
    ($fn_name:expr, $args_len:expr, $expected_args:expr) => {{
        if $expected_args != $args_len {
            let plural = if $expected_args == 1 { false } else { true };
            panic!("Function {} expects exactly {} argument{}, but got {}.",
                   $fn_name, $expected_args, if plural { "s" } else { "" }, $args_len);
        }
        }};
}

fn is_string(string: &str) -> bool {
    let as_bytes = string.as_bytes();
    let quote = '"' as u8;
    as_bytes[0] == quote && as_bytes[as_bytes.len() - 1] == quote
}

pub fn instructions_to_bytecode(instructions: Vec<Instruction>) -> Vec<u8> {
    let mut bytecode = vec![];

    for instruction in instructions {
        match instruction {
            Instruction::Literal(value) => {
                if is_string(&value) {
                    for char in value.chars() {
                        bytecode.push(Bytecode::PUSH as u8);
                        bytecode.push(char as u8);
                    }
                } else if let Ok(parsed_value) = u8::from_str_radix(&value, 10) {
                    bytecode.push(Bytecode::PUSH as u8);
                    bytecode.push(parsed_value);
                } else {
                    println!("Don't know what to do with value '{}'", value);
                }
            }
            Instruction::FunctionCall(fn_name, args) => {
                let args_len = args.len();
                let mut first_arg = "".to_string();
                if let Instruction::Literal(ref string) = args[0] {
                    first_arg = string.to_owned();
                }
                for arg in args {
                    for byte in instructions_to_bytecode(vec![arg]) {
                        bytecode.push(byte);
                    }
                }

                if fn_name == "+".to_string() {
                    check_args!("+", args_len, 2);
                    bytecode.push(Bytecode::ADD as u8);
                } else if fn_name == "-".to_string() {
                    check_args!("-", args_len, 2);
                    bytecode.push(Bytecode::SUB as u8);
                } else if fn_name == "*".to_string() {
                    check_args!("*", args_len, 2);
                    bytecode.push(Bytecode::MULT as u8);
                } else if fn_name == "/".to_string() {
                    check_args!("/", args_len, 2);
                    bytecode.push(Bytecode::DIV as u8);
                } else if fn_name == "print".to_string() {
                    check_args!("print", args_len, 1);
                    if first_arg.len() > 0 && is_string(&first_arg) {
                        bytecode.push(Bytecode::PUSH as u8);
                        bytecode.push(first_arg.len() as u8);
                    } else {
                        bytecode.push(Bytecode::TOSTRING as u8);
                    }
                    bytecode.push(Bytecode::PRINT as u8);
                } else {
                    panic!("No such function: '{}'.", fn_name);
                }
            }
        }
    }

    bytecode
}
