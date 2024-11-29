# backyard-lexer

Generating tokens representation of PHP code.

## features

- Parse string to tokens _(lex() & lex_eval())_

## usage

    fn main() {
        let code = r#"<?php
        function hello_world($foo) {
          var_dump($foo);
        }"#;

        let parsed = backyard_lexer::lex(code);
        println!("{:?}", parsed);
    }

Resulting this result:

    Ok([
      Token { token_type: Function, value: "function", line: 2, column: 4, offset: 10 },
      Token { token_type: Identifier, value: "hello_world", line: 2, column: 13, offset: 19 },
      Token { token_type: LeftParenthesis, value: "(", line: 2, column: 24, offset: 30 },
      Token { token_type: Variable, value: "foo", line: 2, column: 25, offset: 31 },
      Token { token_type: RightParenthesis, value: ")", line: 2, column: 29, offset: 35 },
      Token { token_type: LeftCurlyBracket, value: "{", line: 2, column: 31, offset: 37 },
      Token { token_type: Identifier, value: "var_dump", line: 3, column: 6, offset: 45 },
      Token { token_type: LeftParenthesis, value: "(", line: 3, column: 14, offset: 53 },
      Token { token_type: Variable, value: "foo", line: 3, column: 15, offset: 54 },
      Token { token_type: RightParenthesis, value: ")", line: 3, column: 19, offset: 58 },
      Token { token_type: Semicolon, value: ";", line: 3, column: 20, offset: 59 },
      Token { token_type: RightCurlyBracket, value: "}", line: 4, column: 4, offset: 65 }
    ])

## heavily inspired by

- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
