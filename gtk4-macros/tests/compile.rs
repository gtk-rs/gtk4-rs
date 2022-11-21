// Take a look at the license at the top of the repository in the LICENSE file.

#[test]
fn failures() {
    let t = trybuild2::TestCases::new();
    #[cfg(feature = "xml_validation")]
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-bad-xml.rs",
        "error: Failed reading template XML",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-duplicate-id-attr.rs",
        "error: two instances of the same attribute argument, each argument must be specified only once",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-missing-child-attr.rs",
        "error: field `label` with type `TemplateChild` possibly missing #[template_child] attribute",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-missing-fq-child-attr.rs",
        "error: field `label` with type `TemplateChild` possibly missing #[template_child] attribute",
    );
    #[cfg(feature = "xml_validation")]
    t.compile_fail_check_sub(
        "tests/compile-fail/composite-template-missing-id.rs",
        "error: Template child with id `label` not found in template XML",
    );
    t.compile_fail_check_sub(
        "tests/compile-fail/template-callback-async-ret-value.rs",
        "`async` only allowed on template callbacks without a return value",
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
        "error: Receiver cannot be a mutable reference",
    );
}
