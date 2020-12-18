use std::fs;
use std::env;

#[derive(Debug, PartialEq)]
enum Token {
    KEYWORD(String),
    STRING(String),
    NEWLINE
}


fn main() {
    let filename = &env::args().collect::<Vec<String>>()[1];
    let contents = fs::read_to_string(filename)
                   .expect("Something went wrong reading the file");

    compile(contents);
}

fn compile(program: String) {
    let _tokens = lexer(program);
    // println!("{:#?}", tokens);
}

fn lexer(program: String) {
    let keywords = ["show"];
    let mut tokens: Vec<Token> = Vec::new();
    for line in program.lines() {
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
                    panic!("Could not lex file");
                }
            };
        }
        if tokens[tokens.len() - 1] != Token::NEWLINE {
            tokens.push(Token::NEWLINE);
        }
    }
    println!("{:#?}", tokens);
}
