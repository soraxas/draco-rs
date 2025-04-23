use autocxx::prelude::*;
use std::pin::Pin;

use crate::prelude::ffi;

pub trait StatusOr<T> {
    fn ok(&self) -> bool {
        matches!(
            self.status().within_box().code(),
            ffi::draco::Status_Code::OK
        )
    }

    fn error_msg(&self) -> String {
        self.status().within_box().error_msg_string().to_string()
    }

    // to be implemented by the mangled cxx template type
    fn status(&self) -> impl New<Output = ffi::draco::Status>;

    // to be implemented by the mangled cxx template type
    fn value(self: Pin<&mut Self>) -> T;
}

impl StatusOr<UniquePtr<ffi::draco::Mesh>>
    for ffi::draco_StatusOr_std_unique_ptr_draco_Mesh_AutocxxConcrete
{
    fn status(&self) -> impl New<Output = ffi::draco::Status> {
        ffi::draco_extra::unpack_status_or_mesh_status(self)
    }

    fn value(self: Pin<&mut Self>) -> UniquePtr<ffi::draco::Mesh> {
        ffi::draco_extra::unpack_status_or_mesh_value(self)
    }
}

impl StatusOr<UniquePtr<ffi::draco::PointCloud>>
    for ffi::draco_StatusOr_std_unique_ptr_draco_PointCloud_AutocxxConcrete
{
    fn status(&self) -> impl New<Output = ffi::draco::Status> {
        ffi::draco_extra::unpack_status_or_pointcloud_status(self)
    }

    fn value(self: Pin<&mut Self>) -> UniquePtr<ffi::draco::PointCloud> {
        ffi::draco_extra::unpack_status_or_pointcloud_value(self)
    }
}
