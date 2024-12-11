use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "switch ($a) {
  case 1:
    break;
  case 2:
    return;
  default:
    continue;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn short() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "switch ($a):
  case 1:
    break;
  case 2:
    return;
  default:
    continue;
endswitch;"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn case_bracket() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "switch ($a) {
  case 1: {
    break;
  }
  default:
    continue;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(asts);
}
