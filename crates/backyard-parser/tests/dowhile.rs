use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "do {\n} while (false);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn separated_by_comment() {
  let asts = parse(true, "do {\n}\n// this comment\nwhile (false);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
