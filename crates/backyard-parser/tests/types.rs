use backyard_parser::parse_eval;

#[test]
fn union() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "class A {
  private array|\\Closure $suggestedValues = [];
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn intersection() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A {
    protected \\A&\\B $currentHandler;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "class A {
  protected ((\\A|\\C)&\\B)|null $currentHandler2;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn break_on_var() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "class A {
  function __construct(protected A&null &$a){
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn single_parenthesis() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "class A {
  protected (int) $a;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
