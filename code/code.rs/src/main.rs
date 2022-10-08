mod token;

use crate::token::{Lexer, Literal, Word};
use std::io::{stdin, Read};

fn main() {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    let parse = Lexer::new(input.into_iter());
    for token in parse {
        println!("{:?}", token);
    }
}

struct Module {
    items: Vec<Item>,
}

enum Item {
    Function(Function),
}

struct Function {
    arguments: Vec<Argument>,
    body: Block,
}

struct Argument {
    name: Word,
    type_: Word,
}

struct Block {
    statements: Vec<Statement>,
}

enum Statement {
    Expression(Expression),
}

enum Expression {
    Dot(Dot),
    Call(Call),
    Block(Block),
    Literal(Literal),
}

struct Dot {
    left: Box<Expression>,
    right: Box<Expression>,
}

struct Call {
    left: Box<Expression>,
    args: Vec<Expression>,
}
