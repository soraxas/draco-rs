mod bindgen;
mod converter;
pub mod pointcloud;

pub mod prelude {
    pub mod ffi {
        pub use crate::bindgen::prelude::ffi::*;
        // pub use crate::bindgen::prelude::ffi::draco_extra;
        // pub use crate::bindgen_extra::prelude::ffi::draco_extra;
    }
    pub use crate::converter::StatusOr;
}
