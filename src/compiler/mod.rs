mod tokeniser;
use self::tokeniser::{Token, tokenise};
pub mod bytecode;
use self::bytecode::{instructions_to_bytecode};

#[derive(Debug)]
pub enum Instruction {
    FunctionCall(String, Vec<Instruction>),
    Literal(String),
}

fn tokens_to_instructions(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut instructions = vec![];

    for token in tokens {
        match token {
            Token::Atom(value) => instructions.push(Instruction::Literal(value)),
            Token::List(tokens) => {
                if tokens.len() > 0 {
                    let (head, tail) = tokens.split_at(1);
                    let fn_name = match head.first().unwrap().to_owned() {
                        Token::Atom(value) => value,
                        _ => panic!("Illegal function call."),
                    };
                    instructions.push(Instruction::FunctionCall(fn_name,
                                                        tokens_to_instructions(tail.to_vec())));
                }
            }
        };
    }

    instructions
}

pub fn compile(program: String) -> Vec<u8> {
    let tokens = tokenise(program);
    let instructions = tokens_to_instructions(tokens);
    instructions_to_bytecode(instructions)
}
