use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(
    true,
    "class A {
  protected public(get) static var ?A $a = 4;
  public readonly A|callable|null $b = 4, $c = 6;
  public string $d { &get { return $e; } set(int &...$i) => $f; };
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
