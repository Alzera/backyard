use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(
    true,
    "trait B {
  use Ale;
  use Loggable, Usable {
    log as private alias;
    Loggable::log as aliasLoggable;
    Usable::useResource insteadof Loggable;
  }
  public const MY_CONST = \"constant\";
  public static ?A $instance = 4;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
