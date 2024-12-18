use backyard_lexer::lex;

#[test]
fn basic() {
  let tokens = lex(true, "67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn decimal() {
  let tokens = lex(true, "6.7").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn only_decimal() {
  let tokens = lex(true, ".67").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn hex() {
  let tokens = lex(true, "0xff9abc").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn binary() {
  let tokens = lex(true, "0b1010_1010").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
