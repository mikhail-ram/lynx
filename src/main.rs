use std::fs;

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
    let contents = fs::read_to_string("../Hello World.lynx")
                   .expect("Something went wrong reading the file");

    compile(contents);
}

fn compile(program: String) {
    let tokens = lexer(program);
    // println!("{:?}", tokens);
}

fn lexer(program: String) {
    for line in program.lines() {
        if line == "" { continue }
        let mut word = String::new();
        let mut string_started = false;
        let mut iterator = line.chars().peekable();
        while let Some(char) = iterator.next() {
        // for char in line.chars() {
            if char.is_alphanumeric() {
                word.push(char);
            }
            else {
                if !word.is_empty() {
                    println!("{:?}", word);
                    word.clear();
                }
                if char == '"' {
                    if string_started {
                        string_started = false;
                    }
                    else {
                        string_started = true;
                    }
                }
                if char == '=' {
                    println!("EQUALS");
                }
                else if char == '/' {
                    let peek = match iterator.peek() {
                        Some(string) => string,
                        None => panic!("Program failed. Can't tell what '/' refers to.")
                    };
                    let peek = *peek;
                    if peek == '/' {
                        break;
                    }
                }
                else if char == '>' {
                    println!("GREATERTHAN");
                }
                else if char == '<' {
                    println!("LESSTHAN");
                }
                else if char == ':' {
                    let peek = *iterator.peek().unwrap();
                    if peek == ':' {
                        println!("BLOCK");
                        break;
                    }
                }
                else if char == ',' {
                    println!("COMMA");
                }
                else if char == '[' {
                    println!("LSQUAREBRACKET");
                }
                else if char == ']' {
                    println!("RSQUAREBRACKET");
                }
                else if char == '(' {
                    println!("LBRACKET");
                }
                else if char == ')' {
                    println!("RBRACKET");
                }
                else if char == '-' {
                    println!("MINUS");
                }
            }
        }
        println!("{:?}", word);
    }
}
