use std::fs;
use std::env;

enum Token {
    NEWLINE,
    COMMA,
    EQUALS,
    LBRACKET,
    RBRACKET,
    MULTIPLY,
    DIVIDE,
    PLUS,
    MINUS,
    EQUALSTO,
    NOTEQUALSTO,
    LESSTHAN,
    GREATERTHAN,
    LESSTHANEQUALTO,
    GREATERTHANEQUALTO,
    ASK,
    SHOW,
    DO,
    IF,
    WHILE,
    INT,
    FLOAT,
    STRING,
    BOOl,
    IDENT(String),
    BLOCK
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
                   .expect("Something went wrong reading the file");

    compile(contents);
}

fn compile(program: String) {
    let tokens = lexer(program);
    // println!("{:#?}", tokens);
}

fn lexer(program: String) {
    let mut tokens: Vec<Token> = Vec::new();
    for line in program.lines() {
        if line == "" { continue; }
        let mut string_started = false;
        let mut goes_into = false;
        let mut word = String::new();
        let mut gets_from = false;
        let mut iterator = line.chars().peekable();
        while let Some(char) = iterator.next() {
            let peek = *iterator.peek().unwrap_or(&' ');
            {
                if char.is_alphanumeric() {
                    word.push(char);
                }
                else if !word.is_empty() {
                    if word.chars().all(char::is_alphabetic) {
                        println!("IDENT({})", word);
                    }
                    else {
                        println!("NUMBER({})", word);
                    }
                    word.clear();
                }
            }
            if char == '"' {
                let mut string_started = !string_started;
            }
            else if char == '=' {
                println!("EQUALS");
            }
            else if char == '/' && !char.is_alphanumeric() {
                if peek == '/' {
                    break;
                }
                else { println!("DIVIDE") }
            }
            else if char == '>' { println!("GREATERTHAN") }
            else if char == '<' {
                if peek == '(' || peek == '-' {
                    println!("GETSFROM");
                    gets_from = true;
                    iterator.next();
                }
                else { println!("LESSTHAN") }
            }
            else if char == ':' {
                if peek == ':' {
                    println!("BLOCK");
                    break;
                }
            }
            else if char == ',' { println!("COMMA") }
            else if char == '[' { println!("LSQUAREBRACKET") }
            else if char == ']' { println!("RSQUAREBRACKET") }
            else if char == '(' { println!("LBRACKET") }
            else if char == ')' {
                if peek == '>' && goes_into {
                    println!("GOESINTO");
                    goes_into = false;
                    iterator.next();
                }
                else if peek == '-' && gets_from {
                    println!("GETSFROM");
                    gets_from = false;
                    iterator.next();
                }
                else { println!("RBRACKET") }
            }
            else if char == '-' {
                if peek == '>' {
                    println!("GOESINTO");
                    iterator.next();
                }
                else if peek == '(' {
                    iterator.next();
                    let next_peek = *iterator.peek().expect("Program failed. Can't tell what '(' refers to.");
                    if next_peek.is_alphabetic() {
                        println!("GOESINTO");
                        goes_into = true;
                    }
                    else { println!("MINUS") }
                }
                else { println!("MINUS") }
            }
        }
        if word != "" { println!("{}", word) }
        println!("NEWLINE");
    }
}
