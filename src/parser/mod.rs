use anyhow::{Result, bail};
use ast::Node;
use log::{debug, info};
use regex::Regex;
use std::collections::HashMap;

mod ast;

#[derive(PartialEq, Eq, Hash, Default, Clone)]
pub struct Token {
    ttype: TokenType,
    value: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Default)]
enum TokenType {
    #[default]
    Empty,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Identifier,
    Constant,
    KWInt,
    KWVoid,
    KWReturn,
    Comment,
}

pub fn parse(contents: &mut String) -> Result<()> {
    let tokens = tokenize(contents)?;
    info!("Successfully parsed {} tokens.", tokens.len());

    Ok(())
}

/// Determine whether a program is valid
fn check(tokens: Vec<Token>) -> Result<bool> {
    todo!();
}

fn tokenize(contents: &mut String) -> Result<Vec<Token>> {
    let mut regexps: HashMap<TokenType, Regex> = HashMap::new();
    regexps.insert(TokenType::OpenParen, Regex::new(r"\(").unwrap());
    regexps.insert(TokenType::CloseParen, Regex::new(r"\)").unwrap());
    regexps.insert(TokenType::OpenBrace, Regex::new(r"\{").unwrap());
    regexps.insert(TokenType::CloseBrace, Regex::new(r"\}").unwrap());
    regexps.insert(TokenType::Semicolon, Regex::new(r";").unwrap());
    regexps.insert(TokenType::KWReturn, Regex::new(r"return\b").unwrap());
    regexps.insert(TokenType::KWVoid, Regex::new(r"void\b").unwrap());
    regexps.insert(TokenType::KWInt, Regex::new(r"int\b").unwrap());
    regexps.insert(TokenType::Constant, Regex::new(r"([0-9]+)\b").unwrap());
    regexps.insert(
        TokenType::Comment,
        Regex::new(r"(?:\/\/.*\n)|(?:\/\*.*\*\/)").unwrap(),
    );
    regexps.insert(
        TokenType::Identifier,
        Regex::new(r"([a-zA-Z_]\w*)\b").unwrap(),
    );

    let mut tokens: Vec<Token> = Vec::new();
    while !contents.is_empty() {
        debug!("remaining: {}", contents);
        *contents = contents.trim_start().to_owned();

        let mut longest = 0;
        let mut longest_token = Token::default();
        for (token, regexp) in &regexps {
            let m = regexp.find(contents);
            if m.is_none() || m.unwrap().start() > 0 {
                continue;
            }

            let fullmatch = m.unwrap();
            let mut match_length = fullmatch.end() - fullmatch.start();

            if match_length > longest {
                match *token {
                    // Identifiers completely cover the KW regexes
                    TokenType::Identifier => {
                        for ttype in [TokenType::KWReturn, TokenType::KWVoid, TokenType::KWInt] {
                            if let Some(kw) = regexps.get(&ttype).unwrap().find(fullmatch.as_str())
                            {
                                match_length = kw.len();
                                longest_token = Token {
                                    ttype,
                                    value: kw.as_str().to_owned(),
                                };
                                break;
                            }
                        }
                    }

                    TokenType::Constant => {
                        let cap = regexp.captures(contents).unwrap().get(0).unwrap();
                        longest_token = Token {
                            ttype: token.clone(),
                            value: cap.as_str().to_owned(),
                        };
                    }

                    _ => {
                        longest_token = Token {
                            ttype: token.clone(),
                            value: fullmatch.as_str().to_owned(),
                        };
                    }
                }

                longest = match_length;
            }
        }

        if longest == 0 {
            bail!("Unable to tokenize :(");
        }

        tokens.push(longest_token.clone());
        *contents = contents[longest..].trim_end().to_owned();
    }

    debug!(
        "Tokens: {:?}",
        tokens.iter().map(|t| t.value.clone()).collect::<String>()
    );

    Ok(tokens)
}
