// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Ordering;
use std::cmp;

impl From<cmp::Ordering> for Ordering {
    fn from(o: cmp::Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            cmp::Ordering::Equal => Ordering::Equal,
            cmp::Ordering::Greater => Ordering::Larger,
            cmp::Ordering::Less => Ordering::Smaller,
        }
    }
}

impl From<Ordering> for cmp::Ordering {
    fn from(o: Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            Ordering::Equal => cmp::Ordering::Equal,
            Ordering::Larger => cmp::Ordering::Greater,
            Ordering::Smaller => cmp::Ordering::Less,
            Ordering::__Unknown(_) => unreachable!(),
        }
    }
}
