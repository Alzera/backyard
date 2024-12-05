use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = match($a) {
  'ucs2', 'utf-16' => 2,
  default => 1
};").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
