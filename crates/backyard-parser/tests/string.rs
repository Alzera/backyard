use backyard_parser::parse_eval;

#[test]
fn basic() {
  let asts = parse_eval("$a = \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn single_quote() {
  let asts = parse_eval("$a = ' ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}';").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn nowdoc() {
  let asts = parse_eval("echo <<<'START'
a {$a}
START;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn heredoc() {
  let asts = parse_eval("echo <<<START
a {$a}
START;;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}
