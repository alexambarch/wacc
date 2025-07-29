use crate::parser::tokenizer::{Token, TokenType};
use anyhow::{Result, bail};
use std::vec::IntoIter;

type TokenIterator = IntoIter<Token>;

#[derive(Debug, Default)]
enum NodeType {
    #[default]
    Empty,
    Program,
    Function,
    Statement,
    Expression,
    Identifier(String),
    Constant(i32),
}

/*
 * The Node type should encode multiple possible options
 * Each option is a chain of tokens
 * Each Node
 */
#[derive(Default)]
pub struct Node {
    ntype: NodeType,
    children: Vec<Node>,
}

#[derive(Debug)]
struct ParseError {
    expected: TokenType,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("Parsing error: expected {:?}", self.expected))?;
        Ok(())
    }
}

/// A program is a function
pub fn parse_program(tokens: &mut TokenIterator) -> Result<Node> {
    let func = parse_function(tokens)?;
    Ok(Node {
        ntype: NodeType::Program,
        children: vec![func],
    })
}

/// A function is "int" + identifier + "(" + "void" + ")" + "{" + statement + "}"
fn parse_function(tokens: &mut TokenIterator) -> Result<Node> {
    expect(TokenType::KWInt, tokens)?;
    let name = parse_identifier(tokens)?;
    expect(TokenType::OpenParen, tokens)?;
    expect(TokenType::KWVoid, tokens)?;
    expect(TokenType::CloseParen, tokens)?;

    expect(TokenType::OpenBrace, tokens)?;
    let statement = parse_statement(tokens)?;
    expect(TokenType::CloseBrace, tokens)?;

    Ok(Node {
        ntype: NodeType::Function,
        children: vec![name, statement],
    })
}

/// A statement is "return" + expression + ";"
fn parse_statement(tokens: &mut TokenIterator) -> Result<Node> {
    expect(TokenType::KWReturn, tokens)?;
    let exp = parse_expression(tokens)?;
    expect(TokenType::Semicolon, tokens)?;

    return Ok(Node {
        ntype: NodeType::Statement,
        children: vec![exp],
    });
}

/// An expression is currently only a constant
fn parse_expression(tokens: &mut TokenIterator) -> Result<Node> {
    let constant = parse_constant(tokens)?;
    Ok(Node {
        ntype: NodeType::Expression,
        children: vec![constant],
    })
}

/// An identifier is a string of characters
fn parse_identifier(tokens: &mut TokenIterator) -> Result<Node> {
    match tokens.next() {
        Some(t) => {
            if t.ttype != TokenType::Identifier {
                bail!(ParseError {
                    expected: TokenType::Identifier,
                });
            }

            Ok(Node {
                ntype: NodeType::Identifier(t.value),
                children: vec![],
            })
        }
        None => bail!(ParseError {
            expected: TokenType::Constant,
        }),
    }
}

/// A constant is a number
fn parse_constant(tokens: &mut TokenIterator) -> Result<Node> {
    match tokens.next() {
        Some(t) => {
            if t.ttype != TokenType::Constant {
                bail!(ParseError {
                    expected: TokenType::Constant,
                });
            }

            Ok(Node {
                ntype: NodeType::Constant(t.value.parse()?),
                children: vec![],
            })
        }
        None => bail!(ParseError {
            expected: TokenType::Constant,
        }),
    }
}

/// Consume a token and check it matches what we expect
fn expect(ttype: TokenType, tokens: &mut TokenIterator) -> Result<bool> {
    match tokens.next() {
        Some(token) => Ok(token.ttype == ttype),
        None => bail!(ParseError { expected: ttype }),
    }
}
