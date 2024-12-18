use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A {
  protected public(get) static var ?A $a = 4;
  public readonly A|callable|null $b = 4, $c = 6;
  public string $d { &get { return $e; } set(int &...$i) => $f; };
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
