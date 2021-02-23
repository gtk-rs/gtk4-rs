#[cfg(doctest)]
mod doctests {
    // Book markdown checks
    doc_comment::doctest!("../../src/basics/introduction.md");
    doc_comment::doctest!("../../src/basics/installation.md");
    doc_comment::doctest!("../../src/basics/hello_world.md");
    doc_comment::doctest!("../../src/basics/getting_started.md");
    doc_comment::doctest!("../../src/basics/widgets.md");
    doc_comment::doctest!("../../src/basics/gobject.md");
    doc_comment::doctest!("../../src/basics/main_loop.md");
}
