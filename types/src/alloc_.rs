//! Alloc-dependent features

use crate::{Lei, lei};
use alloc::borrow::ToOwned;

impl ToOwned for lei {
    type Owned = Lei;

    fn to_owned(&self) -> Self::Owned {
        Lei::from_bytes_unchecked(&self.0)
    }
}
