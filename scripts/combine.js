const fs = require("fs-extra");
const path = require("path");
const { execSync } = require("child_process");

async function renameTsToDts(directory) {
  try {
    const items = await fs.readdir(directory, { withFileTypes: true });

    for (const item of items) {
      const fullPath = path.join(directory, item.name);

      if (item.isDirectory()) {
        await renameTsToDts(fullPath);
      } else if (
        item.isFile() &&
        item.name.endsWith(".ts") &&
        !item.name.endsWith(".d.ts")
      ) {
        const newPath = path.join(
          directory,
          item.name.replace(/\.ts$/, ".d.ts")
        );
        await fs.rename(fullPath, newPath);
      }
    }
  } catch (err) {
    console.error("Error during renaming:", err);
  }
}

async function createBuilder(directory) {
  const builder = `type NodeBase = {
  leading_comments?: Array<Node>;
  trailing_comments?: Array<Node>;
};
const build = (node_type: string, args: NodeBase & any): Node => {
  const { leading_comments, trailing_comments, ...rest } = args;
  return {
    node_type,
    leading_comments: leading_comments ?? [],
    trailing_comments: trailing_comments ?? [],
    ...rest,
  };
};
const builder = {
  anonymous_function: (args: NodeBase & AnonymousFunctionNode): Node => build("anonymous_function", args),
  argument: (args: NodeBase & ArgumentNode): Node => build("argument", args),
  array: (args: NodeBase & ArrayItemNode): Node => build("array", args),
  array_item: (args: NodeBase & ArrayLookupNode): Node => build("array_item", args),
  array_lookup: (args: NodeBase & ArrayNode): Node => build("array_lookup", args),
  arrow_function: (args: NodeBase & ArrowFunctionNode): Node => build("arrow_function", args),
  assignment: (args: NodeBase & AssignmentNode): Node => build("assignment", args),
  bin: (args: NodeBase & BinNode): Node => build("bin", args),
  block: (args: NodeBase & BlockNode): Node => build("block", args),
  break: (args: NodeBase & BreakNode): Node => build("break", args),
  call: (args: NodeBase & CallNode): Node => build("call", args),
  case: (args: NodeBase & CaseNode): Node => build("case", args),
  cast: (args: NodeBase & CastNode): Node => build("cast", args),
  catch: (args: NodeBase & CatchNode): Node => build("catch", args),
  class: (args: NodeBase & ClassNode): Node => build("class", args),
  clone: (args: NodeBase & CloneNode): Node => build("clone", args),
  comment_block: (args: NodeBase & CommentBlockNode): Node => build("comment_block", args),
  comment_doc: (args: NodeBase & CommentDocNode): Node => build("comment_doc", args),
  comment_line: (args: NodeBase & CommentLineNode): Node => build("comment_line", args),
  const: (args: NodeBase & ConstNode): Node => build("const", args),
  const_property: (args: NodeBase & ConstPropertyNode): Node => build("const_property", args),
  continue: (args: NodeBase & ContinueNode): Node => build("continue", args),
  declare: (args: NodeBase & DeclareArgumentNode): Node => build("declare", args),
  declare_argument: (args: NodeBase & DeclareNode): Node => build("declare_argument", args),
  do_while: (args: NodeBase & DoWhileNode): Node => build("do_while", args),
  echo: (args: NodeBase & EchoNode): Node => build("echo", args),
  encapsed: (args: NodeBase & EncapsedNode): Node => build("encapsed", args),
  encapsed_part: (args: NodeBase & EncapsedPartNode): Node => build("encapsed_part", args),
  enum: (args: NodeBase & EnumItemNode): Node => build("enum", args),
  enum_item: (args: NodeBase & EnumNode): Node => build("enum_item", args),
  eval: (args: NodeBase & EvalNode): Node => build("eval", args),
  exit: (args: NodeBase & ExitNode): Node => build("exit", args),
  for: (args: NodeBase & ForNode): Node => build("for", args),
  foreach: (args: NodeBase & ForeachNode): Node => build("foreach", args),
  function: (args: NodeBase & FunctionNode): Node => build("function", args),
  global: (args: NodeBase & GlobalNode): Node => build("global", args),
  goto: (args: NodeBase & GotoNode): Node => build("goto", args),
  identifier: (args: NodeBase & IdentifierNode): Node => build("identifier", args),
  if: (args: NodeBase & IfNode): Node => build("if", args),
  include: (args: NodeBase & IncludeNode): Node => build("include", args),
  instance_of: (args: NodeBase & InstanceOfNode): Node => build("instance_of", args),
  interface: (args: NodeBase & InterfaceNode): Node => build("interface", args),
  label: (args: NodeBase & LabelNode): Node => build("label", args),
  list: (args: NodeBase & ListNode): Node => build("list", args),
  magic: (args: NodeBase & MagicNode): Node => build("magic", args),
  match: (args: NodeBase & MatchArmNode): Node => build("match", args),
  match_arm: (args: NodeBase & MatchNode): Node => build("match_arm", args),
  method: (args: NodeBase & MethodNode): Node => build("method", args),
  namespace: (args: NodeBase & NamespaceNode): Node => build("namespace", args),
  new: (args: NodeBase & NewNode): Node => build("new", args),
  number: (args: NodeBase & NumberNode): Node => build("number", args),
  object_access: (args: NodeBase & ObjectAccessNode): Node => build("object_access", args),
  parameter: (args: NodeBase & ParameterNode): Node => build("parameter", args),
  parent: (args: NodeBase & ParentNode): Node => build("parent", args),
  parenthesis: (args: NodeBase & ParenthesisNode): Node => build("parenthesis", args),
  post: (args: NodeBase & PostNode): Node => build("post", args),
  pre: (args: NodeBase & PreNode): Node => build("pre", args),
  print: (args: NodeBase & PrintNode): Node => build("print", args),
  program: (args: NodeBase & ProgramNode): Node => build("program", args),
  property: (args: NodeBase & PropertyItemNode): Node => build("property", args),
  property_item: (args: NodeBase & PropertyNode): Node => build("property_item", args),
  return: (args: NodeBase & ReturnNode): Node => build("return", args),
  static: (args: NodeBase & StaticLookupNode): Node => build("static", args),
  static_lookup: (args: NodeBase & StaticNode): Node => build("static_lookup", args),
  string: (args: NodeBase & StringNode): Node => build("string", args),
  switch: (args: NodeBase & SwitchNode): Node => build("switch", args),
  ternary: (args: NodeBase & TernaryNode): Node => build("ternary", args),
  trait: (args: NodeBase & ThrowNode): Node => build("trait", args),
  trait_use: (args: NodeBase & TraitNode): Node => build("trait_use", args),
  trait_use_alias: (args: NodeBase & TraitUseAliasNode): Node => build("trait_use_alias", args),
  trait_use_precedence: (args: NodeBase & TraitUseNode): Node => build("trait_use_precedence", args),
  throw: (args: NodeBase & TraitUsePrecedenceNode): Node => build("throw", args),
  try: (args: NodeBase & TryNode): Node => build("try", args),
  type: (args: NodeBase & TypeNode): Node => build("type", args),
  use: (args: NodeBase & UseNode): Node => build("use", args),
  variable: (args: NodeBase & VariableNode): Node => build("variable", args),
  while: (args: NodeBase & WhileNode): Node => build("while", args),
  yield: (args: NodeBase & YieldFromNode): Node => build("yield", args),
  yield_from: (args: NodeBase & YieldNode): Node => build("yield_from", args),
};
export { builder };`;
  let imports = "";

  try {
    const items = await fs.readdir(directory, { withFileTypes: true });
    const files = items.filter((file) => file.isFile());

    for (const file of files) {
      const imp = `import type { ${path.basename(
        file.name,
        ".ts"
      )} } from "./nodes/${file.name}";`;
      imports += imp + "\n";
    }
    fs.writeFileSync("./dist/builder.ts", imports + "\n" + builder);

    execSync("tsc dist/builder.ts --declaration --module commonjs");

    fs.removeSync("./dist/builder.ts");
  } catch (err) {
    console.error("Error during creating builder:", err);
  }
}

async function updatePackageJson() {
  try {
    const packageJsonPath = "./dist/package.json";

    const packageJson = JSON.parse(await fs.readFile(packageJsonPath, "utf8"));

    const updatedPackageJson = {
      name: packageJson.name,
      version: packageJson.version,
      main: "index.js",
    };

    await fs.promises.writeFile(
      packageJsonPath,
      JSON.stringify(updatedPackageJson, null, 2),
      "utf8"
    );

    console.log("package.json updated successfully.");
  } catch (error) {
    console.error("Error updating package.json:", error);
  }
}

fs.removeSync("./dist");
fs.moveSync("./crates/backyard/pkg", "./dist");
fs.moveSync("./crates/backyard-lexer/bindings", "./dist/token");
fs.moveSync("./crates/backyard-nodes/bindings", "./dist/nodes");

let dts = fs.readFileSync("./dist/backyard.d.ts", "utf-8");
dts =
  `import type { Token } from "./token/Token";
import type { Node } from "./nodes/Node";
` + dts;
fs.writeFile("./dist/backyard.d.ts", dts, "utf-8");

renameTsToDts("./dist");
createBuilder("./dist/nodes");

fs.writeFileSync(
  "./dist/index.js",
  `const { lex, parse, generate } = require("./backyard.js");
const { builder } = require("./builder.js");

module.exports = {
    lex,
    parse,
    generate,
    builder,
};`
);
updatePackageJson();
