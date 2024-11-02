#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod macros;
mod lexer;
mod parser;

use napi::Env;
use napi::JsObject;

use crate::lexer::{ lex as process_lex, token::Token };
use crate::parser::parse as process_parse;

#[napi]
pub fn lex(input: String) -> Vec<Token> {
  process_lex(input)
}

#[napi]
pub fn parse(input: String, env: Env) -> Vec<JsObject> {
  process_parse(input)
    .iter()
    .map(|x| x.to_object(env))
    .collect()
}
