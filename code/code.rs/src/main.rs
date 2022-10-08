mod token;

use token::{Keyword, Token};

use crate::token::{Lexer, Literal, Word};
use std::{
    io::{stdin, Read},
    iter::Peekable,
};

fn main() -> Result<(), Error> {
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    let tokens: Vec<Token> = Lexer::new(input.into_iter()).collect();
    let mut tokens_iter = tokens.into_iter().peekable();

    let module = Module::parse(&mut tokens_iter)?;

    match tokens_iter.next() {
        None => {}
        Some(other) => {
            return Err(Error::Unexpected {
                expected: "eof",
                got: other,
            })
        }
    }
    println!("{:#?}", module);
    Ok(())
}

type Tokens = Peekable<std::vec::IntoIter<Token>>;

#[derive(Debug)]
enum Error {
    Eof,
    Unexpected { expected: &'static str, got: Token },
}

#[derive(Debug)]
struct Module {
    items: Vec<Item>,
}

impl Module {
    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        let mut items = Vec::new();
        loop {
            match tokens.peek() {
                Some(Token::RightBrace) | None => break,
                Some(_) => {}
            }
            items.push(Item::parse(tokens)?);
        }
        Ok(Self { items })
    }
}

#[derive(Debug)]
enum Item {
    Function(Function),
}

impl Item {
    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        match tokens.peek() {
            Some(Token::Keyword(Keyword::Func)) => Function::parse(tokens).map(Self::Function),
            None => Err(Error::Eof),
            Some(other) => Err(Error::Unexpected {
                expected: "`func`",
                got: other.clone(),
            }),
        }
    }
}

#[derive(Debug)]
struct Function {
    name: Word,
    args: Vec<Argument>,
    body: Block,
}

impl Function {
    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        match tokens.next() {
            Some(Token::Keyword(Keyword::Func)) => {}
            None => return Err(Error::Eof),
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "`func`",
                    got: other,
                });
            }
        }

        let name = match tokens.next() {
            Some(Token::Word(word)) => word,
            None => return Err(Error::Eof),
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "word",
                    got: other,
                });
            }
        };
        match tokens.next() {
            Some(Token::LeftParen) => {}
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "opening parentheses `(`",
                    got: other,
                });
            }
            None => return Err(Error::Eof),
        }
        let mut args = Vec::new();
        loop {
            match tokens.peek() {
                Some(Token::RightParen) => {
                    tokens.next();
                    break;
                }
                None => return Err(Error::Eof),
                Some(_) => {}
            }
            args.push(Argument::parse(tokens)?);
            match tokens.next() {
                Some(Token::Comma) => {}
                Some(Token::RightParen) => {
                    break;
                }
                None => return Err(Error::Eof),
                Some(other) => {
                    return Err(Error::Unexpected {
                        expected: "`,` or `)`",
                        got: other,
                    });
                }
            }
        }
        match tokens.next() {
            Some(Token::LeftBrace) => {}
            None => return Err(Error::Eof),
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "`{`",
                    got: other,
                })
            }
        }
        let body = Block::parse_body(tokens)?;
        match tokens.next() {
            Some(Token::RightBrace) => {}
            None => return Err(Error::Eof),
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "`}`",
                    got: other,
                })
            }
        }
        Ok(Function { name, args, body })
    }
}

#[derive(Debug)]
struct Argument {
    name: Word,
    type_: Word,
}

impl Argument {
    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        let name = match tokens.next() {
            Some(Token::Word(word)) => word,
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "word",
                    got: other,
                })
            }
            None => return Err(Error::Eof),
        };
        match tokens.next() {
            Some(Token::Colon) => {}
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "`:`",
                    got: other,
                })
            }
            None => return Err(Error::Eof),
        }
        let type_ = match tokens.next() {
            Some(Token::Word(word)) => word,
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "word",
                    got: other,
                })
            }
            None => return Err(Error::Eof),
        };
        Ok(Argument { name, type_ })
    }
}

#[derive(Debug)]
struct Block {
    statements: Vec<Statement>,
}

impl Block {
    fn parse_body(tokens: &mut Tokens) -> Result<Self, Error> {
        let mut statements = Vec::new();
        loop {
            match tokens.peek() {
                Some(Token::RightBrace) => return Ok(Self { statements }),
                Some(_) => {
                    statements.push(Statement::parse(tokens)?);
                }
                None => return Err(Error::Eof),
            }
        }
    }
}

#[derive(Debug)]
enum Statement {
    Expression(Expression),
}

impl Statement {
    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        match tokens.peek() {
            _ => {
                let expression = Expression::parse(tokens)?;
                match tokens.next() {
                    Some(Token::Semicolon) => Ok(Self::Expression(expression)),
                    Some(other) => Err(Error::Unexpected {
                        expected: "semicolon `;`",
                        got: other,
                    }),
                    None => Err(Error::Eof),
                }
            }
        }
    }
}

#[derive(Debug)]
enum Expression {
    Dot(Dot),
    Call(Call),
    Block(Block),
    Literal(Literal),
    Word(Word),
}

impl Expression {
    fn parse_base(tokens: &mut Tokens) -> Result<Self, Error> {
        match tokens.next() {
            Some(Token::Word(word)) => Ok(Expression::Word(word)),
            Some(Token::Literal(literal)) => Ok(Expression::Literal(literal)),
            Some(Token::LeftBrace) => {
                let block = Block::parse_body(tokens)?;
                match tokens.next() {
                    Some(Token::RightBrace) => {}
                    Some(other) => {
                        return Err(Error::Unexpected {
                            expected: "closing brace `}`",
                            got: other,
                        })
                    }
                    None => return Err(Error::Eof),
                }
                Ok(Expression::Block(block))
            }
            Some(other) => {
                return Err(Error::Unexpected {
                    expected: "word, literal, or `{`",
                    got: other.clone(),
                })
            }
            None => Err(Error::Eof),
        }
    }

    fn parse(tokens: &mut Tokens) -> Result<Self, Error> {
        let mut acc = Self::parse_base(tokens)?;
        loop {
            match tokens.peek() {
                Some(Token::Dot) => {
                    tokens.next();

                    let rhs = Expression::parse_base(tokens)?;
                    acc = Expression::Dot(Dot {
                        left: Box::new(acc),
                        right: Box::new(rhs),
                    });
                }
                Some(Token::LeftParen) => {
                    tokens.next();

                    let mut args = Vec::new();
                    loop {
                        match tokens.peek() {
                            Some(Token::RightParen) => {
                                tokens.next();
                                break;
                            }
                            None => return Err(Error::Eof),
                            Some(_) => {}
                        }
                        args.push(Expression::parse(tokens)?);
                        match tokens.next() {
                            Some(Token::Comma) => {}
                            Some(Token::RightParen) => {
                                break;
                            }
                            None => return Err(Error::Eof),
                            Some(other) => {
                                return Err(Error::Unexpected {
                                    expected: "`,` or `)`",
                                    got: other,
                                })
                            }
                        }
                    }
                    acc = Expression::Call(Call {
                        left: Box::new(acc),
                        args,
                    });
                }
                _ => return Ok(acc),
            }
        }
    }
}

#[derive(Debug)]
struct Dot {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
struct Call {
    left: Box<Expression>,
    args: Vec<Expression>,
}
