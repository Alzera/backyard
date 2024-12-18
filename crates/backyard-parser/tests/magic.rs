use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "__DIR__ . __FILE__;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
