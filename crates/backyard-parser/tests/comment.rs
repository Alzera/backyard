use backyard_parser::{ parse, parse_in };

#[test]
fn line_basic() {
  let asts = parse(true, "// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn line_long() {
  let asts = parse(true, "///////////// TEST /////////////").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn block() {
  let asts = parse(true, "/*\ntest\n*/").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn before_block() {
  let asts = parse(true, "if (false) // test
{
}").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn after_block() {
  let asts = parse(true, "do {
}
// test
while(false);").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_block_end() {
  let asts = parse(true, "if (false) {
}
// test").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn at_statement_end() {
  let asts = parse(true, "$a = 5 /* test */;").unwrap();
  insta::assert_yaml_snapshot!(asts);
}

#[test]
fn doc_param_type_1() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// #[test]
// fn doc_param_type_2() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param   string   $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_3() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param  ( string )  $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

#[test]
fn doc_param_type_4() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param ( ( string ) ) $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// #[test]
// fn doc_param_type_5() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param \\\\Foo\\Bar\\\\Baz $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_6() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param   \\\\Foo\\Bar\\\\Baz   $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_7() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param  ( \\\\Foo\\Bar\\\\Baz )  $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_8() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param ( ( \\\\Foo\\Bar\\\\Baz ) ) $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_9() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string|int $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_10() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string | int $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_11() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param (string | int) $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

#[test]
fn doc_param_type_12() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string | int | float $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// #[test]
// fn doc_param_type_13() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string&int $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_14() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string & int $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_15() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param (string & int) $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

#[test]
fn doc_param_type_16() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param (\n  string\n  &\n  int\n) $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// #[test]
// fn doc_param_type_17() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string & int & float $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_18() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string & (int | float) $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

#[test]
fn doc_param_type_19() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string | (int & float) $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// #[test]
// fn doc_param_type_20() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string & int | float $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

// #[test]
// fn doc_param_type_21() {
//   let arena = bumpalo::Bump::new();
//   let tokens = parse_in(&arena, true, "/** @param string | int & float $foo */").unwrap();
//   println!("{:?}", tokens.print(true, false));
//   insta::assert_yaml_snapshot!(tokens.serializable());
// }

#[test]
fn doc_param_type_22() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string[] $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

// array type []
// generic type ?Foo<Bar>

#[test]
fn doc_param_type_23() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string [  ]  $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_24() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param (string | int | float)[] $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_25() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string[][][] $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_26() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param string [  ] [][] $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_27() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(
    &arena,
    true,
    "/** @param (((string | int | float)[])[])[] $foo */"
  ).unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_28() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param $this $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_29() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param ?int $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_30() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param ?Foo<Bar> $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_31() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array<int, Foo\\Bar> $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_32() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array {'a': int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_33() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{a: int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_34() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{a: ?int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_35() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{a?: ?int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_36() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{0: int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_37() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{0?: int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_38() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{int, int} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}

#[test]
fn doc_param_type_39() {
  let arena = bumpalo::Bump::new();
  let tokens = parse_in(&arena, true, "/** @param array{a: int, b: string} $foo */").unwrap();
  println!("{:?}", tokens.print(true, false));
  insta::assert_yaml_snapshot!(tokens.serializable());
}
