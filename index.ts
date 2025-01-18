import init, { format_rust_code } from "./pkg/rustfmt_wasm.js";

async function run() {
  await init();

  const code = `
  struct  Foo {
  bar: i32,
  baz: i32,
  }
  fn main() {
  println!("Hello, world!");
              }
  `;

  try {
    const formattedCode = format_rust_code(code);
    console.log("Formatted code:\n", formattedCode);
  } catch (e) {
    console.error("Error formatting code:", e);
  }
}

run();
