use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("do {\n} while (false);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn separated_by_comment() {
  let asts = parse_eval("do {\n}\n// this comment\nwhile (false);").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
