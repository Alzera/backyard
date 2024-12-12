# backyard

PHP parser written in rust.

## features

- Parse string to tokens _(lex() & lex_eval())_
- Parse string to AST _(parse() & parse_eval())_
- Convert AST back to string _(generator())_
- Simplify building AST nodes _(Builder)_
- Simple walker through AST nodes _(Walker)_

## todo

- [ ] Standarize Tokens and Nodes

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
