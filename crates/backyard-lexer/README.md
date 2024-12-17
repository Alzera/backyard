# backyard-lexer

Generating tokens representation of PHP code.

## features

- Parse string to tokens _(lex() & lex_eval())_

## usage

    fn main() {
      let arena = bumpalo::Bump::new();
      let code = r#"<?php
      function hello_world($foo) {
        var_dump($foo);
      }"#;

      let tokens = backyard_lexer::lex(&arena, code);
      println!("{:?}", tokens);
    }

Resulting this:

    Ok([
      Token { token_type: Function, value: "function", line: 2, column: 2, offset: 8 },
      Token { token_type: UnqualifiedName, value: "hello_world", line: 2, column: 11, offset: 17 },
      Token { token_type: LeftParenthesis, value: "(", line: 2, column: 22, offset: 28 },
      Token { token_type: Variable, value: "foo", line: 2, column: 23, offset: 29 },
      Token { token_type: RightParenthesis, value: ")", line: 2, column: 27, offset: 33 },
      Token { token_type: LeftCurlyBracket, value: "{", line: 2, column: 29, offset: 35 },
      Token { token_type: UnqualifiedName, value: "var_dump", line: 3, column: 4, offset: 41 },
      Token { token_type: LeftParenthesis, value: "(", line: 3, column: 12, offset: 49 },
      Token { token_type: Variable, value: "foo", line: 3, column: 13, offset: 50 },
      Token { token_type: RightParenthesis, value: ")", line: 3, column: 17, offset: 54 },
      Token { token_type: Semicolon, value: ";", line: 3, column: 18, offset: 55 },
      Token { token_type: RightCurlyBracket, value: "}", line: 4, column: 2, offset: 59 }
    ])

## ecosystem

- [backyard-nodes (Node / AST, builder, walker and printer)](https://crates.io/crates/backyard-nodes)
- [backyard-parser](https://crates.io/crates/backyard-parser)
- [backyard-generator](https://crates.io/crates/backyard-generator)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
