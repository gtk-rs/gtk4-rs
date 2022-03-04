// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
fn failures() {
    let t = trybuild2::TestCases::new();
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-duplicate-id-attr.rs",
        "error: two instances of the same attribute argument, each argument must be specified only once",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/template-callback-arg-after-rest.rs",
        "error: Arguments past argument with `rest` attribute",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/template-callback-duplicate-rest.rs",
        "error: Duplicate `rest` attribute",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/template-callback-duplicate.rs",
        "error: Duplicate `template_callback` attribute",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/template-callback-mut-self.rs",
        "error: Receiver must be `&self`",
    );
}
