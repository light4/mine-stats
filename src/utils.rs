use std::{ops::Deref, time::SystemTime};

use bincode::{Decode, Encode};

/// use for cache
#[derive(Debug, Clone, Decode, Encode)]
pub(crate) struct SystemTimeWrapper(SystemTime);

impl Default for SystemTimeWrapper {
    fn default() -> Self {
        Self(SystemTime::now())
    }
}

impl Deref for SystemTimeWrapper {
    type Target = SystemTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
