use tree_sitter::TreeCursor;

pub fn call_and_walk<CbArg>(
    crs: &mut TreeCursor,
    src: &str,
    cb: &mut dyn Fn(&TreeCursor, &str, &mut CbArg) -> (),
    cb_arg: &mut CbArg,
) {
    cb(crs, src, cb_arg);
    if crs.goto_first_child() {
        call_and_walk(crs, src, cb, cb_arg);
    }
    while crs.goto_next_sibling() {
        call_and_walk(crs, src, cb, cb_arg);
    }
    crs.goto_parent();
}
