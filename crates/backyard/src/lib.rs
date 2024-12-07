use backyard_lexer::{ lex as process_lex, lex_eval as process_lex_eval };
use backyard_nodes::node::Node;
use backyard_parser::{ parse as process_parse, parse_eval as process_parse_eval };
use backyard_generator::generate as process_generate;
use serde::Serialize;
use serde_wasm_bindgen::{ Error, Serializer };
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Array<Token>")]
  pub type TokenArray;

  #[wasm_bindgen(typescript_type = "Array<Node>")]
  pub type NodeArray;

  // #[wasm_bindgen(js_namespace = console)]
  // fn log(s: &str);
}

#[wasm_bindgen]
pub fn lex(input: String) -> Result<TokenArray, Error> {
  match process_lex(&input) {
    Ok(tokens) => serde_wasm_bindgen::to_value(&tokens).map(|v| v.into()),
    Err(err) => Err(Error::new(format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn lex_eval(input: String) -> Result<TokenArray, Error> {
  match process_lex_eval(&input) {
    Ok(tokens) => serde_wasm_bindgen::to_value(&tokens).map(|v| v.into()),
    Err(err) => Err(Error::new(format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn parse(input: String) -> Result<NodeArray, Error> {
  match process_parse(&input) {
    Ok(nodes) => {
      let serializer = Serializer::new().serialize_maps_as_objects(true);
      nodes.serialize(&serializer).map(|v| v.into())
    }
    Err(err) => Err(Error::new(format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn parse_eval(input: String) -> Result<NodeArray, Error> {
  match process_parse_eval(&input) {
    Ok(nodes) => {
      let serializer = Serializer::new().serialize_maps_as_objects(true);
      nodes.serialize(&serializer).map(|v| v.into())
    }
    Err(err) => Err(Error::new(format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn generate(input: NodeArray) -> Result<String, Error> {
  let nodes: Box<Node> = serde_wasm_bindgen::from_value(input.obj)?;

  match process_generate(&nodes) {
    Ok(nodes) => { Ok(nodes) }
    Err(err) => Err(Error::new(format!("{}", err))),
  }
}
