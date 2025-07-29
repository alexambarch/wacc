use anyhow::{Result, bail};
use log::debug;
use regex::Regex;
use std::collections::HashMap;

/// All possible token types
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum TokenType {
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
    Empty,
}

/// A constant array of every token type and the regex to parse it
const TOKENS: [(TokenType, &str); 11] = [
    (TokenType::OpenParen, r"\("),
    (TokenType::CloseParen, r"\)"),
    (TokenType::OpenBrace, r"\{"),
    (TokenType::CloseBrace, r"\}"),
    (TokenType::Semicolon, r";"),
    (TokenType::Identifier, r"([a-zA-Z_]\w*)\b"),
    (TokenType::Constant, r"([0-9]+)\b"),
    (TokenType::KWInt, r"int\b"),
    (TokenType::KWReturn, r"return\b"),
    (TokenType::KWVoid, r"void\b"),
    (TokenType::Comment, r"(?:\/\/.*\n)|(?:\/\*.*\*\/)"),
];

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub value: String,
}

pub struct Tokenizer {
    exps: HashMap<TokenType, Regex>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        let mut regexps: HashMap<TokenType, Regex> = HashMap::new();
        for (ttype, exp) in TOKENS {
            regexps.insert(ttype, Regex::new(exp).unwrap());
        }

        Tokenizer { exps: regexps }
    }

    pub fn tokenize(&self, contents: &mut String) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        while !contents.is_empty() {
            debug!("remaining: {}", contents);
            *contents = contents.trim_start().to_owned();

            let mut longest = 0;
            let mut longest_token: Option<Token> = None;
            for (token, regexp) in &self.exps {
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
                            let mut kw_covered = false;
                            let cap = regexp.captures(contents).unwrap().get(0).unwrap();

                            for ttype in [TokenType::KWReturn, TokenType::KWVoid, TokenType::KWInt]
                            {
                                if let Some(kw) =
                                    self.exps.get(&ttype).unwrap().find(fullmatch.as_str())
                                {
                                    kw_covered = true;
                                    match_length = kw.len();
                                    longest_token = Some(Token {
                                        ttype,
                                        value: kw.as_str().to_owned(),
                                    });
                                    break;
                                }
                            }

                            if !kw_covered {
                                longest_token = Some(Token {
                                    ttype: token.clone(),
                                    value: cap.as_str().to_owned(),
                                });
                            }
                        }

                        TokenType::Constant => {
                            let cap = regexp.captures(contents).unwrap().get(0).unwrap();
                            longest_token = Some(Token {
                                ttype: token.clone(),
                                value: cap.as_str().to_owned(),
                            });
                        }

                        _ => {
                            longest_token = Some(Token {
                                ttype: token.clone(),
                                value: fullmatch.as_str().to_owned(),
                            });
                        }
                    }

                    longest = match_length;
                }
            }

            if longest == 0 || longest_token.is_none() {
                bail!("Unable to tokenize :(");
            }

            tokens.push(longest_token.unwrap().clone());
            *contents = contents[longest..].trim_end().to_owned();
        }

        debug!(
            "Tokens: {}",
            tokens
                .iter()
                .map(|t| t.value.clone() + ", ")
                .collect::<String>()
        );

        Ok(tokens)
    }
}
