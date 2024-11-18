import { lex, parse, generate } from "./dist/backyard.js";

try {
  let tokens = lex("function a(int $x, int $y = 0): int {}");
  console.log("Tokens:", tokens);
  let nodes = parse("function a(int $x, int $y = 0): int {}");
  console.log("Nodes:", nodes);
  let gen = generate(nodes);
  console.log("Generate:", gen);
} catch (e) {
  console.log(e);
}
