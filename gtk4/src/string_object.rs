// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{GStr, GString};

use crate::StringObject;

impl From<StringObject> for String {
    #[inline]
    fn from(value: StringObject) -> Self {
        skip_assert_initialized!();
        Self::from(&value)
    }
}

impl From<StringObject> for GString {
    #[inline]
    fn from(value: StringObject) -> Self {
        skip_assert_initialized!();
        Self::from(&value)
    }
}

impl<'a> From<&'a StringObject> for String {
    #[inline]
    fn from(value: &'a StringObject) -> Self {
        skip_assert_initialized!();
        value.string().to_string()
    }
}

impl<'a> From<&'a StringObject> for GString {
    #[inline]
    fn from(value: &'a StringObject) -> Self {
        skip_assert_initialized!();
        value.string()
    }
}

impl From<String> for StringObject {
    #[inline]
    fn from(value: String) -> Self {
        skip_assert_initialized!();
        Self::new(&value)
    }
}

impl<'a> From<&'a String> for StringObject {
    #[inline]
    fn from(value: &'a String) -> Self {
        skip_assert_initialized!();
        Self::new(value)
    }
}

impl<'a> From<&'a GStr> for StringObject {
    #[inline]
    fn from(v: &'a GStr) -> Self {
        skip_assert_initialized!();
        Self::new(v)
    }
}

impl From<GString> for StringObject {
    #[inline]
    fn from(value: GString) -> Self {
        skip_assert_initialized!();
        Self::new(&value)
    }
}

impl<'a> From<&'a GString> for StringObject {
    #[inline]
    fn from(value: &'a GString) -> Self {
        skip_assert_initialized!();
        Self::new(value)
    }
}

impl From<&str> for StringObject {
    #[inline]
    fn from(value: &str) -> Self {
        skip_assert_initialized!();
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::StringObject;
    use crate as gtk4;

    #[test]
    fn from_into() {
        let obj = StringObject::new("some_str");
        assert_eq!(String::from(&obj), obj.string());
        assert_eq!(String::from(obj), "some_str");
    }
}
