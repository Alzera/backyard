use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "enum Suit {
  case Hearts;
  case Spades;
}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn typed() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "enum Suit: int {
  case Hearts = 5;
  case Spades = 6;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn content() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "enum Suit implements SuitInterface {
  case Hearts;
  case Spades;

  public const MY_CONST = \"constant\";

  public function color(): string {
    return match($this) {
      Suit::Hearts, Suit::Diamonds => 'Red',
      Suit::Clubs, Suit::Spades => 'Black'
    };
  }
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
