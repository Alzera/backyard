use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = 5 + 5;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn chained() {
  let asts = parse_eval(
    "$this->a($a)
  ?? $this->b($b) ?? $this->c($c)
    ?? $this->d($d) ?? $this->e($e);"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
