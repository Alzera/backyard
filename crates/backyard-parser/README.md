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

Resulting this result:

    Ok(Node {
      leadings: [],
      trailings: [],
      node_type: Program,
      node: Program(ProgramNode {
        children: [
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
                node: Identifier(IdentifierNode { name: "hello_world" }),
                loc: Some(RangeLocation {
                  start: Location {
                    line: 2,
                    column: 13,
                    offset: 19
                  },
                  end: Location {
                    line: 2,
                    column: 24,
                    offset: 30
                  }
                })
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
                      node: Identifier(IdentifierNode { name: "foo" }),
                      loc: Some(RangeLocation {
                        start: Location {
                          line: 2,
                          column: 25,
                          offset: 31
                        },
                        end: Location {
                          line: 2,
                          column: 28,
                          offset: 34
                        }
                      })
                    },
                    value: None
                  }),
                  loc: Some(RangeLocation {
                    start: Location {
                      line: 2,
                      column: 25,
                      offset: 31
                    },
                    end: Location {
                      line: 2,
                      column: 25,
                      offset: 31
                    }
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
                          node: Identifier(IdentifierNode { name: "var_dump" }),
                          loc: Some(RangeLocation {
                            start: Location {
                              line: 3,
                              column: 6,
                              offset: 45
                            },
                            end: Location {
                              line: 3,
                              column: 14,
                              offset: 53
                            }
                          })
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
                                    node: Identifier(IdentifierNode { name: "foo" }),
                                    loc: Some(RangeLocation {
                                      start: Location {
                                        line: 3,
                                        column: 15,
                                        offset: 54
                                      },
                                      end: Location {
                                        line: 3,
                                        column: 18,
                                        offset: 57
                                      }
                                    })
                                  }
                                }),
                                loc: Some(RangeLocation {
                                  start: Location {
                                    line: 3,
                                    column: 15,
                                    offset: 54
                                  },
                                  end: Location {
                                    line: 3,
                                    column: 18,
                                    offset: 57
                                  }
                                })
                              }
                            }),
                            loc: Some(RangeLocation {
                              start: Location {
                                line: 3,
                                column: 15,
                                offset: 54
                              },
                              end: Location {
                                line: 3,
                                column: 15,
                                offset: 54
                              }
                            })
                          }
                        ]
                      }),
                      loc: Some(RangeLocation {
                        start: Location {
                          line: 3,
                          column: 14,
                          offset: 53
                        },
                        end: Location {
                          line: 3,
                          column: 19,
                          offset: 58
                        }
                      })
                    }
                  ]
                }),
                loc: Some(RangeLocation {
                  start: Location {
                    line: 2,
                    column: 31,
                    offset: 37
                  },
                  end: Location {
                    line: 4,
                    column: 4,
                    offset: 65
                  }
                })
              })
            }),
            loc: Some(RangeLocation {
              start: Location {
                line: 2,
                column: 4,
                offset: 10
              },
              end: Location {
                line: 4,
                column: 4,
                offset: 65
              }
            })
          }
        ]
      }),
      loc: Some(RangeLocation {
        start: Location {
          line: 1,
          column: 0,
          offset: 0
        },
        end: Location {
          line: 4,
          column: 4,
          offset: 65
        }
      })
    })

## heavily inspired by

- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
