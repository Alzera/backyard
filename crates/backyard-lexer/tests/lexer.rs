use backyard_lexer::{ error::LexError, lex, lex_eval };

#[test]
fn magic() {
  let tokens = lex_eval("__DIR__").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn type_test() {
  let tokens = lex_eval("int").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn id() {
  let tokens = lex_eval("sample_id").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_equal() {
  let tokens = lex_eval("$a =& $b = $c === $d == (fn() => $d)();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_ambersand() {
  let tokens = lex_eval("$a &= $b & $c && $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_hash() {
  let tokens = lex_eval("#[attr]\n$a; # comment").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_question_mark() {
  let tokens = lex("<?php $a ??= ($b ?? $c) ? ($d ?: $e) : $f?->g; ?>>>>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_percent() {
  let tokens = lex_eval("$a %= $b % $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_caret() {
  let tokens = lex_eval("$a ^= $b ^ $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_star() {
  let tokens = lex_eval("$a **= $b *= $c * $d ** $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_slash() {
  let tokens = lex_eval("$a = $b / $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_dot() {
  let tokens = lex_eval("$a = $b . $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_pipe() {
  let tokens = lex_eval("$a |= $b | $c || $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_minus() {
  let tokens = lex_eval("$a -= $b->c - $d-- - --$e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_gt() {
  let tokens = lex_eval("$a >>= $b >= $c >> $d > $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_lt() {
  let tokens = lex_eval("$a <<= $b <= $c << $d < $e <> $f <=> $g;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_colon() {
  let tokens = lex_eval("$a = $b::c ? $d : $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_exclamation() {
  let tokens = lex_eval("$a = $b !== $c != !!$d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_plus() {
  let tokens = lex_eval("$a += $b++ + ++$c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_square_bracket() {
  let tokens = lex_eval("$a[1, 2, 3];").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_backslash() {
  let tokens = lex_eval("\\A\\B;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_tilde() {
  let tokens = lex_eval("$a = ~false;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_at() {
  let tokens = lex_eval("$a = @$this();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn unrecognized() {
  let tokens = lex_eval("class � {\n}");
  assert_eq!(tokens, Err(LexError::Unrecognized { token: "�".to_string(), line: 1, column: 7 }));
}
