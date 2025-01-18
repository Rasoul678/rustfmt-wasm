use wasm_bindgen::prelude::*;
use prettyplease::unparse;
use syn::File;

#[wasm_bindgen]
pub fn format_rust_code(code: &str) -> Result<String, JsValue> {
    // Parse the input code into a syntax tree
    let syntax_tree: File = syn::parse_str(code)
        .map_err(|e| JsValue::from_str(&format!("Parsing error: {}", e)))?;

    // Format the code using prettyplease
    let formatted_code = unparse(&syntax_tree);

    Ok(formatted_code)
}