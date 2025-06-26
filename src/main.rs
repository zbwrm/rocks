use std::iter::Peekable;

use thiserror::Error;

#[derive(Debug, Clone)]
enum TokenType {
    // number literal
    Number(usize),

    // dice expressions
    Dice,
    KeepHighest,
    KeepLowest,
    DropHighest,
    DropLowest,

    // math expressions
    Plus,
    Minus,
    Times,
    Divide,
    LeftParen,
    RightParen,

    // equality expression
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(Debug)]
struct Token {
    tokentype: TokenType,
    start: usize,
    end: usize,
}

#[derive(Error, Debug)]
enum LexError {
    #[error("invalid character: {0} at char #{1}")]
    InvalidCharacter(char, usize),
}

#[derive(Debug)]
struct LexedExpression(Vec<Token>);

impl LexedExpression {
    fn consume_digitstack(digitstack: &mut Vec<(usize, u8)>) -> Option<Token> {
        let mut num_literal = 0;
        for digit_character in digitstack.iter() {
            let digit = digit_character.1;
            num_literal = num_literal * 10 + digit;
        }
        let start = digitstack[0].0;
        let end: usize;
        if let Some(last) = digitstack.last() {
            end = last.0;
        } else {
            return None; // input is empty
        }
        digitstack.clear();
        Some(Token {
            tokentype: TokenType::Number(num_literal as usize),
            start,
            end,
        })
    }
}

impl TryFrom<&str> for LexedExpression {
    type Error = LexError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut digitstack = Vec::<(usize, u8)>::new();
        let mut tokens = Vec::<Token>::new();

        let mut characters = value.chars().enumerate().peekable();
        while let (Some((i, c)), next) = (characters.next(), characters.peek()) {
            match (c, next) {
                (d, Some((_, d2))) if d.is_ascii_digit() && d2.is_ascii_digit() => {
                    digitstack.push((i, d.to_digit(10).unwrap().try_into().unwrap()));
                    // guaranteed to be digit < 255
                }
                (d, Some((_, c2))) if d.is_ascii_digit() && !c2.is_ascii_digit() => {
                    digitstack.push((i, d.to_digit(10).unwrap().try_into().unwrap())); // guaranteed to be digit < 255
                    if let Some(tok) = LexedExpression::consume_digitstack(&mut digitstack) {
                        tokens.push(tok);
                    }
                }
                (d, None) if d.is_ascii_digit() => {
                    digitstack.push((i, d.to_digit(10).unwrap().try_into().unwrap())); // guaranteed to be digit < 255
                    if let Some(tok) = LexedExpression::consume_digitstack(&mut digitstack) {
                        tokens.push(tok);
                    }
                }
                ('!', Some((i2, '='))) => {
                    tokens.push(Token {
                        tokentype: TokenType::NotEquals,
                        start: i,
                        end: i2.clone(),
                    });
                    characters.next();
                }
                ('>', Some((i2, '='))) => {
                    tokens.push(Token {
                        tokentype: TokenType::GreaterEquals,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('<', Some((i2, '='))) => {
                    tokens.push(Token {
                        tokentype: TokenType::LessEquals,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('k', Some((i2, 'h'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::KeepHighest,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('k', Some((i2, 'l'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::KeepLowest,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('d', Some((i2, 'h'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::DropHighest,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('d', Some((i2, 'l'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::DropLowest,
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('d', _) => tokens.push(Token {
                    tokentype: TokenType::Dice,
                    start: i,
                    end: i,
                }),
                ('=', _) => tokens.push(Token {
                    tokentype: TokenType::Equals,
                    start: i,
                    end: i,
                }),
                ('>', _) => tokens.push(Token {
                    tokentype: TokenType::Greater,
                    start: i,
                    end: i,
                }),
                ('<', _) => tokens.push(Token {
                    tokentype: TokenType::Less,
                    start: i,
                    end: i,
                }),
                ('+', _) => tokens.push(Token {
                    tokentype: TokenType::Plus,
                    start: i,
                    end: i,
                }),
                ('-', _) => tokens.push(Token {
                    tokentype: TokenType::Minus,
                    start: i,
                    end: i,
                }),
                ('*', _) => tokens.push(Token {
                    tokentype: TokenType::Times,
                    start: i,
                    end: i,
                }),
                ('/', _) => tokens.push(Token {
                    tokentype: TokenType::Divide,
                    start: i,
                    end: i,
                }),
                ('(', _) => tokens.push(Token {
                    tokentype: TokenType::LeftParen,
                    start: i,
                    end: i,
                }),
                (')', _) => tokens.push(Token {
                    tokentype: TokenType::RightParen,
                    start: i,
                    end: i,
                }),
                anything_else => return Err(LexError::InvalidCharacter(anything_else.0, i)),
            }
        }

        return Ok(LexedExpression(tokens));
    }
}

fn main() {
    let dice_expr = "20d6kl2+5>=10";
    let tokens = LexedExpression::try_from(dice_expr);
    println!("{:#?}", dice_expr);
    println!("{:#?}", tokens);
}
