use crate::prelude::ffi::{self};
use autocxx::prelude::*;

pub type DracoStatusType<T> = Result<T, UniquePtr<ffi::draco::Status>>;

// This is a wrapper around the attribute id returned by the C++ API
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AttrId(pub i32);

impl AttrId {
    pub fn as_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl From<c_int> for AttrId {
    fn from(value: c_int) -> Self {
        Self(value.0)
    }
}
impl From<AttrId> for c_int {
    fn from(value: AttrId) -> Self {
        c_int::from(value.0)
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
//
// This library only exposed some commonly used draco objects
// and methods on them.
//
// If one desires to call methods not exposed by this wrapper,
// one can use the `GetDracoInner` trait to get the inner draco object.
//
// E.g.,
// ```rust
// let pc: PointCloud = PointCloud::new();
// let mut inner: &mut UniquePtr<ffi::draco::PointCloud> = pc.get_inner_mut();
// inner.pin_mut().GetAttributeByUniqueId(...);
// ```
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
