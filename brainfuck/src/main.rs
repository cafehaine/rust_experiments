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
    let mut character: usize = 1;
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
                character = 1
            },
            _    => character += 1,
        }
    }
}

fn execute(tokens: &Vec<BrainfuckToken>) {
    // Define a new memory buffer for the brainfuck program.
    // This buffer uses 8bit long unsigned integers that overflow.
    // The buffer is 256 elements long.
    let mut memory: [Wrapping<u8>; 256] = [Wrapping(0u8); 256];
    // The memoryp pointer used by the brainfuck program also wraps arround.
    let mut mem_pointer = Wrapping(0u8);

    let mut token_pointer = 0;

    while token_pointer < tokens.len() {
        let token = &tokens[token_pointer];
        match token.bftype {
            BrainfuckTokenType::ValueIncrement => {
                memory[mem_pointer.0 as usize] += Wrapping(1);
                token_pointer += 1;
            },
            BrainfuckTokenType::ValueDecrement => {
                memory[mem_pointer.0 as usize] -= Wrapping(1);
                token_pointer += 1;
            },
            BrainfuckTokenType::PointerIncrement => {
                mem_pointer += Wrapping(1);
                token_pointer += 1;
            },
            BrainfuckTokenType::PointerDecrement => {
                mem_pointer -= Wrapping(1);
                token_pointer += 1;
            },
            BrainfuckTokenType::LoopStart => {
                if memory[mem_pointer.0 as usize].0 != 0 {
                    token_pointer += 1;
                } else {
                    let mut loop_end_to_find = 1;
                    let mut new_token_pointer = token_pointer + 1;
                    while token_pointer < tokens.len() && loop_end_to_find != 0 {
                        match tokens[new_token_pointer].bftype {
                            BrainfuckTokenType::LoopStart => loop_end_to_find += 1,
                            BrainfuckTokenType::LoopEnd => loop_end_to_find -= 1,
                            _ => (),
                        }
                        new_token_pointer += 1;
                    }
                    if loop_end_to_find != 0 {
                        panic!("Unexpected EOF while looking for end of loop starting at line {} char {}", token.line, token.character);
                    }
                    token_pointer = new_token_pointer;
                }
            },
            BrainfuckTokenType::LoopEnd => {
                if memory[mem_pointer.0 as usize].0 == 0 {
                    token_pointer += 1;
                } else {
                    let mut loop_start_to_find = 1;
                    let mut new_token_pointer = token_pointer;
                    while token_pointer > 0 && loop_start_to_find != 0 {
                        new_token_pointer -= 1;
                        match tokens[new_token_pointer].bftype {
                            BrainfuckTokenType::LoopStart => loop_start_to_find -= 1,
                            BrainfuckTokenType::LoopEnd => loop_start_to_find += 1,
                            _ => (),
                        }
                    }
                    if loop_start_to_find != 0 {
                        panic!("Unexpected start of file while looking for start of loop ending at line {} char {}", token.line, token.character);
                    }
                    token_pointer = new_token_pointer;
                }
            }
            BrainfuckTokenType::PutChar => {
                print!("{}", memory[mem_pointer.0 as usize].0 as char);
                token_pointer += 1;
            },
            _ => {
                panic!("Unhandled brainfuck token type at line {} char {}", token.line, token.character);
            }
        }
    }
}

fn main() -> std::io::Result<()>{
    let program = read_to_string("program.bf")?;

    let mut tokens = Vec::new();

    lexer(&program, &mut tokens);

    execute(&tokens);

    Ok(())
}
