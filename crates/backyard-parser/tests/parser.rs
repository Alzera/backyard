use backyard_parser::parse_eval;

#[test]
fn test_fail() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "function a a() {}");
  assert!(asts.is_err());
}
