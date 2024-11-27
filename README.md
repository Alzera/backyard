# backyard

PHP parser for NodeJs written with rust.

## features

- Parse string to tokens _(lex() & lex_eval())_
- Parse string to AST _(parse() & parse_eval())_
- Convert AST back to string _(generator())_
- Simple AST builder _(builder)_

## todo

- [ ] AST Visit API
- [ ] Improve unit testing
- [ ] Benchmarking
- [ ] Split base code, so it can be ported to another environtment _(if anyone can help me with this, reference link or something, I would be gratefull)_

## usage

### lex

    const { lex } = require("../dist");

    const code = `<?php
    function hello_world($foo) {
      var_dump($foo);
    }`;

    const tokens = lex(code);
    console.log(JSON.stringify(tokens, null, 2));

Resulting this json:

    [
      {
        "token_type": "Function",
        "value": "function",
        "line": 2,
        "column": 0,
        "offset": 6
      },
      {
        "token_type": "Identifier",
        "value": "hello_world",
        "line": 2,
        "column": 9,
        "offset": 15
      },
      {
        "token_type": "LeftParenthesis",
        "value": "(",
        "line": 2,
        "column": 20,
        "offset": 26
      },
      {
        "token_type": "Variable",
        "value": "foo",
        "line": 2,
        "column": 21,
        "offset": 27
      },
      {
        "token_type": "RightParenthesis",
        "value": ")",
        "line": 2,
        "column": 25,
        "offset": 31
      },
      {
        "token_type": "LeftCurlyBracket",
        "value": "{",
        "line": 2,
        "column": 27,
        "offset": 33
      },
      {
        "token_type": "Identifier",
        "value": "var_dump",
        "line": 3,
        "column": 2,
        "offset": 37
      },
      {
        "token_type": "LeftParenthesis",
        "value": "(",
        "line": 3,
        "column": 10,
        "offset": 45
      },
      {
        "token_type": "Variable",
        "value": "foo",
        "line": 3,
        "column": 11,
        "offset": 46
      },
      {
        "token_type": "RightParenthesis",
        "value": ")",
        "line": 3,
        "column": 15,
        "offset": 50
      },
      {
        "token_type": "Semicolon",
        "value": ";",
        "line": 3,
        "column": 16,
        "offset": 51
      },
      {
        "token_type": "RightCurlyBracket",
        "value": "}",
        "line": 4,
        "column": 0,
        "offset": 53
      }
    ]

### parse

    const { parse } = require("../dist");

    const code = `<?php
    function hello_world($foo) {
      var_dump($foo);
    }`;

    const parsed = parse(code);
    console.log(JSON.stringify(parsed, null, 2));

Resulting this json:

    [
      {
        "leadings": [],
        "trailings": [],
        "node_type": "function",
        "is_ref": false,
        "name": {
          "leadings": [],
          "trailings": [],
          "node_type": "identifier",
          "name": "hello_world"
        },
        "parameters": [
          {
            "leadings": [],
            "trailings": [],
            "node_type": "parameter",
            "is_ref": false,
            "is_ellipsis": false,
            "name": {
              "leadings": [],
              "trailings": [],
              "node_type": "identifier",
              "name": "foo"
            }
          }
        ],
        "body": {
          "leadings": [],
          "trailings": [],
          "node_type": "block",
          "statements": [
            {
              "leadings": [],
              "trailings": [],
              "node_type": "call",
              "name": {
                "leadings": [],
                "trailings": [],
                "node_type": "identifier",
                "name": "var_dump"
              },
              "arguments": [
                {
                  "leadings": [],
                  "trailings": [],
                  "node_type": "call_argument",
                  "value": {
                    "leadings": [],
                    "trailings": [],
                    "node_type": "variable",
                    "name": {
                      "leadings": [],
                      "trailings": [],
                      "node_type": "identifier",
                      "name": "foo"
                    }
                  }
                }
              ]
            }
          ]
        }
      }
    ]

### generate

    const { parse, generate } = require("../dist");

    const code = `<?php
    function hello_world($foo) {
      var_dump($foo);
    }`;

    const parsed = parse(code);
    const generated = generate(parsed);
    console.log(generated);

Resulting this string:

    function hello_world($foo) {
      var_dump($foo);
    }

## heavily inspired by

- [nikic/PHP-Parser](https://github.com/nikic/PHP-Parser)
- [glayzzle/php-parser](https://github.com/glayzzle/php-parser)
