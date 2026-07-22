import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const requiredFiles = [
  "index.html",
  "site/course.css",
  "site/course.js",
  "site/static-server.mjs",
  "site/vendor/mermaid/mermaid.min.js",
  "site/vendor/mermaid/LICENSE",
  "site/vendor/mermaid/VERSION",
];
const missingFiles = requiredFiles.filter((path) => !existsSync(path));
if (missingFiles.length) {
  throw new Error(`Missing site files: ${missingFiles.join(", ")}`);
}

const courseSource = readFileSync("site/course.js", "utf8");
new Function(courseSource);

const indexSource = readFileSync("index.html", "utf8");
if (!indexSource.includes("site/vendor/mermaid/mermaid.min.js")) {
  throw new Error("The Mermaid browser bundle is not loaded by index.html.");
}
if (!courseSource.includes("renderMermaidDiagrams")) {
  throw new Error("The course reader does not include Mermaid rendering support.");
}

const manifestPaths = new Set(
  [...courseSource.matchAll(/path: "([^"]+\.md)"/g)].map((match) => match[1].replaceAll("\\", "/")),
);

function markdownFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) return markdownFiles(path);
    return entry.name.endsWith(".md") ? [path.replaceAll("\\", "/")] : [];
  });
}

const sourcePaths = new Set(markdownFiles("course"));
const mermaidDiagramCount = [...sourcePaths].reduce(
  (total, path) => total + (readFileSync(path, "utf8").match(/```mermaid\b/g) || []).length,
  0,
);
if (!mermaidDiagramCount) throw new Error("No Mermaid diagrams were found to render.");
const missingFromManifest = [...sourcePaths].filter((path) => !manifestPaths.has(path));
const missingOnDisk = [...manifestPaths].filter((path) => !sourcePaths.has(path));

if (missingFromManifest.length || missingOnDisk.length) {
  const messages = [];
  if (missingFromManifest.length) messages.push(`Not in navigation: ${missingFromManifest.join(", ")}`);
  if (missingOnDisk.length) messages.push(`Missing on disk: ${missingOnDisk.join(", ")}`);
  throw new Error(messages.join("\n"));
}

console.log(
  `Course reader check passed with ${manifestPaths.size} Markdown pages and ${mermaidDiagramCount} Mermaid diagrams.`,
);
