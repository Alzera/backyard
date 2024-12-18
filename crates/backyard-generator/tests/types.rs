use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn union() {
  let asts = parse(true, "class A {
  private array|\\Closure $suggestedValues = [];
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn intersection() {
  let asts = parse(true, "class A {
    protected \\A&\\B $currentHandler;
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn parenthesis() {
  let asts = parse(true, "class A {
  protected ((\\A|\\C)&\\B)|null $currentHandler2;
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn break_on_var() {
  let asts = parse(true, "class A {
  function __construct(protected A&null &$a){
  }
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn single_parenthesis() {
  let asts = parse(true, "class A {
  protected (int) $a;
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
