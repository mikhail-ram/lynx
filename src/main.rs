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
        let mut goes_into = false;
        let mut gets_from = false;
        let mut iterator = line.chars().peekable();
        while let Some(char) = iterator.next() {
        // for char in line.chars() {
            if char.is_alphanumeric() {
                word.push(char);
            }
            else if !word.is_empty() {
                println!("{:?}", word);
                word.clear();
            }
            if char == '"' {
                let mut string_started = !string_started;
            }
            if char == '=' {
                println!("EQUALS");
            }
            else if char == '/' && !char.is_alphanumeric() {
                let peek = *iterator.peek().expect("Program failed. Can't tell what '/' refers to.");
                if peek == '/' {
                    break;
                }
                else {
                    println!("DIVIDE");
                }
            }
            else if char == '>' {
                println!("GREATERTHAN");
            }
            else if char == '<' {
                let peek = *iterator.peek().expect("Program failed. Can't tell what '<' refers to.");
                if peek == '(' || peek == '-' {
                    println!("GETSFROM");
                    gets_from = true;
                    iterator.next();
                }
                else {
                    println!("LESSTHAN");
                }
            }
            else if char == ':' {
                let peek = *iterator.peek().expect("Program failed. Can't tell what ':' refers to.");
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
                let peek = *iterator.peek().unwrap_or(&' ');
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
                else {
                    println!("RBRACKET");
                }
            }
            else if char == '-' {
                let peek = *iterator.peek().expect("Program failed. Can't tell what '-' refers to.");
                if peek == '>' {
                    println!("GOESINTO");
                    iterator.next();
                }
                else if peek == '(' {
                    iterator.next();
                    let peek = *iterator.peek().expect("Program failed. Can't tell what '(' refers to.");
                    if peek.is_alphabetic() {
                        println!("GOESINTO");
                        goes_into = true;
                    }
                    else {
                        println!("MINUS");
                    }
                }
                else {
                    println!("MINUS");
                }
            }
        }
        println!("{:?}", word);
    }
}
