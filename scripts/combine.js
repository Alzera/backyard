const fs = require("fs-extra");
const path = require("path");

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

// async function processDts() {
//   let content = `// Auto-generated index.d.ts file\n\n`;

//   for (const dir of ["./nodes", "./token"]) {
//     const dirPath = path.join("./dist", dir);
//     if (await fs.pathExists(dirPath)) {
//       const files = await fs.readdir(dirPath);

//       files.forEach((file) => {
//         const fileExt = path.extname(file);

//         if (fileExt === ".ts") {
//           const oldFilePath = path.join(dirPath, file);
//           const fileNameWithoutExt = path.basename(file, ".ts");
//           const newFilePath = path.join(dirPath, `${fileNameWithoutExt}.d.ts`);
//           fs.renameSync(oldFilePath, newFilePath);

//           const filePath = path.join(dir, `${fileNameWithoutExt}.d.ts`);
//           content += `export * from "./${filePath}";\n`;
//         }
//       });
//     } else {
//       console.log(`Directory ${dirPath} does not exist.`);
//     }
//   }

//   await fs.writeFile("./dist/index.d.ts", content);
//   console.log(`index.d.ts has been generated`);
// }

// processDts();
