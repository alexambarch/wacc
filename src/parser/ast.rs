use crate::parser::Token;
use anyhow::Result;

#[derive(Debug)]
enum NodeType<'a> {
    Empty,
    Program,
    Function(String, &'a NodeType<'a>),
    Return(&'a NodeType<'a>),
    Expression(&'a NodeType<'a>),
    Constant(i32)
}

/*
 * The Node type should encode multiple possible options
 * Each option is a chain of tokens
 * Each Node
 */
pub struct Node<'a> {
    name: NodeType<'a>,
    value: Option<String>,
    children: Vec<Node<'a>>
}

struct ParseError<'a> {
    line: usize,
    char: usize,
    expected: NodeType<'a>,
    got: NodeType<'a>
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("Error on line {}, char {}: expected {:?}, got {:?}", self.line, self.char, self.expected, self.got))?;
        Ok(())
    }
}

/*
 * 1. Create the type for the Node
 * 2. Build a tree out of the nodes to define the rules
 * 3. Write a function to check that a list of tokens passes the rules defined in the tree
 */
pub fn validate(tokens: Vec<Token>) -> bool {
    true
}

fn parse_constant<'a>(tokens: &'a Vec<Token>) -> Node<'a> {
    Ok(&Node)
}
