use backyard_generator::generate_serializable_node;
use backyard_parser::parse;

#[test]
fn basic() {
  let asts = parse(true, "$a = \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn single_quote() {
  let asts = parse(true, "$a = ' ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}';").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn nowdoc() {
  let asts = parse(true, "echo <<<'START'
a {$a}
START;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}

#[test]
fn heredoc() {
  let asts = parse(true, "echo <<<START
a {$a}
START;;").unwrap();
  insta::assert_yaml_snapshot!(generate_serializable_node(&asts).unwrap());
}
