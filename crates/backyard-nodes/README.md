# backyard-nodes

Nodes representing PHP code AST, with simple builder and walker.

## features

- Ast Nodes
- _"builder"_ simplify building AST nodes
- _"walker"_ simple walker through AST nodes

## usage

### builder

This builder is behind the `builder` feature.

    use backyard_nodes::builder::{ Builder, BlueprintBuildable };
    fn main() {
      let arena = bumpalo::Bump::new();
      let b = Builder::new();
      let node = b
        .Program(&[b.Assignment(b.Variable(b.Identifier("a")), "=", b.Number("21"))])
        .build(&arena);
      println!("{node:?}");
    }

Resulting this:

    Node {
      node_type: Program,
      wrapper: Program(ProgramNode {
        children: [
          Node {
            node_type: Assignment,
            wrapper: Assignment(AssignmentNode {
              left: Node {
                node_type: Variable,
                wrapper: Variable(VariableNode {
                  name: Node {
                    node_type: Identifier,
                    wrapper: Identifier(IdentifierNode { name: "a" }),
                    loc: None, leadings: None, trailings: None
                  }
                }),
                loc: None, leadings: None, trailings: None
              },
              operator: "=",
              right: Node {
                node_type: Number,
                wrapper: Number(NumberNode { value: "21" }),
                loc: None, leadings: None, trailings: None
              }
            }),
            loc: None, leadings: None, trailings: None
          }]
        }),
      loc: None, leadings: None, trailings: None
    }

### walker

This builder is behind the `walker` feature.

    use backyard_nodes::{ builder::{ BlueprintBuildable, Builder }, walker::Walker, NodeType };
    #[test]
    fn builder() {
      let arena = bumpalo::Bump::new();
      let b = Builder::new();
      let node = b
        .Program(&[b.Assignment(b.Variable(b.Identifier("a")), "=", b.Number("21"))])
        .build(&arena);
      let mut walker = Walker::new(&*node).into_iter();

      assert_eq!(NodeType::Program, walker.next().unwrap().node_type);
      assert_eq!(NodeType::Assignment, walker.next().unwrap().node_type);
      assert_eq!(NodeType::Variable, walker.next().unwrap().node_type);
      assert_eq!(NodeType::Identifier, walker.next().unwrap().node_type);
      assert_eq!(NodeType::Number, walker.next().unwrap().node_type);
      assert!(walker.next().is_none());
    }

## ecosystem

- [backyard-lexer (Tokenizer)](https://crates.io/crates/backyard-lexer)
- [backyard-parser](https://crates.io/crates/backyard-parser)
- [backyard-generator](https://crates.io/crates/backyard-generator)

## heavily inspired by

- [oxc-project/oxc](https://github.com/oxc-project/oxc)
- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)

## license

[MIT](https://github.com/Alzera/backyard/blob/main/LICENSE)
