use core::ops::Range;
use tree_sitter::TreeCursor;

pub fn print_nodes(crs: &TreeCursor, src: &str, _: &mut ()) -> () {
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

pub fn collect_identifiers(
    crs: &TreeCursor,
    src: &str,
    identifiers: &mut Vec<(String, Range<usize>)>,
) {
    let node = crs.node();
    if node.kind() == "identifier" {
        identifiers.push((
            node.utf8_text(src.as_bytes()).unwrap().to_string(),
            node.byte_range(),
        ));
    }
}
