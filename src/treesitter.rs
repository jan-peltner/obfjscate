use std::fs::read_to_string;
use tree_sitter::{Parser, Tree};

pub fn setup_parser() -> Result<(String, Tree), String> {
    let file_str = read_to_string("./js/simple.js").map_err(|_| "Could not read JS file")?;
    let js_grammar = tree_sitter_javascript::language();

    let mut parser = Parser::new();
    parser
        .set_language(&js_grammar)
        .map_err(|_| "Could not set JS grammar!")?;

    let tree: Tree = parser
        .parse(&file_str, None)
        .ok_or("Could not parse JS file!")?;
    Ok((file_str, tree))
}
