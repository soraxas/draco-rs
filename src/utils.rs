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

impl From<usize> for ffi::draco::PointIndex {
    fn from(val: usize) -> Self {
        ffi::draco::PointIndex { val: val as u32 }
    }
}

// This is a wrapper around the draco object
// which centralizes the rust-interface of the draco object
pub struct WrappedDracoObject<T>(pub(super) UniquePtr<T>)
where
    T: cxx::memory::UniquePtrTarget;

// This trait is used to get the inner unique pointer of a wrapped draco object
pub trait GetDracoInner {
    type Inner;
    fn get_inner(&self) -> &UniquePtr<Self::Inner>
    where
        Self::Inner: cxx::memory::UniquePtrTarget;
    fn get_inner_mut(&mut self) -> &mut UniquePtr<Self::Inner>
    where
        Self::Inner: cxx::memory::UniquePtrTarget;
}

// This impl is used to get the inner unique pointer of a wrapped draco object
impl<T> GetDracoInner for WrappedDracoObject<T>
where
    T: cxx::memory::UniquePtrTarget,
{
    type Inner = T;

    fn get_inner(&self) -> &UniquePtr<T> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut UniquePtr<T> {
        &mut self.0
    }
}
