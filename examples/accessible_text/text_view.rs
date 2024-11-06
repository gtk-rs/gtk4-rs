use gtk::glib;
use gtk::subclass::prelude::*;

mod imp {
    use gtk::{graphene, ACCESSIBLE_ATTRIBUTE_OVERLINE, ACCESSIBLE_ATTRIBUTE_OVERLINE_SINGLE};

    use super::*;

    #[derive(Default)]
    pub struct AccessibleTextView {}

    #[glib::object_subclass]
    impl ObjectSubclass for AccessibleTextView {
        const NAME: &'static str = "AccessibleTextView";
        type Type = super::AccessibleTextView;
        type ParentType = gtk::TextView;
        type Interfaces = (gtk::AccessibleText,);
    }

    impl ObjectImpl for AccessibleTextView {}
    impl WidgetImpl for AccessibleTextView {}
    impl AccessibleTextImpl for AccessibleTextView {
        fn attributes(
            &self,
            offset: u32,
        ) -> Vec<(gtk::AccessibleTextRange, glib::GString, glib::GString)> {
            let attributes = self.parent_attributes(offset);
            println!("attributes({offset}) -> {attributes:?}");
            attributes
        }

        fn caret_position(&self) -> u32 {
            let pos = self.parent_caret_position();
            println!("caret_position() -> {pos}");
            pos
        }

        fn contents(&self, start: u32, end: u32) -> Option<glib::Bytes> {
            let content = self.parent_contents(start, end);
            println!(
                "contents({start}, {end}) -> {:?}",
                content
                    .as_ref()
                    .map(|c| std::str::from_utf8(c.as_ref()).unwrap())
            );
            content
        }

        fn contents_at(
            &self,
            offset: u32,
            granularity: gtk::AccessibleTextGranularity,
        ) -> Option<(u32, u32, glib::Bytes)> {
            let contents = self.parent_contents_at(offset, granularity);
            println!(
                "contents_at offset({offset}, {granularity:?}) -> {:?}",
                contents
                    .as_ref()
                    .map(|(s, e, c)| (s, e, std::str::from_utf8(c.as_ref()).unwrap()))
            );
            contents
        }

        fn default_attributes(&self) -> Vec<(glib::GString, glib::GString)> {
            let mut attrs = self.parent_default_attributes();

            // Attributes can be added and removed
            attrs.push((
                ACCESSIBLE_ATTRIBUTE_OVERLINE.to_owned(),
                ACCESSIBLE_ATTRIBUTE_OVERLINE_SINGLE.to_owned(),
            ));
            println!("default_attributes() -> {attrs:?}");
            attrs
        }

        fn selection(&self) -> Vec<gtk::AccessibleTextRange> {
            let selection = self.parent_selection();
            println!("selection() -> {selection:?}");
            selection
        }

        fn extents(&self, start: u32, end: u32) -> Option<graphene::Rect> {
            let extents = self.parent_extents(start, end);
            println!("extents({start}, {end}) -> {extents:?}");
            extents
        }

        fn offset(&self, point: &graphene::Point) -> Option<u32> {
            let offset = self.parent_offset(point);
            println!("offset({:?}) -> {offset:?}", point);
            offset
        }
    }

    impl TextViewImpl for AccessibleTextView {}
}

glib::wrapper! {
    pub struct AccessibleTextView(ObjectSubclass<imp::AccessibleTextView>)
    @extends gtk::Widget, gtk::TextView,
    @implements gtk::Accessible, gtk::AccessibleText, gtk::Buildable, gtk::ConstraintTarget, gtk::Scrollable;
}
