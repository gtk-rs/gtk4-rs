#[cfg(doctest)]
mod doctests {
    // Book markdown checks
    doc_comment::doctest!("../../src/custom_widgets.md");
    doc_comment::doctest!("../../src/gobject_concepts.md");
    doc_comment::doctest!("../../src/gobject_memory_managment.md");
    doc_comment::doctest!("../../src/hello_world.md");
    doc_comment::doctest!("../../src/installation.md");
    doc_comment::doctest!("../../src/introduction.md");
    doc_comment::doctest!("../../src/main_event_loop.md");
    doc_comment::doctest!("../../src/widgets.md");
}
