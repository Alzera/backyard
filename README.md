# backyard

PHP parser written in rust.

## features

- Parse string to tokens _(lex() & lex_eval())_ (`backyard-lexer`)
- Parse string to AST _(parse() & parse_eval())_ (`backyard-parser`)
- Convert AST back to string _(generate())_ (`backyard-generator`)
- Simplify building AST nodes (`backyard-nodes`, behind the `builder` feature)
- Walker through AST nodes, support explorer to ancestors and siblings (`backyard-nodes`, behind the `walker` feature)
- Print AST nodes as treeline (`backyard-nodes`, behind the `printer` feature)

## usage

- [backyard-nodes (Node / AST, with builder and walker)](https://github.com/Alzera/backyard/tree/main/crates/backyard-nodes)
- [backyard-lexer (Tokenizer)](https://github.com/Alzera/backyard/tree/main/crates/backyard-lexer)
- [backyard-parser](https://github.com/Alzera/backyard/tree/main/crates/backyard-parser)
- [backyard-generator](https://github.com/Alzera/backyard/tree/main/crates/backyard-generator)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
