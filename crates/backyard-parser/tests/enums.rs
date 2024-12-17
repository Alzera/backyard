use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "enum Suit {
  case Hearts;
  case Spades;
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn typed() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "enum Suit: int {
  case Hearts = 5;
  case Spades = 6;
}").unwrap();
  insta::assert_yaml_snapshot!(asts.serializable());
}

#[test]
fn content() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
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
  insta::assert_yaml_snapshot!(asts.serializable());
}
