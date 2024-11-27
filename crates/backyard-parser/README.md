# backyard-parser

Parse PHP code to AST node.

## features

- Parse string to AST _(parse() & parse_eval())_

## usage

    fn main() {
        let code = r#"<?php
        function hello_world($foo) {
          var_dump($foo);
        }"#;

        let parsed = backyard_parse::parse(code);
        println!("{:?}", parsed);
    }

Resulting this json:

    Ok([
      Node {
        leadings: [],
        trailings: [],
        node_type: Function,
        node: Function(FunctionNode {
          is_ref: false,
          name: Node {
            leadings: [],
            trailings: [],
            node_type: Identifier,
            node: Identifier(IdentifierNode { name: "hello_world" })
          },
          parameters: [
            Node {
              leadings: [],
              trailings: [],
              node_type: Parameter,
              node: Parameter(ParameterNode {
                variable_type: None,
                is_ref: false,
                is_ellipsis: false,
                name: Node {
                  leadings: [],
                  trailings: [],
                  node_type: Identifier,
                  node: Identifier(IdentifierNode { name: "foo" })
                },
                value: None
              })
            }
          ],
          return_type: None,
          body: Some(Node {
            leadings: [],
            trailings: [],
            node_type: Block,
            node: Block(BlockNode {
              statements: [
                Node {
                  leadings: [],
                  trailings: [],
                  node_type: Call,
                  node: Call(CallNode {
                    name: Node {
                      leadings: [],
                      trailings: [],
                      node_type: Identifier,
                      node: Identifier(IdentifierNode { name: "var_dump" })
                    },
                    arguments: [
                      Node {
                        leadings: [],
                        trailings: [],
                        node_type: CallArgument,
                        node: CallArgument(CallArgumentNode {
                          name: None,
                          value: Node {
                            leadings: [],
                            trailings: [],
                            node_type: Variable,
                            node: Variable(VariableNode {
                              name: Node {
                                leadings: [],
                                trailings: [],
                                node_type: Identifier,
                                node: Identifier(IdentifierNode { name: "foo" })
                              }
                            })
                          }
                        })
                      }
                    ]
                  })
                }
              ]
            })
          })
        })
      }
    ])

## heavily inspired by

- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
