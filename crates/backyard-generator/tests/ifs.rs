use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_else() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) {} else {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_elseif() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) {} elseif (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn with_else_if() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) {} else if (true) {}").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nested() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "if (false) {if (false) {} else (true) {}} else if (true) {}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn no_block() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (false) $e = 5; else if (true) $e = 6;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "if (true): elseif (false): else: endif;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
