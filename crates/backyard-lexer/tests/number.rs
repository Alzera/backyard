use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let tokens = lex_eval("67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn decimal() {
  let tokens = lex_eval("6.7").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn only_decimal() {
  let tokens = lex_eval(".67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn hex() {
  let tokens = lex_eval("0xff9abc").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn binary() {
  let tokens = lex_eval("0b1010_1010").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
