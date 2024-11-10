#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod macros;
mod lexer;
mod parser;
mod generator;
mod test_utils;

use parser::node::Nodes;

use crate::lexer::{ lex as process_lex, token::Token };
use crate::parser::parse as process_parse;
use crate::generator::generate as process_generate;

#[napi]
pub fn lex(input: String) -> Vec<Token> {
  process_lex(&input)
}

#[napi]
pub fn parse(input: String) -> Nodes {
  process_parse(&input)
}

#[napi]
pub fn generate(input: Nodes) -> String {
  process_generate(input)
}
