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
    // TODO: get start and end indices for number
    // start: usize,
    // end: usize,
}

#[derive(Error, Debug)]
enum LexError {
    #[error("invalid character: {0}")]
    InvalidCharacter(char),
}

impl Token {
    fn lex_from_string(input: &str) -> Result<Vec<Token>, LexError> {
        let mut charstack = Vec::<(usize, char)>::new();
        let mut tokens = Vec::<Token>::new();

        let mut characters = input.chars().enumerate().peekable();
        while let (Some((i, c)), next) = (characters.next(), characters.peek()) {
            match (c, next) {
                (d, n_c) if d.is_ascii_digit() => {
                    charstack.push((i, d));
                    // TODO: refactor around
                    // what happens if next is None or Some(_, !c2.is_ascii_digit())
                    if let Some((_, c2)) = n_c {
                        if !c2.is_ascii_digit() {
                            let mut num_literal = 0;
                            // TODO: get start and end indices for number
                            // TODO: remove clone
                            for digit_character in charstack.clone() {
                                let digit = digit_character.1.to_digit(10).unwrap();
                                num_literal = num_literal * 10 + digit;
                            }
                            tokens.push(Token {
                                tokentype: TokenType::Number(num_literal as usize),
                            });
                            charstack.clear();
                        }
                    } else {
                        let mut num_literal = 0;
                        // TODO: get start and end indices for number
                        // TODO: remove clone
                        for digit_character in charstack.clone() {
                            let digit = digit_character.1.to_digit(10).unwrap();
                            num_literal = num_literal * 10 + digit;
                        }
                        tokens.push(Token {
                            tokentype: TokenType::Number(num_literal as usize),
                        });
                        charstack.clear();
                    }
                }
                ('!', Some((_, '='))) => {
                    characters.next();
                    tokens.push(Token {
                        tokentype: TokenType::NotEquals,
                    });
                }
                ('>', Some((_, '='))) => {
                    characters.next();
                    tokens.push(Token {
                        tokentype: TokenType::GreaterEquals,
                    });
                }
                ('<', Some((_, '='))) => {
                    characters.next();
                    tokens.push(Token {
                        tokentype: TokenType::LessEquals,
                    });
                }
                ('k', Some((_, 'h'))) => {
                    characters.next();
                    tokens.push(Token {
                        tokentype: TokenType::KeepHighest,
                    });
                }
                ('k', Some((_, 'l'))) => {
                    characters.next();
                    tokens.push(Token {
                        tokentype: TokenType::KeepLowest,
                    });
                }
                ('d', _) => tokens.push(Token {
                    tokentype: TokenType::Dice,
                }),
                ('=', _) => tokens.push(Token {
                    tokentype: TokenType::Equals,
                }),
                ('>', _) => tokens.push(Token {
                    tokentype: TokenType::Greater,
                }),
                ('<', _) => tokens.push(Token {
                    tokentype: TokenType::Less,
                }),
                ('+', _) => tokens.push(Token {
                    tokentype: TokenType::Plus,
                }),
                ('-', _) => tokens.push(Token {
                    tokentype: TokenType::Minus,
                }),
                ('*', _) => tokens.push(Token {
                    tokentype: TokenType::Times,
                }),
                ('/', _) => tokens.push(Token {
                    tokentype: TokenType::Divide,
                }),
                ('(', _) => tokens.push(Token {
                    tokentype: TokenType::LeftParen,
                }),
                (')', _) => tokens.push(Token {
                    tokentype: TokenType::RightParen,
                }),
                anything_else => return Err(LexError::InvalidCharacter(anything_else.0)),
            }
        }

        return Ok(tokens);
    }
}

fn main() {
    let dice_expr = "20d6+5>10";
    let tokens = Token::lex_from_string(dice_expr);
    println!("{:#?}", dice_expr);
    println!("{:#?}", tokens);
}
