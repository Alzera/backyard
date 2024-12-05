use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn single_quote() {
  let asts = parse_eval("$a = ' ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}';").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nowdoc() {
  let asts = parse_eval("echo <<<'START'
a {$a}
START;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn heredoc() {
  let asts = parse_eval("echo <<<START
a {$a}
START;;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
