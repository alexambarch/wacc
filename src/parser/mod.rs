use anyhow::Result;
use ast::parse_program;
use log::{debug, info};
use std::fmt::{Display, Formatter};
use tokenizer::{TokenType, Tokenizer};

pub mod ast;
mod tokenizer;

#[derive(Debug)]
struct ParseError {
    expected: TokenType,
    got: TokenType,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!(
            "Parsing error: expected {:?}, got {:?}",
            self.expected, self.got
        ))?;
        Ok(())
    }
}

pub fn generate_ast(contents: &mut String, lex_only: bool) -> Result<()> {
    let tokenizer = Tokenizer::new();
    let tokens = tokenizer.tokenize(contents)?;
    info!("Successfully parsed {} tokens.", tokens.len());

    if !lex_only {
        let ast = parse_program(&mut tokens.into_iter())?;
        info!("Successfully parsed tokens into AST!");
        debug!("AST: {:?}", ast);
    }

    Ok(())
}
