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

#[test]
fn doc_tags() {
  let tokens = lex(
    true,
    "/**
 * Example of a comprehensive PHPDoc with all tags.
 * 
 * @param string $name User name.
 * @phpstan-param non-empty-string $name
 * @psalm-param non-empty-string $name
 * @phan-param string $name
 * 
 * @param-immediately-invoked-callable callable $callback
 * @phpstan-param-immediately-invoked-callable callable():void $callback
 * 
 * @param-later-invoked-callable callable $lazyCallback
 * @phpstan-param-later-invoked-callable callable(int):string $lazyCallback
 * 
 * @param-closure-this object $thisObject
 * @phpstan-param-closure-this self $thisObject
 * 
 * @pure-unless-callable-is-impure
 * @phpstan-pure-unless-callable-is-impure
 * 
 * @var int
 * @phpstan-var positive-int
 * @psalm-var positive-int
 * @phan-var int
 * 
 * @return bool
 * @phpstan-return true
 * @psalm-return true
 * @phan-return bool
 * @phan-real-return bool
 * 
 * @throws \\Exception
 * @phpstan-throws \\InvalidArgumentException
 * 
 * @mixin \\MyClass
 * @phan-mixin \\MyTrait
 * 
 * @psalm-require-extends \\BaseClass
 * @phpstan-require-extends \\BaseClass
 * 
 * @psalm-require-implements \\MyInterface
 * @phpstan-require-implements \\MyInterface
 * 
 * @deprecated Use `newMethod()` instead.
 * 
 * @property string $firstName
 * @property-read string $lastName
 * @property-write string $password
 * @phpstan-property non-empty-string $firstName
 * @phpstan-property-read string $lastName
 * @phpstan-property-write string $password
 * @psalm-property string $middleName
 * @psalm-property-read string $nickName
 * @psalm-property-write string $email
 * @phan-property string $address
 * @phan-property-read string $city
 * @phan-property-write string $postalCode
 * 
 * @method void setName(string $name)
 * @phpstan-method void setName(non-empty-string $name)
 * @psalm-method void setName(non-empty-string $name)
 * @phan-method void setName(string $name)
 * 
 * @template T
 * @phpstan-template T
 * @psalm-template T
 * @phan-template T
 * 
 * @template-covariant T
 * @phpstan-template-covariant T
 * @psalm-template-covariant T
 * 
 * @template-contravariant T
 * @phpstan-template-contravariant T
 * @psalm-template-contravariant T
 * 
 * @extends \\BaseCollection<int>
 * @phpstan-extends \\BaseCollection<int>
 * @phan-extends \\BaseCollection<int>
 * @phan-inherits \\BaseCollection
 * 
 * @template-extends \\MyList<string>
 * 
 * @implements \\MyInterface<string>
 * @phpstan-implements \\MyInterface<string>
 * @template-implements \\MyInterface<string>
 * 
 * @use \\HelperTrait
 * @phpstan-use \\HelperTrait<string>
 * @template-use \\HelperTrait<T>
 * 
 * @phpstan-type MyType array<string, mixed>
 * @psalm-type MyType array<string, mixed>
 * @phan-type MyType array<string, mixed>
 * 
 * @phpstan-import-type ImportedType from \\AnotherClass
 * @psalm-import-type ImportedType from \\AnotherClass
 * 
 * @phpstan-assert array $data
 * @phpstan-assert-if-true non-empty-array $data
 * @phpstan-assert-if-false empty-array $data
 * @psalm-assert string $name
 * @psalm-assert-if-true non-empty-string $name
 * @psalm-assert-if-false empty-string $name
 * @phan-assert string $address
 * @phan-assert-if-true non-empty-string $address
 * @phan-assert-if-false empty-string $address
 * 
 * @phpstan-this-out self
 * @phpstan-self-out \\MyClass
 * @psalm-this-out self
 * @psalm-self-out \\MyClass
 * 
 * @param-out string $result
 * @phpstan-param-out non-empty-string $result
 * @psalm-param-out non-empty-string $result
 */
function example(): void {}"
  ).unwrap();
  insta::assert_yaml_snapshot!(tokens);
}
