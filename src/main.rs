use std::fs::read_to_string;
use tree_sitter::{Parser, Tree, TreeCursor};

fn print_and_dip(crs: &mut TreeCursor, src: &str) {
    let node = crs.node();
    let depth = (crs.depth() * 2) as usize;
    println!(
        "{:depth$}node type: {}\n{:depth$}depth: {}\n{:depth$}content: {}",
        "",
        node.kind(),
        "",
        crs.depth(),
        "",
        node.utf8_text(src.as_bytes())
            .expect("Couldn't decode node text content")
    );
    if crs.goto_first_child() {
        print_and_dip(crs, src);
    }
    while crs.goto_next_sibling() {
        print_and_dip(crs, src);
    }
    crs.goto_parent();
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
    print_and_dip(&mut cursor, &file_str);
    Ok(())
}
