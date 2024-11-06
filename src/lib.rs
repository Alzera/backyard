#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod macros;
mod lexer;
mod parser;

use napi::Env;
use parser::node::Nodes;

use crate::lexer::{ lex as process_lex, token::Token };
use crate::parser::parse as process_parse;

#[napi]
pub fn lex(input: String) -> Vec<Token> {
  process_lex(input)
}

#[napi]
pub fn parse(input: String, _: Env) -> Nodes {
  process_parse(input)
}
