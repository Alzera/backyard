use backyard_parser::parse;

#[test]
fn test_fail() {
  let asts = parse(true, "function a a() {}");
  assert!(asts.is_err());
}
