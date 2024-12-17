# backyard-generator

Convert AST node back to PHP code.

## features

- Convert AST back to string _(generate())_

## usage

    let arena = bumpalo::Bump::new();
    let code = r#"<?php
    // leading comment
    function hello_world($foo) {
      var_dump($foo);
    }"#;

    let parsed = backyard_parser::parse(&arena, code).unwrap();
    let generated = backyard_generator::generate(&parsed).unwrap();
    println!("{:?}", generated);

Resulting this code:

    // leading comment
    function hello_world($foo) {
      var_dump($foo);
    }

## ecosystem

- [backyard-nodes (Node / AST, with builder, walker and printer)](https://crates.io/crates/backyard-nodes)
- [backyard-lexer (Tokenizer)](https://crates.io/crates/backyard-lexer)
- [backyard-parser](https://crates.io/crates/backyard-parser)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
