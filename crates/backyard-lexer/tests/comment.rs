use backyard_lexer::lex;

#[test]
fn line() {
  let tokens = lex(true, "// test").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_new_line() {
  let tokens = lex(true, "// test\n$a").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn line_tag_close() {
  let tokens = lex(false, "<?php // test ?>").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn block() {
  let tokens = lex(true, "/* test */").unwrap();
  for token in tokens.iter() {
    println!("{token:?}");
  }
  println!("");
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_1() {
  let tokens = lex(true, "/** @param string $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_2() {
  let tokens = lex(true, "/** @param   string   $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_3() {
  let tokens = lex(true, "/** @param  ( string )  $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_4() {
  let tokens = lex(true, "/** @param ( ( string ) ) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_5() {
  let tokens = lex(true, "/** @param \\\\Foo\\Bar\\\\Baz $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_6() {
  let tokens = lex(true, "/** @param   \\\\Foo\\Bar\\\\Baz   $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_7() {
  let tokens = lex(true, "/** @param  ( \\\\Foo\\Bar\\\\Baz )  $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_8() {
  let tokens = lex(true, "/** @param ( ( \\\\Foo\\Bar\\\\Baz ) ) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_9() {
  let tokens = lex(true, "/** @param string|int $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_10() {
  let tokens = lex(true, "/** @param string | int $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_11() {
  let tokens = lex(true, "/** @param (string | int) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_12() {
  let tokens = lex(true, "/** @param string | int | float $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_13() {
  let tokens = lex(true, "/** @param string&int $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_14() {
  let tokens = lex(true, "/** @param string & int $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_15() {
  let tokens = lex(true, "/** @param (string & int) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_16() {
  let tokens = lex(true, "/** @param (\n  string\n  &\n  int\n) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_17() {
  let tokens = lex(true, "/** @param string & int & float $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_18() {
  let tokens = lex(true, "/** @param string & (int | float) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_19() {
  let tokens = lex(true, "/** @param string | (int & float) $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_20() {
  let tokens = lex(true, "/** @param string & int | float $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_21() {
  let tokens = lex(true, "/** @param string | int & float $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_22() {
  let tokens = lex(true, "/** @param string[] $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_23() {
  let tokens = lex(true, "/** @param string [  ]  $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_24() {
  let tokens = lex(true, "/** @param (string | int | float)[] $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_25() {
  let tokens = lex(true, "/** @param string[][][] $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_26() {
  let tokens = lex(true, "/** @param string [  ] [][] $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_27() {
  let tokens = lex(true, "/** @param (((string | int | float)[])[])[] $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_28() {
  let tokens = lex(true, "/** @param $this $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_29() {
  let tokens = lex(true, "/** @param ?int $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_30() {
  let tokens = lex(true, "/** @param ?Foo<Bar> $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_31() {
  let tokens = lex(true, "/** @param array<int, Foo\\Bar> $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_32() {
  let tokens = lex(true, "/** @param array {'a': int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_33() {
  let tokens = lex(true, "/** @param array{a: int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_34() {
  let tokens = lex(true, "/** @param array{a: ?int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_35() {
  let tokens = lex(true, "/** @param array{a?: ?int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_36() {
  let tokens = lex(true, "/** @param array{0: int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_37() {
  let tokens = lex(true, "/** @param array{0?: int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_38() {
  let tokens = lex(true, "/** @param array{int, int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_39() {
  let tokens = lex(true, "/** @param array{a: int, b: string} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_40() {
  let tokens = lex(
    true,
    "/** @param array{a?: int, b: string, 0: int, 1?: DateTime, hello: string} $foo */"
  ).unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_41() {
  let tokens = lex(true, "/** @param array{a: int, b: array{c: callable(): int}} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_type_42() {
  let tokens = lex(true, "/** @param ?array{a: int} $foo */").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_no_var() {
  let tokens = lex(true, "/** @param string testing */\n$a = 5;").unwrap();
  insta::assert_yaml_snapshot!(tokens);
}

#[test]
fn doc_param_description() {
  let tokens = lex(
    true,
    "/** @param string $foo testing
  *                    second line description*/\n$a = 5;"
  ).unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
