mod ast;
mod token;

use token::Token;

use crate::token::Lexer;
use std::io::{stdin, Read};

fn main() -> Result<(), ast::Error> {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    let tokens: Vec<Token> = Lexer::new(input.into_iter()).collect();
    let mut tokens_iter = tokens.into_iter().peekable();

    let module = ast::Module::parse(&mut tokens_iter)?;

    match tokens_iter.next() {
        None => {}
        Some(other) => {
            return Err(ast::Error::Unexpected {
                expected: "eof",
                got: other,
            })
        }
    }
    println!("{:?}", module);
    Ok(())
}
