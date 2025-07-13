use anyhow::Result;
use clap::Parser;
use log::info;
use parser::tokenize;
use std::fs::File;
use std::io::prelude::*;

mod logger;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Print debug messages
    #[arg(short, long)]
    debug: bool,

    /// Run the lexer, stop before parsing
    #[arg(long)]
    lex: bool,

    /// Run the lexer + parser, stop before assembler
    #[arg(long)]
    parse: bool,

    /// Run the lexer + parser + assembler, stop before codegen
    #[arg(long)]
    codegen: bool,

    /// The source file to compile
    #[arg(value_name = "file")]
    filename: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    logger::init(if args.debug { log::Level::Debug } else { log::Level::Info });

    let _early_exit = args.lex || args.parse || args.codegen;

    let mut contents = read_file(args.filename)?;
    let tokens = tokenize(&mut contents)?;

    info!("Successfully parsed {} tokens.", tokens.len());

    Ok(())
}

fn read_file(filename: String) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
