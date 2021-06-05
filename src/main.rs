use std::fs;
use std::env;
use std::process;
use std::io::{self, BufRead, Write};

#[derive(Debug, PartialEq)]
enum Token {
    KEYWORD(String),
    STRING(String),
    NEWLINE
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let lynx = Lynx(args).run();
    // compile(contents);
}

fn compile(program: String) {
    let _tokens = lexer(program);
    // println!("{:#?}", tokens);
}

struct Lynx (Vec<String>);

impl Lynx {
    fn run(&mut self) {
        if self.0.len() > 1 {
            let contents = fs::read_to_string(&self.0[1])
                           .expect("Something went wrong reading the file");
            let tokens = Lynx::lex(contents);
        }
        else {
            let stdin = io::stdin();
            loop {
                print!(">> ");
                io::stdout().flush().expect("Error flushing stdout");
                let mut input = String::new();
                stdin.lock().read_line(&mut input).expect("Error reading from stdin");
                println!("got: {}", input);
            }
        }
    }

    fn error(line_number: usize, line: &str, error_code: i32) {
        println!("Error: Could not lex file at -> \n\n  |\n{} | {}\n  |\n", line_number, line);
        process::exit(error_code);
    }

    fn lex(program: String) {
        let keywords = ["show"];
        let mut tokens: Vec<Token> = Vec::new();
        for (line_number, line) in program.lines().enumerate() {
            let line_number = line_number + 1;
            let mut keyword_buffer = String::new();
            let mut string_started = false;
            let mut slash = false;
            for letter in line.chars() {
                match letter {
                    '"' => {
                        string_started = !string_started;
                        if !string_started {
                            tokens.push(Token::STRING(keyword_buffer.clone()));
                        }
                        keyword_buffer.clear();
                    },
                    letter if string_started => {
                        keyword_buffer.push(letter);
                    },
                    letter if letter.is_alphabetic() => {
                        keyword_buffer.push(letter);
                    }
                    _ if keywords.contains(&keyword_buffer.as_str()) => {
                        tokens.push(Token::KEYWORD(keyword_buffer.clone()));
                        keyword_buffer.clear();
                    },
                    '/' => {
                        slash = !slash;
                        if !slash { break }
                    },
                    ' ' => {
                        continue;
                    },
                    num if letter.is_numeric() => {
                    },
                    _ => {
                        Lynx::error(line_number, line, 1);
                    }
                };
            }
            /*
            if tokens[tokens.len() - 1] != Token::NEWLINE {
                tokens.push(Token::NEWLINE);
            }
            */
        }
        println!("{:#?}", tokens);
    }
}

fn lexer(program: String) {
    let keywords = ["show"];
    let mut tokens: Vec<Token> = Vec::new();
    for (line_number, line) in program.lines().enumerate() {
        let line_number = line_number + 1;
        let mut keyword_buffer = String::new();
        let mut string_started = false;
        let mut slash = false;
        for letter in line.chars() {
            match letter {
                '"' => {
                    string_started = !string_started;
                    if !string_started {
                        tokens.push(Token::STRING(keyword_buffer.clone()));
                    }
                    keyword_buffer.clear();
                },
                letter if string_started => {
                    keyword_buffer.push(letter);
                },
                letter if letter.is_alphabetic() => {
                    keyword_buffer.push(letter);
                }
                _ if keywords.contains(&keyword_buffer.as_str()) => {
                    tokens.push(Token::KEYWORD(keyword_buffer.clone()));
                    keyword_buffer.clear();
                },
                '/' => {
                    slash = !slash;
                    if !slash { break }
                },
                ' ' => {
                    continue;
                },
                _ => {
                    println!("Error: Could not lex file at -> \n\n  |\n{} | {}\n  |\n", line_number, line);
                    process::exit(1);
                }
            };
        }
        if tokens[tokens.len() - 1] != Token::NEWLINE {
            tokens.push(Token::NEWLINE);
        }
    }
    println!("{:#?}", tokens);
}
