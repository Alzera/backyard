const fs = require("fs-extra");
const path = require("path");
const { exec } = require("child_process");

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
  let imports = "";
  let builders = "";

  try {
    const items = await fs.readdir(directory, { withFileTypes: true });
    const files = items.filter((file) => file.isFile());

    for (const file of files) {
      const filename = path.basename(file.name, ".ts");
      const file_name = filename
        .replace("Node", "")
        .replace(/([A-Z])/g, "_$1")
        .toLowerCase()
        .replace(/^_/, "");
      imports += `import type { ${filename} } from "./nodes/${file.name}";\n`;
      if (!["NodeType", "Node"].includes(filename)) {
        builders += `${file_name}: (args: NodeBase & ${filename}): Node => build("${file_name}", args),\n`;
      }
    }

    const builder = `${imports}
  type NodeBase = {
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
    ${builders}
  };
  export { builder };`;
    fs.writeFileSync("./dist/builder.ts", builder);

    exec("tsc dist/builder.ts --declaration --module commonjs", async () => {
      fs.removeSync("./dist/builder.ts");
    });
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
  `const { lex, lex_eval, parse, parse_eval, generate } = require("./backyard.js");
const { builder } = require("./builder.js");

module.exports = {
  lex,
  lex_eval,
  parse,
  parse_eval,
  generate,
  builder,
};`
);
updatePackageJson();
