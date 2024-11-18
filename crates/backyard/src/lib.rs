use backyard_lexer::lex as process_lex;
use backyard_parser::parse as process_parse;
use backyard_generator::generate as process_generate;
use serde::Serialize;
use serde_wasm_bindgen::{ Error, Serializer };
use wasm_bindgen::{ prelude::wasm_bindgen, JsValue };

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
    Err(err) => Err(Error::new(&format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn parse(input: String) -> Result<NodeArray, Error> {
  match process_parse(&input) {
    Ok(nodes) => {
      let serializer = Serializer::new().serialize_maps_as_objects(true);
      nodes.serialize(&serializer).map(|v| v.into())
    }
    Err(err) => Err(Error::new(&format!("{}", err))),
  }
}

#[wasm_bindgen]
pub fn generate(input: NodeArray) -> Result<String, JsValue> {
  let nodes = serde_wasm_bindgen::from_value(input.obj)?;

  let ok = process_generate(nodes);
  Ok(ok)
}
