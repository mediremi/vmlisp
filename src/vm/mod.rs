use std::fmt;
use enum_primitive::FromPrimitive;
use compiler::bytecode::Bytecode;

struct Stack {
    max_size: usize,
    vector: Vec<u8>,
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.vector)
    }
}

impl Stack {
    fn new(max_size: u32) -> Stack {
        Stack {
            max_size: max_size as usize,
            vector: vec![],
        }
    }

    fn push(&mut self, value: u8) {
        if self.vector.len() == self.max_size {
            panic!("Stack overflow.");
        }
        self.vector.push(value);
    }

    fn pop(&mut self) -> u8 {
        if self.vector.len() == 0 {
            panic!("Cannot pop when length is 0.");
        }
        self.vector.pop().unwrap()
    }
}

struct VM {
    stack: Stack,
}

impl VM {
    fn new(stack_size: u32) -> VM {
        VM { stack: Stack::new(stack_size) }
    }

    fn byte_to_bytecode(&self, byte: u8) -> Option<Bytecode> {
        Bytecode::from_u8(byte)
    }

    fn execute(&mut self, bytecode: Vec<u8>) {
        let mut skip = false;

        for (i, byte) in bytecode.iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            let op = self.byte_to_bytecode(*byte).unwrap();

            match op {
                Bytecode::PUSH => {
                    self.stack.push(bytecode[i + 1]);
                    skip = true;
                }
                Bytecode::ADD => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a + b);
                }
                Bytecode::SUB => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a - b);
                }
                Bytecode::MULT => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a * b);
                }
                Bytecode::DIV => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a / b);
                }
                Bytecode::PRINT => {
                    let string_length = self.stack.pop();
                    let mut chars = vec![];
                    for _ in 0..string_length {
                        chars.push(self.stack.pop());
                    }
                    chars.reverse();
                    println!("{}", String::from_utf8(chars).unwrap());
                }
                Bytecode::TOSTRING => {
                    let number = self.stack.pop().to_string();
                    for digit in number.chars() {
                        self.stack.push(digit as u8);
                    }
                    self.stack.push(number.len() as u8);
                }
                _ => {}
            }
        }
    }
}

pub fn execute(bytecode: Vec<u8>) {
    VM::new(1024).execute(bytecode);
}
