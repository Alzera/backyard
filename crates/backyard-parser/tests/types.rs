use backyard_parser::parse_eval;

#[test]
fn union() {
  let asts = parse_eval("class A {
  private array|\\Closure $suggestedValues = [];
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn intersection() {
  let asts = parse_eval("class A {
    protected \\A&\\B $currentHandler;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn parenthesis() {
  let asts = parse_eval("class A {
  protected ((\\A|\\C)&\\B)|null $currentHandler2;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn break_on_var() {
  let asts = parse_eval("class A {
  function __construct(protected A&null &$a){
  }
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn single_parenthesis() {
  let asts = parse_eval("class A {
  protected (int) $a;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
