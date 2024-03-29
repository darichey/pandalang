use pandalang_parser::ast::Program;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn parse(source: &str) -> Result<String, String> {
    let ast = parse_(source)?;
    Ok(format!("{:#?}", ast))
}

#[wasm_bindgen]
pub fn typecheck(source: &str) -> Result<String, String> {
    let ast = parse_(source)?;
    let types = pandalang_types::check_prog_to_strings(ast).map_err(|err| err.to_string())?;
    Ok(format!("{:#?}", types))
}

#[wasm_bindgen]
pub fn run(source: &str) -> Result<String, String> {
    let ast = parse_(source)?;
    pandalang_types::check_prog_to_strings(ast.clone()).map_err(|err| err.to_string())?;
    let mut stdout = Vec::new();
    pandalang_eval::run_program(ast, &mut stdout)?;
    Ok(String::from_utf8_lossy(&stdout).to_string())
}

fn parse_(source: &str) -> Result<Program, String> {
    pandalang_parser::parse(source).map_err(|err| err.to_string())
}
