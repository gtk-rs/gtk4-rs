use gtk::gio;

fn main() {
    // composite_templates
    gio::compile_resources(
        "composite_templates/1/resources",
        "composite_templates/1/resources/resources.gresource.xml",
        "composite_templates_1.gresource",
    );
    // ANCHOR: composite_templates_2
    gio::compile_resources(
        "composite_templates/2/resources",
        "composite_templates/2/resources/resources.gresource.xml",
        "composite_templates_2.gresource",
    );
    // ANCHOR_END: composite_templates_2
    gio::compile_resources(
        "composite_templates/3/resources",
        "composite_templates/3/resources/resources.gresource.xml",
        "composite_templates_3.gresource",
    );
    gio::compile_resources(
        "composite_templates/4/resources",
        "composite_templates/4/resources/resources.gresource.xml",
        "composite_templates_4.gresource",
    );
    gio::compile_resources(
        "composite_templates/5/resources",
        "composite_templates/5/resources/resources.gresource.xml",
        "composite_templates_5.gresource",
    );
    gio::compile_resources(
        "composite_templates/6/resources",
        "composite_templates/6/resources/resources.gresource.xml",
        "composite_templates_6.gresource",
    );
}
