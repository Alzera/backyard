use backyard_lexer::lex_eval;

#[test]
fn basic() {
  let tokens = lex_eval("\"test\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped() {
  let tokens = lex_eval("\"test \\\" still\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn with_variable() {
  let tokens = lex_eval("\"test $a\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_var() {
  let tokens = lex_eval("\"test \\$a \"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn with_advance() {
  let tokens = lex_eval("\"test {$a}\"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_advance() {
  let tokens = lex_eval("\"test \\{$a} \"").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn backtick() {
  let tokens = lex_eval("`test {$a}`").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn single_quote() {
  let tokens = lex_eval("'test {$a}'").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn escaped_single_quote() {
  let tokens = lex_eval("'test \\' still'").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn nowdoc() {
  let tokens = lex_eval("<<<'TEST'\ntest\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn heredoc() {
  let tokens = lex_eval("<<<TEST\ntest $a\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn spaced_doc() {
  let tokens = lex_eval("<<<   'TEST'  \ntest\nTEST;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
