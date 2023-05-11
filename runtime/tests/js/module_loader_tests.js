import { assertEquals, assertMatch, assertRejects } from "./vendored/asserts.bundle.js";
import { test } from "./helpers.js";

test("dynamically import file next to the main module file", async () => {
  const { KEY } = await import("./empty_module.js");
  assertEquals(KEY, 1);
});

test("statically import a file inside the module tree", async () => {
  // lib contains `import` from `./log.js`, check that it's allowed
  // also check that we can import from nested directories
  await import("./module_fixtures/lib.js");
});

test("cannot import files outside the main module directory", async () => {
  let err = await assertRejects(() => import("../../js/99_main.js"));
  assertMatch(err.message, /Cannot import files outside of module root directory/);
});

test("cannot import files over http", async () => {
  let err = await assertRejects(() => import("https://deno.land/std@0.181.0/version.ts"));
  assertMatch(err.message, /Zinnia supports importing from relative paths only/);
});
