use backyard_parser::parse_eval;

#[test]
fn test_fail() {
  let asts = parse_eval("function a a() {}");
  assert!(asts.is_err());
}
