// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{
    prelude::*, Accessible, AccessibleAutocomplete, AccessibleInvalidState, AccessibleProperty,
    AccessibleRelation, AccessibleSort, AccessibleState, AccessibleTristate, Orientation,
};
use glib::{translate::*, Value};

// rustdoc-stripper-ignore-next
/// Trait containing manually implemented methods of [`Accessible`](crate::Accessible).
///
/// ```no_run
/// # use gtk4 as gtk;
/// # use gtk::prelude::*;
/// let entry = gtk::Entry::new();
/// let label = gtk::Label::new(Some("Entry"));
/// entry.update_property(&[
///     gtk::accessible::Property::Description("Test"),
///     gtk::accessible::Property::Orientation(gtk::Orientation::Horizontal),
/// ]);
/// entry.update_relation(&[
///     gtk::accessible::Relation::LabelledBy(&[label.upcast_ref()]),
/// ]);
/// entry.update_state(&[
///     gtk::accessible::State::Invalid(gtk::AccessibleInvalidState::Grammar),
/// ]);
/// ```
pub trait AccessibleExtManual {
    #[doc(alias = "gtk_accessible_update_property")]
    #[doc(alias = "gtk_accessible_update_property_value")]
    fn update_property(&self, properties: &[Property]);

    #[doc(alias = "gtk_accessible_update_relation")]
    #[doc(alias = "gtk_accessible_update_relation_value")]
    fn update_relation(&self, relations: &[Relation]);

    #[doc(alias = "gtk_accessible_update_state")]
    #[doc(alias = "gtk_accessible_update_state_value")]
    fn update_state(&self, states: &[State]);
}

impl<O: IsA<Accessible>> AccessibleExtManual for O {
    fn update_property(&self, properties: &[Property]) {
        let mut properties_ptr = vec![];
        let mut values = vec![];
        for prop in properties {
            let (p, v) = prop.to_property_value();
            properties_ptr.push(p.into_glib());
            values.push(v);
        }

        unsafe {
            ffi::gtk_accessible_update_property_value(
                self.as_ref().to_glib_none().0,
                properties.len() as i32,
                mut_override(properties_ptr.as_ptr()),
                ToGlibContainerFromSlice::to_glib_none_from_slice(&values).0,
            )
        }
    }

    fn update_relation(&self, relations: &[Relation]) {
        let mut relations_ptr = vec![];
        let mut values = vec![];
        for relation in relations {
            let (r, v) = relation.to_relation_value();
            relations_ptr.push(r.into_glib());
            values.push(v);
        }

        unsafe {
            ffi::gtk_accessible_update_relation_value(
                self.as_ref().to_glib_none().0,
                relations.len() as i32,
                mut_override(relations_ptr.as_ptr()),
                ToGlibContainerFromSlice::to_glib_none_from_slice(&values).0,
            )
        }
    }

    fn update_state(&self, states: &[State]) {
        let mut states_ptr = vec![];
        let mut values = vec![];
        for state in states {
            let (s, v) = state.to_state_value();
            states_ptr.push(s.into_glib());
            values.push(v);
        }

        unsafe {
            ffi::gtk_accessible_update_state_value(
                self.as_ref().to_glib_none().0,
                states.len() as i32,
                mut_override(states_ptr.as_ptr()),
                ToGlibContainerFromSlice::to_glib_none_from_slice(&values).0,
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// Type-safe enum container for [`AccessibleProperty`](crate::AccessibleProperty) values.
#[derive(Debug)]
#[non_exhaustive]
pub enum Property<'p> {
    Autocomplete(AccessibleAutocomplete),
    Description(&'p str),
    HasPopup(bool),
    KeyShortcuts(&'p str),
    Label(&'p str),
    Level(i32),
    Modal(bool),
    MultiLine(bool),
    MultiSelectable(bool),
    Orientation(Orientation),
    Placeholder(&'p str),
    ReadOnly(bool),
    Required(bool),
    RoleDescription(&'p str),
    Sort(AccessibleSort),
    ValueMax(f64),
    ValueMin(f64),
    ValueNow(f64),
    ValueText(&'p str),
}

impl<'p> Property<'p> {
    fn to_property_value(&self) -> (AccessibleProperty, Value) {
        use Property::*;

        match self {
            Autocomplete(v) => (AccessibleProperty::Autocomplete, v.into_glib().to_value()),
            Description(v) => (AccessibleProperty::Description, v.to_value()),
            HasPopup(v) => (AccessibleProperty::HasPopup, v.to_value()),
            KeyShortcuts(v) => (AccessibleProperty::KeyShortcuts, v.to_value()),
            Label(v) => (AccessibleProperty::Label, v.to_value()),
            Level(v) => (AccessibleProperty::Level, v.to_value()),
            Modal(v) => (AccessibleProperty::Modal, v.to_value()),
            MultiLine(v) => (AccessibleProperty::MultiLine, v.to_value()),
            MultiSelectable(v) => (AccessibleProperty::MultiSelectable, v.to_value()),
            Orientation(v) => (AccessibleProperty::Orientation, v.into_glib().to_value()),
            Placeholder(v) => (AccessibleProperty::Placeholder, v.to_value()),
            ReadOnly(v) => (AccessibleProperty::ReadOnly, v.to_value()),
            Required(v) => (AccessibleProperty::Required, v.to_value()),
            RoleDescription(v) => (AccessibleProperty::RoleDescription, v.to_value()),
            Sort(v) => (AccessibleProperty::Sort, v.into_glib().to_value()),
            ValueMax(v) => (AccessibleProperty::ValueMax, v.to_value()),
            ValueMin(v) => (AccessibleProperty::ValueMin, v.to_value()),
            ValueNow(v) => (AccessibleProperty::ValueNow, v.to_value()),
            ValueText(v) => (AccessibleProperty::ValueText, v.to_value()),
        }
    }
}

// rustdoc-stripper-ignore-next
/// Type-safe enum container for [`AccessibleRelation`](crate::AccessibleRelation) values.
#[derive(Debug)]
#[non_exhaustive]
pub enum Relation<'r> {
    ActiveDescendant(&'r Accessible),
    ColCount(i32),
    ColIndex(i32),
    ColIndexText(&'r str),
    ColSpan(i32),
    Controls(&'r [&'r Accessible]),
    DescribedBy(&'r [&'r Accessible]),
    Details(&'r [&'r Accessible]),
    ErrorMessage(&'r Accessible),
    FlowTo(&'r [&'r Accessible]),
    LabelledBy(&'r [&'r Accessible]),
    Owns(&'r [&'r Accessible]),
    PosInSet(i32),
    RowCount(i32),
    RowIndex(i32),
    RowIndexText(&'r str),
    RowSpan(i32),
    SetSize(i32),
}

impl<'r> Relation<'r> {
    fn to_relation_value(&self) -> (AccessibleRelation, Value) {
        use Relation::*;

        fn to_ref_list_value(objects: &[&Accessible]) -> Value {
            skip_assert_initialized!();
            let mut value = Value::from_type(glib::Type::POINTER);
            let list =
                ToGlibContainerFromSlice::<*mut glib::ffi::GList>::to_glib_container_from_slice(
                    objects,
                );
            unsafe {
                glib::gobject_ffi::g_value_set_pointer(
                    value.to_glib_none_mut().0,
                    list.0 as *mut std::ffi::c_void,
                );
            }
            value
        }

        match self {
            ActiveDescendant(v) => (AccessibleRelation::ActiveDescendant, v.to_value()),
            ColCount(v) => (AccessibleRelation::ColCount, v.to_value()),
            ColIndex(v) => (AccessibleRelation::ColIndex, v.to_value()),
            ColIndexText(v) => (AccessibleRelation::ColIndexText, v.to_value()),
            ColSpan(v) => (AccessibleRelation::ColSpan, v.to_value()),
            Controls(v) => (AccessibleRelation::Controls, to_ref_list_value(v)),
            DescribedBy(v) => (AccessibleRelation::DescribedBy, to_ref_list_value(v)),
            Details(v) => (AccessibleRelation::Details, to_ref_list_value(v)),
            ErrorMessage(v) => (AccessibleRelation::ErrorMessage, v.to_value()),
            FlowTo(v) => (AccessibleRelation::FlowTo, to_ref_list_value(v)),
            LabelledBy(v) => (AccessibleRelation::LabelledBy, to_ref_list_value(v)),
            Owns(v) => (AccessibleRelation::Owns, to_ref_list_value(v)),
            PosInSet(v) => (AccessibleRelation::PosInSet, v.to_value()),
            RowCount(v) => (AccessibleRelation::RowCount, v.to_value()),
            RowIndex(v) => (AccessibleRelation::RowIndex, v.to_value()),
            RowIndexText(v) => (AccessibleRelation::RowIndexText, v.to_value()),
            RowSpan(v) => (AccessibleRelation::RowSpan, v.to_value()),
            SetSize(v) => (AccessibleRelation::SetSize, v.to_value()),
        }
    }
}

// rustdoc-stripper-ignore-next
/// Type-safe enum container for [`AccessibleState`](crate::AccessibleState) values.
#[derive(Debug)]
#[non_exhaustive]
pub enum State {
    Busy(bool),
    Checked(AccessibleTristate),
    Disabled(bool),
    Expanded(Option<bool>),
    Hidden(bool),
    Invalid(AccessibleInvalidState),
    Pressed(AccessibleTristate),
    Selected(Option<bool>),
}

impl State {
    fn to_state_value(&self) -> (AccessibleState, Value) {
        use State::*;

        fn to_optional_bool_value(b: &Option<bool>) -> Value {
            skip_assert_initialized!();
            b.map(|b| b as i32).unwrap_or(-1).to_value()
        }

        match self {
            Busy(v) => (AccessibleState::Busy, v.to_value()),
            Checked(v) => (AccessibleState::Checked, v.into_glib().to_value()),
            Disabled(v) => (AccessibleState::Disabled, v.to_value()),
            Expanded(v) => (AccessibleState::Expanded, to_optional_bool_value(v)),
            Hidden(v) => (AccessibleState::Hidden, v.to_value()),
            Invalid(v) => (AccessibleState::Invalid, v.into_glib().to_value()),
            Pressed(v) => (AccessibleState::Pressed, v.into_glib().to_value()),
            Selected(v) => (AccessibleState::Selected, to_optional_bool_value(v)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{self as gtk4, Button};

    #[test]
    fn test_accessible_update_property() {
        let widget = glib::Object::new::<Button>();
        widget.update_property(&[
            Property::Autocomplete(AccessibleAutocomplete::Inline),
            Property::Description("Test"),
            Property::HasPopup(true),
            Property::KeyShortcuts("Test"),
            Property::Label("Test"),
            Property::Level(123),
            Property::Modal(true),
            Property::MultiLine(true),
            Property::MultiSelectable(true),
            Property::Orientation(Orientation::Horizontal),
            Property::Placeholder("Test"),
            Property::ReadOnly(true),
            Property::Required(true),
            Property::RoleDescription("Test"),
            Property::Sort(AccessibleSort::Ascending),
            Property::ValueMax(1.0),
            Property::ValueMin(1.0),
            Property::ValueNow(1.0),
            Property::ValueText("Test"),
        ]);
    }

    #[test]
    fn test_accessible_update_relation() {
        use crate::prelude::*;

        let widget = glib::Object::new::<Button>();
        let other1 = glib::Object::new::<Button>();
        let other2 = glib::Object::new::<Button>();
        widget.update_relation(&[
            Relation::ActiveDescendant(other1.upcast_ref()),
            Relation::ColCount(123),
            Relation::ColIndex(123),
            Relation::ColIndexText("Test"),
            Relation::ColSpan(123),
            Relation::Controls(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::DescribedBy(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::Details(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::ErrorMessage(other1.upcast_ref()),
            Relation::FlowTo(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::LabelledBy(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::Owns(&[other1.upcast_ref(), other2.upcast_ref()]),
            Relation::PosInSet(123),
            Relation::RowCount(123),
            Relation::RowIndex(123),
            Relation::RowIndexText("Test"),
            Relation::RowSpan(123),
            Relation::SetSize(123),
        ]);
    }

    #[test]
    fn test_accessible_update_state() {
        let widget = glib::Object::new::<Button>();
        widget.update_state(&[
            State::Busy(true),
            State::Checked(AccessibleTristate::Mixed),
            State::Disabled(true),
            State::Expanded(Some(true)),
            State::Hidden(true),
            State::Invalid(AccessibleInvalidState::Grammar),
            State::Pressed(AccessibleTristate::Mixed),
            State::Selected(Some(true)),
        ]);
    }
}
