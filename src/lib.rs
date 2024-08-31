pub struct Lexer{}

impl Lexer {
    pub fn lex(input: &str) -> i32 {
        let mut result = 0;
        let mut line_number = 1;
        let mut chars = input.chars().peekable(); // Create a peekable iterator

        while let Some(char) = chars.next() {
            match char {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '-' => println!("MINUS - null"),
                '+' => println!("PLUS + null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                '!' => {
                    if chars.peek() == Some(&'=') {
                        chars.next(); 
                        println!("BANG_EQUAL != null");
                    } else {
                        println!("BANG ! null");
                    }
                },
                '=' => {
                    if chars.peek() == Some(&'=') {
                        chars.next(); 
                        println!("EQUAL_EQUAL == null");
                    } else {
                        println!("EQUAL = null");
                    }
                }
                '\n' => line_number += 1,
                c => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, c);
                    result = 65;
                }
            }
        }

        println!("EOF  null");
        result
    }
}
