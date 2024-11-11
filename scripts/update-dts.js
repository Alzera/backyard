const fs = require("fs");

let indexDTS = fs.readFileSync("./index.d.ts", "utf8");

indexDTS = indexDTS.replace(
  /(export declare class \w+Node) {/g,
  "$1 extends Node {"
);

indexDTS = indexDTS.replaceAll(
  `
  leadingComments: Nodes
  trailingComments: Nodes`,
  ""
);

indexDTS += `
export type Node = {
  leadingComments: Nodes;
  trailingComments: Nodes;
};
export type Nodes = Array<Node>;`;

fs.writeFileSync("./index.d.ts", indexDTS);
