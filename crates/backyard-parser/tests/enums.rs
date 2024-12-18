use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "enum Suit {
  case Hearts;
  case Spades;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn typed() {
  let asts = parse(true, "enum Suit: int {
  case Hearts = 5;
  case Spades = 6;
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn content() {
  let asts = parse(
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
  insta::assert_yaml_snapshot!(asts);
}
