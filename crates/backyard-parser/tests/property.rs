use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval(
    "class A {
  protected public(get) static var ?A $a = 4;
  public readonly A|callable|null $b = 4;
  public string $a { &get { return $a; } set(int &...$i) => $a; };
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
