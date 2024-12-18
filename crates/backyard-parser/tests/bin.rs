use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = 5 + 5;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn chained() {
  let asts = parse(
    true,
    "$this->a($a)
  ?? $this->b($b) ?? $this->c($c)
    ?? $this->d($d) ?? $this->e($e);"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
