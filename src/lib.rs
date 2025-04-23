pub mod bindgen;
// pub mod bindgen_extra;

use autocxx::prelude::*;

// use cxx::{let_cxx_string, CxxString};
use std::pin::Pin;

pub mod prelude {
    pub mod ffi {
        pub use crate::bindgen::prelude::ffi::*;
        // pub use crate::bindgen::prelude::ffi::draco_extra;
        // pub use crate::bindgen_extra::prelude::ffi::draco_extra;
    }
    pub use crate::StatusOr;
}

use crate::prelude::ffi;

pub trait StatusOr<T> {
    fn ok(&self) -> bool {
        matches!(self.status().code(), ffi::draco::Status_Code::OK)
    }

    fn error_msg(&self) -> String {
        self.status().error_msg_string().to_string()
    }

    // to be implemented by the mangled cxx template type
    fn status(&self) -> Pin<Box<ffi::draco::Status>>;

    // to be implemented by the mangled cxx template type
    fn value(self: Pin<&mut Self>) -> T;
}

impl StatusOr<UniquePtr<ffi::draco::Mesh>>
    for ffi::draco_StatusOr_std_unique_ptr_draco_Mesh_AutocxxConcrete
{
    fn status(&self) -> Pin<Box<ffi::draco::Status>> {
        ffi::draco_extra::unpack_status_or_mesh_status(self).within_box()
    }

    fn value(self: Pin<&mut Self>) -> UniquePtr<ffi::draco::Mesh> {
        ffi::draco_extra::unpack_status_or_mesh_value(self)
    }
}

// fn value(rr: &ffi::draco_StatusOr_std_unique_ptr_draco_Mesh_AutocxxConcrete) -> UniquePtr<ffi::draco::Mesh> {
//     ffi::draco_extra::unpack_status_or_mesh_value(rr)
// }
