# backyard-parser

Parse PHP code to AST node.

## features

- Parse string to AST _(parse() & parse_eval())_

## usage

    fn main() {
      let arena = bumpalo::Bump::new();
      let code = r#"<?php
      function hello_world($foo) {
        var_dump($foo);
      }"#;

      let parsed = backyard_parser::parse(&arena, code).unwrap();
      println!("{:?}", parsed.print(false, false));
    }

Notice this output is not including leadings, trailings and location because we print it with `print(false, false)`, more on [backyard-nodes](https://crates.io/crates/backyard-nodes).

    ProgramNode
    └-children[]
      ╙-FunctionNode
        ├-is_ref: false
        ├-name
        │ └-IdentifierNode
        │   └-name: "hello_world"
        ├-parameters[]
        │ ╙-ParameterNode
        │   ├-variable_type: -
        │   ├-is_ref: false
        │   ├-is_ellipsis: false
        │   ├-name
        │   │ └-IdentifierNode
        │   │   └-name: "foo"
        │   └-value: -
        ├-return_type: -
        └-body
          └-BlockNode
            └-statements[]
              ╙-CallNode
                ├-name
                │ └-IdentifierNode
                │   └-name: "var_dump"
                └-arguments[]
                  ╙-CallArgumentNode
                    ├-name: -
                    └-value
                      └-VariableNode
                        └-name
                          └-IdentifierNode
                            └-name: "foo"

## ecosystem

- [backyard-nodes (Node / AST, with builder, walker and printer)](https://crates.io/crates/backyard-nodes)
- [backyard-lexer (Tokenizer)](https://crates.io/crates/backyard-lexer)
- [backyard-generator](https://crates.io/crates/backyard-generator)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
