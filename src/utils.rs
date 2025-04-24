use crate::prelude::ffi::{self};
use autocxx::prelude::*;

pub type DracoStatusType<T> = Result<T, UniquePtr<ffi::draco::Status>>;

// This is a wrapper around the attribute id returned by the C++ API
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AttrId(pub c_int);

impl AttrId {
    pub fn as_u32(&self) -> u32 {
        self.0 .0 as u32
    }
}

impl From<c_int> for AttrId {
    fn from(value: c_int) -> Self {
        Self(value)
    }
}
impl From<AttrId> for c_int {
    fn from(value: AttrId) -> Self {
        value.0
    }
}

impl From<u32> for ffi::draco::PointIndex {
    fn from(val: u32) -> Self {
        ffi::draco::PointIndex { val }
    }
}
