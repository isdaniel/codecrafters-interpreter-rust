pub struct Lexer{}

impl Lexer{
    pub fn lex(input: &str) -> i32 {
        let mut result = 0;
        for (line_number, line) in input.lines().enumerate(){
            for char in line.chars() {
                match char {
                    '(' => println!("LEFT_PAREN ( null"),
                    ')' => println!("RIGHT_PAREN ) null"),
                    '{' => println!("LEFT_BRACE {{ null"),
                    '}' => println!("RIGHT_BRACE }} null"),
                    '*' => println!("STAR * null"),
                    '.' => println!("DOT . null"),
                    '+' => println!("PLUS + null"),
                    ',' => println!("COMMA , null"),
                    c => {
                        println!("[{}]Error: Unexpected character:{}",line_number + 1,c);
                        result = 65;
                    }
                }
            }
        }
        
        println!("EOF  null");
        return result;
    }    
}