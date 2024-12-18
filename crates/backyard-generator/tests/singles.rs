use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn test_break() {
  let asts = parse(true, "break;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_break_with_argument() {
  let asts = parse(true, "break 2;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_continue() {
  let asts = parse(true, "continue;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_continue_with_argument() {
  let asts = parse(true, "continue 2;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_goto() {
  let asts = parse(true, "goto label;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_new() {
  let asts = parse(true, "new Exception();").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_print() {
  let asts = parse(true, "print \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_return() {
  let asts = parse(true, "return;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_return_with_argument() {
  let asts = parse(true, "return 4;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_throw() {
  let asts = parse(true, "throw $a;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_parent() {
  let asts = parse(true, "parent::a").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_static() {
  let asts = parse(true, "static::a").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_clone() {
  let asts = parse(true, "clone $a").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_this() {
  let asts = parse(true, "$this").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_true() {
  let asts = parse(true, "true").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_false() {
  let asts = parse(true, "false").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_null() {
  let asts = parse(true, "null").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_self() {
  let asts = parse(true, "self::a").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn test_inline() {
  let asts = parse(false, "Hello <?= $world ?>").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
