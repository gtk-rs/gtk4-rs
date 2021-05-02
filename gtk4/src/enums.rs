// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Ordering;
use std::cmp;

impl From<cmp::Ordering> for Ordering {
    fn from(o: cmp::Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            cmp::Ordering::Equal => Self::Equal,
            cmp::Ordering::Greater => Self::Larger,
            cmp::Ordering::Less => Self::Smaller,
        }
    }
}

impl From<Ordering> for cmp::Ordering {
    fn from(o: Ordering) -> Self {
        skip_assert_initialized!();
        match o {
            Ordering::Equal => Self::Equal,
            Ordering::Larger => Self::Greater,
            Ordering::Smaller => Self::Less,
            Ordering::__Unknown(_) => unreachable!(),
        }
    }
}
