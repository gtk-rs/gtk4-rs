fn main() {
    manage_docs();
}

#[cfg(all(
    any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &["lib.rs", "prelude.rs", "rt.rs"];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Gsk4, PATH, IGNORES);
    }
}

#[cfg(any(
    all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {}
