use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(Word),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Literal(Literal),
    Colon,
    Semicolon,
    Dot,
    Comma,
    Keyword(Keyword),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word(Vec<u8>);

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Func,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(StringLiteral),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral(Vec<u8>);

pub struct Lexer<I> {
    chars: I,
    state: LexState,
}

impl<I> Lexer<I> {
    pub fn new(chars: I) -> Self {
        Self {
            chars,
            state: LexState::Start,
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = u8>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (next_state, token) = self.state.take().consume(self.chars.next()?);
            self.state = next_state;
            if let Some(token) = token {
                return Some(token);
            }
        }
    }
}

pub type Tokens = Peekable<std::vec::IntoIter<Token>>;

#[derive(Debug, PartialEq)]
enum LexState {
    Invalid,
    Start,
    Pending(u8),
    Word(Vec<u8>),
    StringLiteral(Vec<u8>, StringLiteralState),
}

impl LexState {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Self::Invalid)
    }

    fn consume(self, c: u8) -> (Self, Option<Token>) {
        match self {
            Self::Invalid => unreachable!(),
            Self::Start => match c {
                b'_' | b'a'..=b'z' | b'A'..=b'Z' => (Self::Word(vec![c]), None),
                b'"' => (
                    Self::StringLiteral(vec![], StringLiteralState::Normal),
                    None,
                ),
                b'(' => (Self::Start, Some(Token::LeftParen)),
                b')' => (Self::Start, Some(Token::RightParen)),
                b'{' => (Self::Start, Some(Token::LeftBrace)),
                b'}' => (Self::Start, Some(Token::RightBrace)),
                b':' => (Self::Start, Some(Token::Colon)),
                b';' => (Self::Start, Some(Token::Semicolon)),
                b'.' => (Self::Start, Some(Token::Dot)),
                b',' => (Self::Start, Some(Token::Comma)),
                _ => (Self::Start, None),
            },
            Self::Pending(d) => {
                let (pend_state, pend_token) = Self::Start.consume(d);
                if pend_token.is_some() {
                    assert!(pend_state == Self::Start);
                    (Self::Pending(c), pend_token)
                } else {
                    pend_state.consume(c)
                }
            }
            Self::Word(mut word_state) => match c {
                b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                    word_state.push(c);
                    (Self::Word(word_state), None)
                }
                _ => (
                    Self::Pending(c),
                    Some(match word_state.as_slice() {
                        b"func" => Token::Keyword(Keyword::Func),
                        _ => Token::Word(Word(word_state)),
                    }),
                ),
            },
            Self::StringLiteral(mut string, state) => match state {
                StringLiteralState::Normal => match c {
                    b'\\' => (
                        Self::StringLiteral(string, StringLiteralState::Escape),
                        None,
                    ),
                    b'"' => (
                        Self::Start,
                        Some(Token::Literal(Literal::String(StringLiteral(string)))),
                    ),
                    _ => {
                        string.push(c);
                        (Self::StringLiteral(string, state), None)
                    }
                },
                StringLiteralState::Escape => match c {
                    b'n' => {
                        string.push(b'\n');
                        (
                            Self::StringLiteral(string, StringLiteralState::Normal),
                            None,
                        )
                    }
                    _ => {
                        string.push(c);
                        (
                            Self::StringLiteral(string, StringLiteralState::Normal),
                            None,
                        )
                    }
                },
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum StringLiteralState {
    Normal,
    Escape,
}
