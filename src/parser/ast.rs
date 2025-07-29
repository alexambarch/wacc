use crate::parser::{
    ParseError,
    tokenizer::{Token, TokenType},
};
use anyhow::{Result, bail};
use std::{
    fmt::{Display, Formatter},
    vec::IntoIter,
};

type TokenIterator = IntoIter<Token>;

#[derive(Debug)]
enum NodeType {
    Program,
    Function,
    Statement,
    Expression,
    Identifier(String),
    Constant(i32),
}

impl Display for NodeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            NodeType::Program => f.write_str("Program"),
            NodeType::Function => f.write_str("Function"),
            NodeType::Statement => f.write_str("Statement"),
            NodeType::Expression => f.write_str("Expression"),
            NodeType::Identifier(name) => f.write_str(&format!("Identifier({})", name)),
            NodeType::Constant(value) => f.write_str(&format!("Constant({})", value)),
        }
    }
}

/*
 * A node has a type and a number of children.
 * Possible values are encoded in the type
 */
#[derive(Debug)]
pub struct Node {
    ntype: NodeType,
    children: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(format!("{}({:#?})", self.ntype, self.children).as_str())
    }
}

/// A program is a function
pub fn parse_program(tokens: &mut TokenIterator) -> Result<Node> {
    let func = parse_function(tokens)?;

    if let Some(token) = tokens.next() {
        bail!("There's some extra junk here: {}", token.value);
    }

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
                    got: t.ttype
                });
            }

            Ok(Node {
                ntype: NodeType::Identifier(t.value),
                children: vec![],
            })
        }
        None => bail!(ParseError {
            expected: TokenType::Constant,
            got: TokenType::Empty,
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
                    got: t.ttype
                });
            }

            Ok(Node {
                ntype: NodeType::Constant(t.value.parse()?),
                children: vec![],
            })
        }
        None => bail!(ParseError {
            expected: TokenType::Constant,
            got: TokenType::Empty
        }),
    }
}

/// Consume a token and check it matches what we expect
fn expect(ttype: TokenType, tokens: &mut TokenIterator) -> Result<()> {
    match tokens.next() {
        Some(token) => {
            if token.ttype == ttype {
                return Ok(());
            }
            bail!(ParseError {
                expected: ttype,
                got: token.ttype
            })
        }
        None => bail!(ParseError {
            expected: ttype,
            got: TokenType::Empty
        }),
    }
}
