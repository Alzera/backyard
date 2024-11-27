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

    const { parse } = require("../dist");

    const code = `<?php
    function hello_world($foo) {
      var_dump($foo);
    }`;

    const parsed = parse(code);
    console.log(JSON.stringify(parsed, null, 2));
