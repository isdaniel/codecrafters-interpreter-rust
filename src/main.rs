use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use codecrafters_interpreter as imp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let mut any_cc_err = false;
    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents: String = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            
            //let result = Lexer::lex(&file_contents);
            let lex = imp::Lexer::new(&file_contents);
            for token in lex {
                let token = match token{
                    Ok(t) => t, 
                    Err(e) =>{
                        eprintln!("{e:?}");
                        any_cc_err = true;
                        if let Some(unrecognized) = e.downcast_ref::<imp::lex::SingleTokenError>() {
                            eprintln!(
                                "[line {}] Error: Unexpected character: {}",
                                unrecognized.line(),
                                unrecognized.token
                            );
                        } else if let Some(unterminated) = e.downcast_ref::<imp::lex::SingleTerminatedError>(){
                            eprintln!(
                                "[line {}] Error: Unterminated string.",
                                unterminated.line()
                            );
                        }
                        continue;

                    }
                };
                println!("{token}");
            }
            println!("EOF  null");
            if any_cc_err {
                exit(65);
            }
        },
        "parse" =>{
            let file_contents: String = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut parser = imp::Parser::new(&file_contents);
            parser.parse_expression_within();
            
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
