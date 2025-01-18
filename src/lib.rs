use prettyplease::unparse;
use syn::File;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format_rust_code(code: &str) -> Result<String, JsValue> {
    // Parse the input code into a syntax tree
    let syntax_tree: File =
        syn::parse_str(code).map_err(|e| JsValue::from_str(&format!("Parsing error: {}", e)))?;

    // Format the code using prettyplease
    let formatted = unparse(&syntax_tree);

    let mut result = formatted;

    result = format_imports_and_modules(&result);

    result = add_spaces_after_blocks_expr(&result);

    Ok(result)
}

fn add_spaces_after_blocks_expr(code: &str) -> String {
    let mut result = String::new();
    let mut lines = code.lines().peekable();

    while let Some(line) = lines.next() {
        result.push_str(line);
        result.push('\n');

        // Check if the current line ends with a closing brace `}`
        if line.trim() == "}" {
            // Add an extra newline after the closing brace
            result.push('\n');
        }
    }

    result
}

fn format_imports_and_modules(code: &str) -> String {
    let mut result = String::new();
    let mut imports = Vec::new();
    let mut other_code = Vec::new();

    for line in code.lines() {
        if line.trim().starts_with("use ") || line.trim().starts_with("mod ") {
            imports.push(line.trim());
        } else {
            other_code.push(line);
        }
    }

    // Sort and format imports
    imports.sort();
    for import in imports {
        result.push_str(&format!("{}\n", import));
    }

    result.push_str(&format!("\n\n"));

    // Add the rest of the code
    for line in other_code {
        result.push_str(line);
        result.push('\n');
    }

    result
}
