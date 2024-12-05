use backyard_parser::{ parse, parse_eval };

#[test]
fn test_break() {
  let asts = parse_eval("break;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_break_with_argument() {
  let asts = parse_eval("break 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_continue() {
  let asts = parse_eval("continue;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_continue_with_argument() {
  let asts = parse_eval("continue 2;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_goto() {
  let asts = parse_eval("goto label;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_new() {
  let asts = parse_eval("new Exception();").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_print() {
  let asts = parse_eval("print \"Hello\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_return() {
  let asts = parse_eval("return;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_return_with_argument() {
  let asts = parse_eval("return 4;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_throw() {
  let asts = parse_eval("throw $a;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_parent() {
  let asts = parse_eval("parent::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_static() {
  let asts = parse_eval("static::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_clone() {
  let asts = parse_eval("clone $a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_this() {
  let asts = parse_eval("$this").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_true() {
  let asts = parse_eval("true").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_false() {
  let asts = parse_eval("false").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_null() {
  let asts = parse_eval("null").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_self() {
  let asts = parse_eval("self::a").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn test_inline() {
  let asts = parse("Hello <?= $world ?>").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
