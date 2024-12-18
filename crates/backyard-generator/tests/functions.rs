use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "function &a(?int ...$b = 0, String &$c = [0.01, 0x12], bool $d): ?int {\n}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn construct() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "class A {
  public function __construct(protected int $x, protected string &...$y = 0) {
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn arrow() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "$a = fn &(int $x): ?int => null;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn anonymous() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = function &(int $x, ?int $y) use ($arg2): static {\n};"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
