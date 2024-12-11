use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test \\\" still\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn with_variable() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test $a\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_var() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test \\$a \"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn with_advance() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test {$a}\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_advance() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\"test \\{$a} \"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn backtick() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "`test {$a}`").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn single_quote() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "'test {$a}'").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_single_quote() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "'test \\' still'").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn nowdoc() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "<<<'TEST'\ntest\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn heredoc() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "<<<TEST\ntest $a\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn spaced_doc() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "<<<   'TEST'  \ntest\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
