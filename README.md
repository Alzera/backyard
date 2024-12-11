# backyard

PHP parser written in rust.

## features

- Parse string to tokens _(lex() & lex_eval())_
- Parse string to AST _(parse() & parse_eval())_
- Convert AST back to string _(generator())_

## todo

- [ ] AST Visit API
- [ ] AST Builder API

## usage

- [Node / AST](https://github.com/Alzera/backyard/tree/main/crates/backyard-nodes)
- [Lexer / Tokenizer](https://github.com/Alzera/backyard/tree/main/crates/backyard-lexer)
- [Parser](https://github.com/Alzera/backyard/tree/main/crates/backyard-parser)
- [Generator](https://github.com/Alzera/backyard/tree/main/crates/backyard-generator)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
