#[cfg(target_os = "macos")]
extern crate cc;

fn main() {
    manage_docs();
    #[cfg(target_os = "macos")]
    build_foreground();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs () {
    extern crate lgpl_docs;
    const PATH: &'static str = "src";
    const IGNORES: &'static [&'static str] = &[
        "lib.rs",
        "prelude.rs",
        "rt.rs",
        "signal.rs",
    ];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Gtk4, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() { }

#[cfg(target_os = "macos")]
fn build_foreground() {
    cc::Build::new().file("src/foreground.m").compile("foreground");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
