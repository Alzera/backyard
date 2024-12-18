use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(
    true,
    "switch ($a) {
  case 1:
    break;
  case 2:
    return;
  default:
    continue;
}"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn short() {
  let asts = parse(
    true,
    "switch ($a):
  case 1:
    break;
  case 2:
    return;
  default:
    continue;
endswitch;"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn case_bracket() {
  let asts = parse(true, "switch ($a) {
  case 1: {
    break;
  }
  default:
    continue;
}").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
