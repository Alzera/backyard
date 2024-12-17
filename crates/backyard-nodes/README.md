# backyard-nodes

Nodes representing PHP code AST, with simple builder and walker.

## features

- Ast Nodes
- _"builder"_ simplify building AST nodes (behind the `builder` feature)
- _"walker"_ walker through AST nodes, support explorer to ancestors and siblings (behind the `walker` feature)
- _"printer"_ print AST nodes as treeline (behind the `printer` feature)

## usage

### builder

    use backyard_nodes::{ builder::{ BlueprintBuildable, BoxBlueprint, Builder }, AssignmentType };

    fn main() {
      let arena = bumpalo::Bump::new();
      let b = Builder::new();
      let node = b
        .Program(
          &[
            b
              .Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))
              .add_leading(b.CommentLine("Test leading")),
          ]
        )
        .build(&arena);

      println!("{:?}", node.print(true, false));
    }

Resulting this:

    ProgramNode
    ├-children[]
    │ ╙-AssignmentNode
    │   ├-left
    │   │ └-VariableNode
    │   │   ├-name
    │   │   │ └-IdentifierNode
    │   │   │   ├-name: "a"
    │   │   │   ├-leadings: -
    │   │   │   └-trailings: -
    │   │   ├-leadings: -
    │   │   └-trailings: -
    │   ├-operator: AssignmentType::Default
    │   ├-right
    │   │ └-NumberNode
    │   │   ├-value: "21"
    │   │   ├-leadings: -
    │   │   └-trailings: -
    │   ├-leadings[]
    │   │ ╙-CommentLineNode
    │   │   ├-comment: "Test leading"
    │   │   ├-leadings: -
    │   │   └-trailings: -
    │   └-trailings: -
    ├-leadings: -
    └-trailings: -

### walker

    use backyard_nodes::{
      builder::{ BlueprintBuildable, BoxBlueprint, Builder },
      AssignmentType,
      NodeType,
    };

    fn main() {
      let arena = bumpalo::Bump::new();
      let b = Builder::new();
      let node = b
        .Program(
          &[
            b
              .Assignment(b.Variable(b.Identifier("a")), AssignmentType::Default, b.Number("21"))
              .add_leading(b.CommentLine("Test leading")),
          ]
        )
        .build(&arena);
      let mut walker = node.walk();

      assert_eq!(NodeType::Program, walker.next().unwrap().1.node_type);
      assert_eq!(NodeType::Assignment, walker.next().unwrap().1.node_type);
      assert_eq!(NodeType::Variable, walker.next().unwrap().1.node_type);
      assert_eq!(NodeType::Identifier, walker.next().unwrap().1.node_type);
      assert_eq!(NodeType::Number, walker.next().unwrap().1.node_type);
      assert!(walker.next().is_none());
    }

### printer

Printer has 2 parameters, first is to print leadings and trailings, second is to print location. We use parser for this example, more on [backyard-parser](https://crates.io/crates/backyard-parser).

    fn main() {
      let arena = bumpalo::Bump::new();
      let code = r#"<?php
      // leading comment
      function hello_world($foo) {
        var_dump($foo);
      }"#;

      let parsed = backyard_parser::parse(&arena, code).unwrap();
      println!("{:?}", parsed.print(true, true));
    }

Resulting this:

    ProgramNode
    ├-children[]
    │ ╙-FunctionNode
    │   ├-is_ref: false
    │   ├-name
    │   │ └-IdentifierNode
    │   │   ├-name: "hello_world"
    │   │   ├-leadings: -
    │   │   ├-trailings: -
    │   │   └-location
    │   │     └-start: line 3, column 11, offset 38
    │   │       end: line 3, column 22, offset 49
    │   ├-parameters[]
    │   │ ╙-ParameterNode
    │   │   ├-variable_type: -
    │   │   ├-is_ref: false
    │   │   ├-is_ellipsis: false
    │   │   ├-name
    │   │   │ └-IdentifierNode
    │   │   │   ├-name: "foo"
    │   │   │   ├-leadings: -
    │   │   │   ├-trailings: -
    │   │   │   └-location
    │   │   │     └-start: line 3, column 23, offset 50
    │   │   │       end: line 3, column 26, offset 53
    │   │   ├-value: -
    │   │   ├-leadings: -
    │   │   ├-trailings: -
    │   │   └-location
    │   │     └-start: line 3, column 23, offset 50
    │   │       end: line 3, column 23, offset 50
    │   ├-return_type: -
    │   ├-body
    │   │ └-BlockNode
    │   │   ├-statements[]
    │   │   │ ╙-CallNode
    │   │   │   ├-name
    │   │   │   │ └-IdentifierNode
    │   │   │   │   ├-name: "var_dump"
    │   │   │   │   ├-leadings: -
    │   │   │   │   ├-trailings: -
    │   │   │   │   └-location
    │   │   │   │     └-start: line 4, column 4, offset 62
    │   │   │   │       end: line 4, column 12, offset 70
    │   │   │   ├-arguments[]
    │   │   │   │ ╙-CallArgumentNode
    │   │   │   │   ├-name: -
    │   │   │   │   ├-value
    │   │   │   │   │ └-VariableNode
    │   │   │   │   │   ├-name
    │   │   │   │   │   │ └-IdentifierNode
    │   │   │   │   │   │   ├-name: "foo"
    │   │   │   │   │   │   ├-leadings: -
    │   │   │   │   │   │   ├-trailings: -
    │   │   │   │   │   │   └-location
    │   │   │   │   │   │     └-start: line 4, column 13, offset 71
    │   │   │   │   │   │       end: line 4, column 16, offset 74
    │   │   │   │   │   ├-leadings: -
    │   │   │   │   │   ├-trailings: -
    │   │   │   │   │   └-location
    │   │   │   │   │     └-start: line 4, column 13, offset 71
    │   │   │   │   │       end: line 4, column 16, offset 74
    │   │   │   │   ├-leadings: -
    │   │   │   │   ├-trailings: -
    │   │   │   │   └-location
    │   │   │   │     └-start: line 4, column 13, offset 71
    │   │   │   │       end: line 4, column 13, offset 71
    │   │   │   ├-leadings: -
    │   │   │   ├-trailings: -
    │   │   │   └-location
    │   │   │     └-start: line 4, column 12, offset 70
    │   │   │       end: line 4, column 17, offset 75
    │   │   ├-leadings: -
    │   │   ├-trailings: -
    │   │   └-location
    │   │     └-start: line 3, column 29, offset 56
    │   │       end: line 5, column 2, offset 80
    │   ├-leadings[]
    │   │ ╙-CommentLineNode
    │   │   ├-comment: " leading comment"
    │   │   ├-leadings: -
    │   │   ├-trailings: -
    │   │   └-location
    │   │     └-start: line 2, column 2, offset 8
    │   │       end: line 2, column 2, offset 8
    │   ├-trailings: -
    │   └-location
    │     └-start: line 3, column 2, offset 29
    │       end: line 5, column 2, offset 80
    ├-leadings: -
    ├-trailings: -
    └-location
      └-start: line 1, column 0, offset 0
        end: line 5, column 2, offset 80

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
