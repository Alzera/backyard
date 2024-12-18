use backyard_lexer::{ error::LexError, lex };

#[test]
fn magic() {
  let tokens = lex(true, "__DIR__").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn magic_method() {
  let tokens = lex(true, "__construct").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn type_test() {
  let tokens = lex(true, "int").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn unqualified_name() {
  let tokens = lex(true, "sample_id").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn qualified_name() {
  let tokens = lex(true, "sample_id\\a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn fully_qualified_name() {
  let tokens = lex(true, "\\sample_id\\a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn relative_name() {
  let tokens = lex(true, "namespace\\sample_id\\a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_equal() {
  let tokens = lex(true, "$a =& $b = $c === $d == (fn() => $d)();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_ambersand() {
  let tokens = lex(true, "$a &= $b & $c && $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_hash() {
  let tokens = lex(true, "#[attr]\n$a; # comment").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_question_mark() {
  let tokens = lex(false, "<?php $a ??= ($b ?? $c) ? ($d ?: $e) : $f?->g; ?>>>>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_percent() {
  let tokens = lex(true, "$a %= $b % $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_caret() {
  let tokens = lex(true, "$a ^= $b ^ $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_star() {
  let tokens = lex(true, "$a **= $b *= $c * $d ** $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_slash() {
  let tokens = lex(true, "$a = $b / $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_dot() {
  let tokens = lex(true, "$a = $b . $c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_pipe() {
  let tokens = lex(true, "$a |= $b | $c || $d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_minus() {
  let tokens = lex(true, "$a -= $b->c - $d-- - --$e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_gt() {
  let tokens = lex(true, "$a >>= $b >= $c >> $d > $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_lt() {
  let tokens = lex(true, "$a <<= $b <= $c << $d < $e <> $f <=> $g;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_colon() {
  let tokens = lex(true, "$a = $b::c ? $d : $e;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_exclamation() {
  let tokens = lex(true, "$a = $b !== $c != !!$d;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_plus() {
  let tokens = lex(true, "$a += $b++ + ++$c;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_square_bracket() {
  let tokens = lex(true, "$a[1, 2, 3];").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_backslash() {
  let tokens = lex(true, "\\A\\B;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_tilde() {
  let tokens = lex(true, "$a = ~false;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn test_at() {
  let tokens = lex(true, "$a = @$this();").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn unrecognized() {
  let tokens = lex(true, "class � {\n}");
  assert_eq!(tokens, Err(LexError::Unrecognized { token: "�".to_string(), line: 1, column: 7 }));
}

#[test]
fn test_halt() {
  let tokens = lex(true, "$a = false;\n__halt_compiler();\nThis text is invalid").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
