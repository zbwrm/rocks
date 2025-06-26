use std::ops::Deref;
use std::{convert::Infallible, iter::Peekable, mem, ops::Index};

use thiserror::Error;

#[derive(Debug, Clone)]
enum DiceFilterType {
    KeepHighest,
    KeepLowest,
    DropHighest,
    DropLowest,
}

#[derive(Debug, Clone)]
enum AdditionOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone)]
enum MathOperator {
    Add(AdditionOperator),
    Times,
}

#[derive(Debug, Clone, Copy)]
enum ComparisonOperator {
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
}

#[derive(Debug, Clone)]
enum TokenType {
    Number(usize),
    Dice,
    Df(DiceFilterType),
    Op(MathOperator),
    LeftParen,
    RightParen,
    Cmp(ComparisonOperator),
}

#[derive(Debug)]
struct Token {
    tokentype: TokenType,
    start: usize,
    end: usize,
}

impl Token {
    fn is_comparison_operator(&self) -> bool {
        match self.tokentype {
            TokenType::Cmp(_) => true,
            _ => false,
        }
    }
}

#[derive(Error, Debug)]
enum LexError {
    #[error("invalid character: '{character}' at position {loc}")]
    InvalidCharacter { character: char, loc: usize },
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

impl Index<usize> for LexedExpression {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
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
                        tokentype: TokenType::Cmp(ComparisonOperator::NotEquals),
                        start: i,
                        end: i2.clone(),
                    });
                    characters.next();
                }
                ('>', Some((i2, '='))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Cmp(ComparisonOperator::GreaterEquals),
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('<', Some((i2, '='))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Cmp(ComparisonOperator::LessEquals),
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('k', Some((i2, 'h'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Df(DiceFilterType::KeepHighest),
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('k', Some((i2, 'l'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Df(DiceFilterType::KeepLowest),
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('d', Some((i2, 'h'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Df(DiceFilterType::DropHighest),
                        start: i,
                        end: *i2,
                    });
                    characters.next();
                }
                ('d', Some((i2, 'l'))) => {
                    tokens.push(Token {
                        tokentype: TokenType::Df(DiceFilterType::DropLowest),
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
                    tokentype: TokenType::Cmp(ComparisonOperator::Equals),
                    start: i,
                    end: i,
                }),
                ('>', _) => tokens.push(Token {
                    tokentype: TokenType::Cmp(ComparisonOperator::Greater),
                    start: i,
                    end: i,
                }),
                ('<', _) => tokens.push(Token {
                    tokentype: TokenType::Cmp(ComparisonOperator::Less),
                    start: i,
                    end: i,
                }),
                ('+', _) => tokens.push(Token {
                    tokentype: TokenType::Op(MathOperator::Add(AdditionOperator::Plus)),
                    start: i,
                    end: i,
                }),
                ('-', _) => tokens.push(Token {
                    tokentype: TokenType::Op(MathOperator::Add(AdditionOperator::Minus)),
                    start: i,
                    end: i,
                }),
                ('*', _) => tokens.push(Token {
                    tokentype: TokenType::Op(MathOperator::Times),
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
                anything_else => {
                    return Err(LexError::InvalidCharacter {
                        character: anything_else.0,
                        loc: i,
                    })
                }
            }
        }

        return Ok(LexedExpression(tokens));
    }
}

#[derive(Error, Debug)]
enum ParseError {
    #[error("second comparison operator: {location:?}")]
    ExtraComparisonOperator { location: usize },
    #[error("Too many math operators: {location:?}")]
    ConsecutiveAddOperators { location: usize },
}

struct BinOp {
    lhs: ValueExpr,
    rhs: ValueExpr,
}

struct MathExpression {
    operator: MathOperator,
    sides: BinOp,
}

struct DiceFilter {
    df_type: DiceFilterType,
    num: usize,
}

struct DiceExpression {
    dice: usize,
    sides: usize,
    filter: Option<DiceFilter>,
}

enum ValueExpr {
    Math(Box<MathExpression>),
    Paren(Box<ValueExpr>),
    Dice(Box<DiceExpression>),
    Lit(usize),
}

impl TryFrom<&[Token]> for ValueExpr {
    type Error = ParseError;

    fn try_from(expr_slice: &[Token]) -> Result<Self, Self::Error> {
        todo!();
        for tok in expr_slice {
            println!("{:#?}", tok);
        }
        return Err(ParseError::ConsecutiveAddOperators { location: 0 });
        // return Ok(ValueExpr::Lit(0));
    }
}

struct ComparisonExpr {
    operator: ComparisonOperator,
    sides: BinOp,
}

enum ParsedExpr {
    Cmp(ComparisonExpr),
    Val(ValueExpr),
}

impl TryFrom<LexedExpression> for ParsedExpr {
    type Error = ParseError;

    fn try_from(expr: LexedExpression) -> Result<ParsedExpr, ParseError> {
        let parsed_expression: ParsedExpr;
        // first layer: comparison expression
        // get index of first comparison operator, error otherwise
        let mut is_comparison_expression: bool = false;
        let mut comparison_token_index: Option<usize> = None;
        for (idx, tok) in expr.0.iter().enumerate() {
            match (tok.is_comparison_operator(), is_comparison_expression) {
                (true, false) => {
                    is_comparison_expression = true;
                    comparison_token_index = Some(idx);
                }
                (true, true) => {
                    return Err(ParseError::ExtraComparisonOperator {
                        location: tok.start,
                    });
                }
                (false, _) => {}
            }
        }
        match (is_comparison_expression, comparison_token_index) {
            (true, Some(cmp_idx)) => {
                let rhs = ValueExpr::try_from(&expr.0[cmp_idx + 1..])?;
                let lhs = ValueExpr::try_from(&expr.0[0..cmp_idx])?;

                let comparison = ComparisonExpr {
                    operator: if let TokenType::Cmp(op) = expr[cmp_idx].tokentype {
                        op
                    } else {
                        // occurs if comparison_token_index is not actually a comparison token
                        unreachable!();
                    },
                    sides: BinOp { lhs, rhs },
                };

                parsed_expression = ParsedExpr::Cmp(comparison);
            }
            (true, None) => {
                // occurs when expression is determined to be a comparison expression
                // but no index is given; should never happen as both are defined
                // at the same time
                unreachable!()
            }
            (false, _) => {
                let value_expr = ValueExpr::try_from(expr.0.deref())?;
                parsed_expression = ParsedExpr::Val(value_expr);
            }
        }

        Ok(parsed_expression)
    }
}

fn main() {
    let dice_expr = "20d6kl2+50=20";
    println!("{:#?}", dice_expr);
    let tokens = LexedExpression::try_from(dice_expr).unwrap();
    println!("{:#?}", tokens);
    let pexpr = ParsedExpr::try_from(tokens);
}
