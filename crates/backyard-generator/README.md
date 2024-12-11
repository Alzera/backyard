# backyard-generator

Convert AST node back to PHP code.

## features

- Convert AST back to string _(generator())_

## usage

    fn main() {
      let code = r#"<?php
        function hello_world($foo) {
          var_dump($foo);
        }"#;

      let parsed = backyard_parser::parse(code).unwrap();
      let code = backyard_generator::generate(parsed);
      println!("{:?}", code);
    }

Resulting this code:

    function hello_world($foo) {
      var_dump($foo);
    }

## ecosystem

- [backyard-lexer](https://crates.io/crates/backyard-lexer)
- [backyard-nodes](https://crates.io/crates/backyard-nodes)
- [backyard-parser](https://crates.io/crates/backyard-parser)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
