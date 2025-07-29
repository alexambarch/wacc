use anyhow::Result;
use ast::parse_program;
use log::{debug, info};
use tokenizer::Tokenizer;

mod ast;
mod tokenizer;

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
