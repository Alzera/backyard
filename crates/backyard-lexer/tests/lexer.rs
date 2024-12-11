use backyard_lexer::{ error::LexError, lex, lex_eval };

#[test]
fn magic() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "__DIR__").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn magic_method() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "__construct").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn type_test() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "int").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn id() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "sample_id").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_equal() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a =& $b = $c === $d == (fn() => $d)();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_ambersand() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a &= $b & $c && $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_hash() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "#[attr]\n$a; # comment").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_question_mark() {
  let arena = bumpalo::Bump::new();
  let tokens = lex(&arena, "<?php $a ??= ($b ?? $c) ? ($d ?: $e) : $f?->g; ?>>>>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_percent() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a %= $b % $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_caret() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a ^= $b ^ $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_star() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a **= $b *= $c * $d ** $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_slash() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = $b / $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_dot() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = $b . $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_pipe() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a |= $b | $c || $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_minus() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a -= $b->c - $d-- - --$e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_gt() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a >>= $b >= $c >> $d > $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_lt() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a <<= $b <= $c << $d < $e <> $f <=> $g;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_colon() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = $b::c ? $d : $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_exclamation() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = $b !== $c != !!$d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_plus() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a += $b++ + ++$c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_square_bracket() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a[1, 2, 3];").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_backslash() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "\\A\\B;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_tilde() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = ~false;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_at() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "$a = @$this();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn unrecognized() {
  let arena = bumpalo::Bump::new();
  let tokens = lex_eval(&arena, "class � {\n}");
  assert_eq!(tokens, Err(LexError::Unrecognized { token: "�".to_string(), line: 1, column: 7 }));
}
