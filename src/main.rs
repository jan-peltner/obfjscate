use std::fs::read_to_string;
use tree_sitter::{Parser, Tree, TreeCursor};

fn call_and_walk(crs: &mut TreeCursor, src: &str, cb: fn(&TreeCursor, &str) -> ()) {
    cb(crs, src);
    if crs.goto_first_child() {
        call_and_walk(crs, src, cb);
    }
    while crs.goto_next_sibling() {
        call_and_walk(crs, src, cb);
    }
    crs.goto_parent();
}

fn print_callback(crs: &TreeCursor, src: &str) -> () {
    let node = crs.node();
    let indent = (crs.depth() * 2) as usize;
    println!(
        "{:indent$}node type: {} | depth: {} | grammar id: {} | content: {}",
        "",
        node.kind(),
        crs.depth(),
        node.grammar_id(),
        node.utf8_text(src.as_bytes())
            .expect("Couldn't decode node text content")
    );
}

fn main() -> Result<(), String> {
    let file_str = read_to_string("./js/simple.js").map_err(|_| "Could not read JS file")?;
    let js_grammar = tree_sitter_javascript::language();

    let mut parser = Parser::new();
    parser
        .set_language(&js_grammar)
        .map_err(|_| "Could not set JS grammar!")?;

    let tree: Tree = parser
        .parse(&file_str, None)
        .ok_or("Could not parse JS file!")?;
    let root = tree.root_node();
    let mut cursor = root.walk();

    call_and_walk(&mut cursor, &file_str, print_callback);
    Ok(())
}
