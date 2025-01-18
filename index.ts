import init, { format_rust_code } from "./pkg/rustfmt_wasm.js";

async function run() {
  await init();

  const code = `use std::io;
  use std::io::Write;
  use std::io::Read;
  use std::io::BufReader;
  use std::io::BufRead;
  use std::io::BufWriter;
  use std::io::Write;
  use std::io::Read;
    


  

  struct  Foo {
  bar: i32,
  baz: i32,
  }
  static MY: u32 = 10;

  fn main() {
  println!("Hello, world!");
  let x = 1 +    1;

  match x {
  1 => println!("one"),
  2 => println!("two"),
  3 => println!("three"),
  _ => println!("anything"),
  
  }
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
