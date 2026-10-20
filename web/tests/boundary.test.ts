// @vitest-environment node
/**
 * The web app's boundary. Code in web/src imports only files inside web/src and the packages in
 * the "dependencies" of package.json (test files may also use "devDependencies"). It never
 * imports Node built-ins or anything outside the web app, such as server code: it reaches the
 * server only over HTTP.
 */
import { readFileSync, readdirSync } from "node:fs";
import { builtinModules } from "node:module";
import { dirname, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import ts from "typescript";
import { expect, test } from "vitest";

const webDir = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const srcDir = join(webDir, "src");
const manifest = JSON.parse(
  readFileSync(join(webDir, "package.json"), "utf8"),
) as {
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
};
const runtimePackages = new Set(Object.keys(manifest.dependencies ?? {}));
const devPackages = new Set(Object.keys(manifest.devDependencies ?? {}));
const nodeBuiltins = new Set(builtinModules);

const sourcePattern = /\.[cm]?[jt]sx?$/;
const testPattern = /\.test\.[cm]?[jt]sx?$/;

function packageName(specifier: string): string {
  const parts = specifier.split("/");
  return specifier.startsWith("@")
    ? parts.slice(0, 2).join("/")
    : (parts[0] ?? "");
}

/** The imports in `source` (the file at `file`) that cross the boundary. */
function violations(file: string, source: string): string[] {
  const imports = ts
    .preProcessFile(source, true, true)
    .importedFiles.map((imported) => imported.fileName);
  return imports.flatMap((specifier) => {
    if (specifier.startsWith(".")) {
      const target = resolve(dirname(file), specifier);
      return target === srcDir || target.startsWith(srcDir + sep)
        ? []
        : [`imports "${specifier}", which is outside web/src`];
    }
    if (specifier.startsWith("/")) {
      return [`imports "${specifier}" by absolute path`];
    }
    const name = packageName(specifier);
    if (specifier.startsWith("node:") || nodeBuiltins.has(name)) {
      return [`imports the Node built-in "${specifier}"`];
    }
    if (
      runtimePackages.has(name) ||
      (testPattern.test(file) && devPackages.has(name))
    ) {
      return [];
    }
    return [
      `imports "${specifier}", which is not in the dependencies of package.json`,
    ];
  });
}

test("web/src imports only web/src and its declared packages", () => {
  const files = readdirSync(srcDir, { recursive: true, encoding: "utf8" })
    .filter((name) => sourcePattern.test(name))
    .map((name) => join(srcDir, name));
  expect(files.length).toBeGreaterThan(0);

  const found = files.flatMap((file) =>
    violations(file, readFileSync(file, "utf8")).map(
      (violation) => `${relative(webDir, file)}: ${violation}`,
    ),
  );
  expect(found).toEqual([]);
});

test("the boundary check catches imports that cross it", () => {
  const file = join(srcDir, "tasks", "list.ts");
  const crossing = [
    'import main from "../../../crates/server/src/main.rs";',
    'const api = await import("../../server/api.ts");',
    'import { readFileSync } from "node:fs";',
    'import fs from "fs";',
    'import { test } from "vitest";',
    'import logo from "/logo.svg";',
  ];
  for (const source of crossing) {
    expect({ source, found: violations(file, source).length }).toEqual({
      source,
      found: 1,
    });
  }

  expect(violations(file, 'import { App } from "../App.tsx";')).toEqual([]);
  expect(violations(file, 'import { useState } from "react";')).toEqual([]);
  expect(
    violations(join(srcDir, "list.test.ts"), 'import { test } from "vitest";'),
  ).toEqual([]);
});
