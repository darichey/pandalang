mod console;

use std::io::Write;

use console::console;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(source: &str) -> Result<String, String> {
    let ast =
        pandalang_parser::parse(source).map_err(|err| format!("parse: {}", { err.to_string() }))?;
    pandalang_types::check_prog_to_strings(ast.clone())
        .map_err(|err| format!("check: {}", { err.to_string() }))?;
    let mut stdout = console();
    let output = pandalang_eval::run_program(ast, &mut stdout)?;
    stdout
        .flush()
        .map_err(|err| format!("flush: {}", { err.to_string() }))?;
    Ok(format!("{:#?}", output))
}
