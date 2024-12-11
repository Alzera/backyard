use backyard_generator::generate;
use backyard_parser::parse_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "$a = \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn single_quote() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(
    &arena,
    "$a = ' ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}';"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nowdoc() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "echo <<<'START'
a {$a}
START;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn heredoc() {
  let arena = bumpalo::Bump::new();
  let asts = parse_eval(&arena, "echo <<<START
a {$a}
START;;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
