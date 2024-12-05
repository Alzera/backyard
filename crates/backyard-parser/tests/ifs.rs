use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("if (false) {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_else() {
  let asts = parse_eval("if (false) {} else {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_elseif() {
  let asts = parse_eval("if (false) {} elseif (true) {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn with_else_if() {
  let asts = parse_eval("if (false) {} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn nested() {
  let asts = parse_eval("if (false) {if (false) {} else (true) {}} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn no_block() {
  let asts = parse_eval("if (false) $e = 5; else if (true) $e = 6;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short() {
  let asts = parse_eval("if (true): elseif (false): else: endif;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
