// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::StringList;
use std::iter::FromIterator;

impl FromIterator<&'static str> for StringList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        assert_initialized_main_thread!();
        StringList::new(iter.into_iter().collect::<Vec<_>>().as_slice())
    }
}

impl Extend<&'static str> for StringList {
    fn extend<T: IntoIterator<Item = &'static str>>(&mut self, iter: T) {
        self.splice(
            self.n_items(),
            0,
            iter.into_iter().collect::<Vec<_>>().as_slice(),
        );
    }
}

impl FromIterator<String> for StringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        assert_initialized_main_thread!();
        let string_list = StringList::new(&[]);
        for s in iter {
            string_list.append(&s);
        }
        string_list
    }
}

impl Extend<String> for StringList {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for s in iter {
            self.append(&s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringList;
    use crate::test_synced;

    #[test]
    fn test_from_iter() {
        test_synced(move || {
            let strings = vec!["hello", "world", "stuff"]
                .into_iter()
                .collect::<StringList>();
            assert_eq!(strings.string(1).unwrap(), "world");

            let strings2 = vec!["hello".to_string(), "world".to_string()]
                .into_iter()
                .collect::<StringList>();

            assert_eq!(strings2.string(1).unwrap(), "world");
        });
    }

    #[test]
    fn test_extend() {
        test_synced(move || {
            let mut strings = vec!["hello", "world", "stuff"]
                .into_iter()
                .collect::<StringList>();
            strings.extend(["gtk-rs", "gtk4", "gnome"]);
            assert_eq!(strings.string(4).unwrap(), "gtk4");
        });
    }
}
