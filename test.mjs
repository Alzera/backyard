import { lex, parse, generate, builder as b } from "./dist/index.js";

try {
  let tokens = lex("function a(int $x, int $y = 0): int {}");
  console.log("Tokens:", tokens);
  let nodes = parse("function a(int $x, int $y = 0): int {}");
  console.log("Nodes:", nodes);
  let gen = generate(nodes);
  console.log("Generate:", gen);

  let newNodes = b.program({
    children: [
      b.assignment({
        left: b.variable({ name: b.identifier({ name: "x" }), is_ref: false }),
        operator: "=",
        right: b.number({ value: "1" }),
      }),
    ],
  });
  let gen2 = generate([newNodes]);
  console.log("Generate2:", gen2);
} catch (e) {
  console.log(e);
}
