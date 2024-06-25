mod callbacks;
mod search;
mod treesitter;

use callbacks::*;
use search::*;
use treesitter::*;

fn main() -> Result<(), String> {
    if let Ok((file_str, tree)) = read_and_parse_file() {
        // Get the tree cursor starting from the root node
        let mut cursor = tree.root_node().walk();

        // Example 1: Printing nodes
        call_and_walk(&mut cursor, &file_str, &mut print_nodes, &mut ());

        // Example 2: Collecting identifiers
        let mut identifiers: Vec<(String, core::ops::Range<usize>)> = Vec::new();
        call_and_walk(
            &mut cursor,
            &file_str,
            &mut collect_identifiers,
            &mut identifiers,
        );
        println!("Identifiers found: {:?}", identifiers);
    };

    Ok(())
}
