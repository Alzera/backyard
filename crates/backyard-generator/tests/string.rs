use backyard_generator::generate;
use backyard_parser::arena_parse;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = \" ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}\";"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn single_quote() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(
    &arena,
    true,
    "$a = ' ini string $ \\\" \\$var $b {${\"ale\" . 5}} {$a}';"
  ).unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn nowdoc() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "echo <<<'START'
a {$a}
START;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}

#[test]
fn heredoc() {
  let arena = bumpalo::Bump::new();
  let asts = arena_parse(&arena, true, "echo <<<START
a {$a}
START;;").unwrap();
  insta::assert_yaml_snapshot!(generate(&asts).unwrap());
}
