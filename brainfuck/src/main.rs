#![allow(unused_variables)]
use std::fs::read_to_string;
use std::num::Wrapping;
use std::vec::Vec;

enum BrainfuckTokenType {
    ValueIncrement,
    ValueDecrement,
    LoopStart,
    LoopEnd,
    PointerIncrement,
    PointerDecrement,
    PutChar,
    ReadChar,
}

struct BrainfuckToken {
    bftype: BrainfuckTokenType,
    line: usize,
    character: usize,
}

fn lexer(program: &String, instructions: &mut Vec<BrainfuckToken>) {
    let mut line: usize = 1;
    let mut character: usize = 0;
    for chr in program.chars() {
        match chr {
            '+' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::ValueIncrement,
                    line: line,
                    character: character
                });
                character += 1;
            },
            '-' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::ValueDecrement,
                    line: line,
                    character: character
                });
                character += 1;
            }
            '[' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::LoopStart,
                    line: line,
                    character: character
                });
                character += 1;
            },
            ']' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::LoopEnd,
                    line: line,
                    character: character
                });
                character += 1;
            },
            '>' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::PointerIncrement,
                    line: line,
                    character: character
                });
                character += 1;
            },
            '<' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::PointerDecrement,
                    line: line,
                    character: character
                });
                character += 1;
            },
            '.' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::PutChar,
                    line: line,
                    character: character
                });
                character += 1;
            },
            ',' => {
                instructions.push(BrainfuckToken{
                    bftype: BrainfuckTokenType::ReadChar,
                    line: line,
                    character: character
                });
                character += 1;
            },
            '\n'=> {
                line += 1;
                character = 0
            },
            _    => character += 1,
        }
    }
}

fn main() -> std::io::Result<()>{
    // Define a new memory buffer for the brainfuck program.
    // This buffer uses 8bit long unsigned integers that overflow.
    // The buffer is 256 elements long.
    let mut memory: [Wrapping<u8>; 256] = [Wrapping(0u8); 256];
    // The memoryp pointer used by the brainfuck program also wraps arround.
    let mut mem_pointer = Wrapping(0u8);

    let program = read_to_string("program.bf")?;

    let mut tokens = Vec::new();
    lexer(&program, &mut tokens);
    let mut token_pointer = 0;

    while token_pointer < tokens.len() {
        token_pointer += 1;
    }
    Ok(())
}
